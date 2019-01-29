use byteorder::{ReadBytesExt, LE};
use serde::{Deserialize, Serialize};

use rblitz_packets::{
    packets::game::{answer::SQueryStatusAns, common::*, request::CQueryStatusReq, *},
    PacketId,
};

use crate::client::{ClientId, ClientStatus, Team};
use crate::error::Result;
use crate::packet::{packet_handler::WorldData, Channel};

pub type GameHandler = fn(&mut WorldData, ClientId, u32, &[u8]) -> Result<()>;

pub trait GamePacket: PacketId + Serialize + std::fmt::Debug {}

pub trait GamePacketHandler: GamePacket + for<'de> Deserialize<'de> {
    fn handle(world: &mut WorldData, cid: ClientId, sender_net_id: u32, data: &[u8]) -> Result<()> {
        let this = rblitz_packets::from_bytes::<Self>(data)?;
        log::trace!("[RECEIVED] {:?}", this);
        this.handle_self(world, cid, sender_net_id)
    }
    fn handle_self(self, world: &mut WorldData, cid: ClientId, sender_net_id: u32) -> Result<()>;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct RawGamePacket<'a> {
    pub id: u8,
    pub sender_net_id: u32,
    pub data: &'a [u8],
}

impl<'a> RawGamePacket<'a> {
    pub fn from_slice(mut data: &'a [u8]) -> Result<RawGamePacket<'a>> {
        Ok(RawGamePacket {
            id: data.read_u8()?,
            sender_net_id: data.read_u32::<LE>()?,
            data,
        })
    }
}

// implements it on loading screen packets as well /shrug
impl<T> GamePacket for T where T: PacketId + Serialize + std::fmt::Debug {}

impl GamePacketHandler for CQueryStatusReq {
    fn handle_self(self, world: &mut WorldData, cid: ClientId, _: u32) -> Result<()> {
        world
            .clients
            .get_mut(&cid)
            .unwrap()
            .send_game_packet(Channel::Broadcast, &SQueryStatusAns { is_ok: true });
        Ok(())
    }
}

impl GamePacketHandler for CReconnect {
    fn handle_self(self, world: &mut WorldData, cid: ClientId, _: u32) -> Result<()> {
        world
            .clients
            .get_mut(&cid)
            .unwrap()
            .send_game_packet(Channel::ClientToServer, &SReconnect { client_id: cid.0 });
        Ok(())
    }
}

impl GamePacketHandler for CSendSelectedObjID {
    fn handle_self(self, _world: &mut WorldData, _cid: ClientId, _: u32) -> Result<()> {
        Ok(())
    }
}

impl GamePacketHandler for CSyncVersion {
    fn handle_self(self, world: &mut WorldData, cid: ClientId, _: u32) -> Result<()> {
        let mut player_info: [PlayerLoadInfo; 12] = Default::default();
        for (load_info, client) in player_info.iter_mut().zip(world.clients.values()) {
            *load_info = client.player_load_info();
        }

        world.clients.get_mut(&cid).unwrap().send_game_packet(
            Channel::Broadcast,
            &SSyncVersion {
                is_version_ok: true,
                map: 8, //todo replace with world.read_resource::<Map>().id
                player_info,
                version_string: self.version,
                map_mode: "ODIN".to_owned(),
            },
        );
        Ok(())
    }
}

impl GamePacketHandler for CClientReady {
    fn handle_self(self, world: &mut WorldData, cid: ClientId, _: u32) -> Result<()> {
        let clients = &mut world.clients;
        clients.get_mut(&cid).unwrap().status = ClientStatus::Ready;
        if clients.values().all(|c| c.status == ClientStatus::Ready) {
            log::info!("All clients ready, starting game");
            clients.broadcast(
                Channel::Broadcast,
                &SStartGame {
                    tournament_pause_enabled: false,
                },
            );
            clients
                .values_mut()
                .for_each(|c| c.status = ClientStatus::Connected);
        }
        Ok(())
    }
}

impl GamePacketHandler for CCharSelected {
    fn handle_self(self, world: &mut WorldData, cid: ClientId, _: u32) -> Result<()> {
        let client = world.clients.get_mut(&cid).unwrap();
        let create_hero = SCreateHero {
            unit_net_id: 0x40000001,
            client_id: cid.0,
            net_node_id: 0x40,
            skill_level: 1,
            team_is_order: client.team == Team::Order,
            is_bot: false,
            bot_rank: 0,
            spawn_position_index: 0,
            skin_id: client.skin_id as u32,
            name: client.name.clone(),
            skin: client.champion.clone(),
        };

        client.send_game_packet(
            Channel::Broadcast,
            &SStartSpawn {
                bot_count_order: 0,
                bot_count_chaos: 0,
            },
        );
        client.send_game_packet(Channel::Broadcast, &create_hero);
        client.send_game_packet(Channel::Broadcast, &SEndSpawn);
        Ok(())
    }
}

// Turns out riot is just horrible and use some weird interpolation for the loading percentage
// client side which results in completely inaccurate loading progression
impl GamePacketHandler for CPingLoadInfo {
    fn handle_self(mut self, world: &mut WorldData, cid: ClientId, _: u32) -> Result<()> {
        let client = world.clients.get(&cid).unwrap();
        self.connection_info.player_id = client.player_id;
        world.clients.broadcast(
            Channel::Broadcast,
            &SPingLoadInfo {
                connection_info: self.connection_info,
            },
        );
        Ok(())
    }
}

impl GamePacketHandler for CExit {
    fn handle_self(self, world: &mut WorldData, cid: ClientId, _: u32) -> Result<()> {
        world.clients.get_mut(&cid).unwrap().disconnect();
        Ok(())
    }
}

impl GamePacketHandler for CWorldSendCameraServer {
    fn handle_self(self, _world: &mut WorldData, _cid: ClientId, _: u32) -> Result<()> {
        Ok(())
    }
}

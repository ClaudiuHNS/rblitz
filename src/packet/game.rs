//! currently this is just a collection of almost all packet handlers, ideally the packet handler
//! should be defined where they make the most sense though

use byteorder::{ReadBytesExt, LE};
use serde::{Deserialize, Serialize};

use rblitz_packets::{
    packets::game::{answer::SQueryStatusAns, common::*, request::CQueryStatusReq, *},
    PacketId,
};

use crate::{
    client::{ClientId, ClientStatus},
    error::Result,
    packet::{packet_handler::WorldData, Channel},
    world::components::Team,
};
use rblitz_packets::Vector2;
use rblitz_packets::Vector3;

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

// implements it on loading screen packets as well, shouldnt be too problematic
impl<T> GamePacket for T where T: PacketId + Serialize + std::fmt::Debug {}

impl GamePacketHandler for CQueryStatusReq {
    fn handle_self(self, world: &mut WorldData, cid: ClientId, _: u32) -> Result<()> {
        world.clients.get_mut(&cid).unwrap().send_game_packet(
            0,
            Channel::Broadcast,
            &SQueryStatusAns { is_ok: true },
        );
        Ok(())
    }
}

impl GamePacketHandler for CReconnect {
    fn handle_self(self, world: &mut WorldData, cid: ClientId, _: u32) -> Result<()> {
        world.clients.get_mut(&cid).unwrap().send_game_packet(
            0,
            Channel::ClientToServer,
            &SReconnect { client_id: cid.0 },
        );
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
            let sums = world.summoner_spells.get(client.champion).unwrap();
            *load_info = PlayerLoadInfo {
                player_id: client.player_id,
                summoner_level: client.summoner_level,
                summoner_spell1: sums.0,
                summoner_spell2: sums.1,
                is_bot: false,
                team_id: *world.teams.get(client.champion).unwrap() as u32,
                _pad0: Default::default(),
                _pad1: Default::default(),
                bot_difficulty: 0,
                profile_icon_id: client.profile_icon,
            };
        }

        world.clients.get_mut(&cid).unwrap().send_game_packet(
            0,
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
                0,
                Channel::Broadcast,
                &SStartGame {
                    tournament_pause_enabled: false,
                },
            );
            for c in clients.values_mut() {
                c.status = ClientStatus::Connected;
                let net_id = world.net_ids.get(c.champion).unwrap();
                c.send_game_packet(
                    net_id.id(),
                    Channel::Broadcast,
                    &SOnEnterVisibilityClient {
                        entries: Vec::new(),
                        look_at_pos: None,
                        movement_data: MovementData::Stop(MovementDataStop {
                            position: Vector2 { x: 26.0, y: 280.0 },
                            forward: Vector2 { x: 26.0, y: 280.0 },
                        }),
                    },
                )
            }
        }
        Ok(())
    }
}

impl GamePacketHandler for CCharSelected {
    fn handle_self(self, world: &mut WorldData, cid: ClientId, _: u32) -> Result<()> {
        let client = world.clients.get_mut(&cid).unwrap();
        let net_id = world.net_ids.get(client.champion).unwrap();
        let sums = world.summoner_spells.get(client.champion).unwrap();
        let create_hero = SCreateHero {
            unit_net_id: net_id.id(),
            client_id: cid.0,
            net_node_id: net_id.node_id() as u8,
            skill_level: 0,
            team_is_order: *world.teams.get(client.champion).unwrap() == Team::Order,
            is_bot: false,
            bot_rank: 0,
            spawn_position_index: 1,
            skin_id: client.champ_skin_id,
            name: client.name.clone(),
            skin: world.unit_names.get(client.champion).unwrap().0.clone(),
        };

        client.send_game_packet(
            0,
            Channel::Broadcast,
            &SStartSpawn {
                bot_count_order: 0,
                bot_count_chaos: 0,
            },
        );
        client.send_game_packet(0, Channel::Broadcast, &create_hero);
        client.send_game_packet(
            net_id.id(),
            Channel::Broadcast,
            &SAvatarInfo {
                summoner_spell_ids: [sums.0, sums.1],
                level: 1,
                ..Default::default()
            },
        );
        client.send_game_packet(0, Channel::Broadcast, &SEndSpawn);
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
            0,
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
    #[inline]
    fn handle_self(self, _world: &mut WorldData, _cid: ClientId, _: u32) -> Result<()> {
        Ok(())
    }
}

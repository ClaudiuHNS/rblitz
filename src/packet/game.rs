use byteorder::{ReadBytesExt, LE};
use serde::{Deserialize, Serialize};

use rblitz_packets::{
    packets::game::{answer::SQueryStatusAns, common::*, request::CQueryStatusReq, *},
    PacketId,
};

use crate::packet::packet_handler::WorldData;
use crate::{client::ClientId, error::Result, packet::Channel};

pub type GameHandler = fn(&mut WorldData, ClientId, u32, &[u8]) -> Result<()>;

pub trait GamePacket: PacketId + Serialize
where
    Self: for<'de> Deserialize<'de>,
{
}

pub trait GamePacketHandler: GamePacket {
    fn handle(world: &mut WorldData, cid: ClientId, sender_net_id: u32, data: &[u8]) -> Result<()> {
        rblitz_packets::from_bytes::<Self>(data)?.handle_self(world, cid, sender_net_id)
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
impl<T> GamePacket for T where T: PacketId + Serialize + for<'de> Deserialize<'de> {}

impl GamePacketHandler for CQueryStatusReq {
    fn handle_self(self, world: &mut WorldData, cid: ClientId, _sender_net_id: u32) -> Result<()> {
        world.clients.get_mut(&cid).unwrap().send_game_packet(
            Channel::Broadcast,
            cid.0,
            SQueryStatusAns { is_ok: true },
        );
        Ok(())
    }
}

impl GamePacketHandler for CReconnect {
    fn handle_self(self, world: &mut WorldData, cid: ClientId, _sender_net_id: u32) -> Result<()> {
        world.clients.get_mut(&cid).unwrap().send_game_packet(
            Channel::ClientToServer,
            cid.0,
            SReconnect { client_id: cid.0 },
        );
        Ok(())
    }
}

impl GamePacketHandler for CSyncVersion {
    fn handle_self(self, world: &mut WorldData, cid: ClientId, _sender_net_id: u32) -> Result<()> {
        let mut player_info: [PlayerLoadInfo; 12] = Default::default();
        player_info[0] = PlayerLoadInfo {
            player_id: u64::from(cid.0),
            summoner_level: 30,
            summoner_spell1: 0x06496EA8,
            summoner_spell2: 0x06496EA8,
            ..Default::default()
        };
        world.clients.get_mut(&cid).unwrap().send_game_packet(
            Channel::Broadcast,
            cid.0,
            SSyncVersion {
                is_version_ok: true,
                map: 8,
                player_info,
                version_string: self.version,
                map_mode: "Automatic".to_owned(),
            },
        );
        Ok(())
    }
}

impl GamePacketHandler for CClientReady {
    fn handle_self(self, world: &mut WorldData, cid: ClientId, _sender_net_id: u32) -> Result<()> {
        world.clients.get_mut(&cid).unwrap().send_game_packet(
            Channel::Broadcast,
            cid.0,
            SStartGame {
                tournament_pause_enabled: false,
            },
        );
        Ok(())
    }
}

impl GamePacketHandler for CCharSelected {
    fn handle_self(self, world: &mut WorldData, cid: ClientId, _sender_net_id: u32) -> Result<()> {
        let client = world.clients.get_mut(&cid).unwrap();
        let create_hero = SCreateHero {
            unit_net_id: 0x40000001,
            client_id: cid.0,
            net_node_id: 0x40,
            skill_level: 1,
            team_is_order: true,
            is_bot: false,
            bot_rank: 0,
            spawn_position_index: 0,
            skin_id: 0,
            name: "RBlitzTest".to_owned(),
            skin: "Nasus".to_owned(),
        };

        client.send_game_packet(
            Channel::Broadcast,
            cid.0,
            SStartSpawn {
                bot_count_order: 0,
                bot_count_chaos: 0,
            },
        );
        client.send_game_packet(Channel::Broadcast, cid.0, create_hero);
        client.send_game_packet(Channel::Broadcast, cid.0, SEndSpawn);
        Ok(())
    }
}

impl GamePacketHandler for CPingLoadInfo {
    fn handle_self(self, world: &mut WorldData, cid: ClientId, _sender_net_id: u32) -> Result<()> {
        world.clients.get_mut(&cid).unwrap().send_game_packet(
            Channel::Broadcast,
            cid.0,
            SPingLoadInfo {
                connection_info: self.connection_info,
            },
        );
        Ok(())
    }
}

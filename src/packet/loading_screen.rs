use byteorder::ReadBytesExt;
use rblitz_packets::packets::{loading_screen::*, PacketId};
use serde::{Deserialize, Serialize};

use crate::client::ClientId;
use crate::error::Result;
use crate::packet::packet_handler::WorldData;

#[derive(Copy, Clone, Debug, Default)]
pub struct RawLoadingScreenPacket<'a> {
    pub id: u8,
    pub data: &'a [u8],
}

impl<'a> RawLoadingScreenPacket<'a> {
    pub fn from_slice(mut data: &'a [u8]) -> Result<RawLoadingScreenPacket<'a>> {
        Ok(RawLoadingScreenPacket {
            id: data.read_u8()?,
            data,
        })
    }
}

pub type LoadingScreenHandler = fn(&mut WorldData, ClientId, &[u8]) -> Result<()>;

pub trait LoadingScreenPacket: PacketId + Serialize
where
    Self: for<'de> Deserialize<'de>,
{
    fn handle(world: &mut WorldData, cid: ClientId, data: &[u8]) -> Result<()> {
        rblitz_packets::from_bytes::<Self>(data)?.handle_self(world, cid)
    }
    fn handle_self(self, _world: &mut WorldData, _cid: ClientId) -> Result<()> {
        log::trace!("Unhandled Loading Screen Packet 0x{:X}", Self::ID);
        Ok(())
    }
}

impl LoadingScreenPacket for RequestJoinTeam {
    fn handle_self(self, world: &mut WorldData, cid: ClientId) -> Result<()> {
        let mut roster_update = TeamRosterUpdate::default();
        roster_update.current_team_size_order = 1;
        roster_update.current_team_size_chaos = 0;
        roster_update.team_size_order = 1;
        roster_update.team_size_chaos = 0;
        roster_update.order_player_ids[0] = u64::from(cid.0);

        let mut reskin = RequestReskin::default();
        reskin.player_id = u64::from(cid.0);
        reskin.skin_id = 0;
        reskin.name = "Nasus".to_owned();

        let mut rename = RequestRename::default();
        rename.player_id = u64::from(cid.0);
        rename.skin_id = 0;
        rename.name = "RBlitzTest".to_owned();

        let client = world.clients.get_mut(&cid).unwrap();
        client.send_loading_screen_packet(roster_update);
        client.send_loading_screen_packet(reskin);
        client.send_loading_screen_packet(rename);
        Ok(())
    }
}

impl LoadingScreenPacket for RequestReskin {}

impl LoadingScreenPacket for RequestRename {}

impl LoadingScreenPacket for TeamRosterUpdate {}

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

pub trait LoadingScreenPacket: PacketId + Serialize + std::fmt::Debug
where
    Self: for<'de> Deserialize<'de>,
{
    fn handle(world: &mut WorldData, cid: ClientId, data: &[u8]) -> Result<()> {
        let this = rblitz_packets::from_bytes::<Self>(data)?;
        log::trace!("[RECEIVED] {:?}", this);
        this.handle_self(world, cid)
    }
    fn handle_self(self, _world: &mut WorldData, _cid: ClientId) -> Result<()> {
        Ok(())
    }
}

impl LoadingScreenPacket for RequestJoinTeam {
    fn handle_self(self, world: &mut WorldData, cid: ClientId) -> Result<()> {
        world.clients.send_roster_update(cid);
        Ok(())
    }
}

impl LoadingScreenPacket for RequestReskin {}

impl LoadingScreenPacket for RequestRename {}

impl LoadingScreenPacket for TeamRosterUpdate {}

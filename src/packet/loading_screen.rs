use rblitz_packets::packets::{loading_screen::*, PacketId};
use serde::Serialize;

pub trait LoadingScreenPacket: PacketId + Serialize + Sized + std::fmt::Debug {
    fn to_bytes(&self) -> Box<[u8]> {
        let mut data = Vec::with_capacity(core::mem::size_of::<Self>() + 1);
        data.push(Self::ID);
        rblitz_packets::to_writer(&self, &mut data).unwrap();
        data.into_boxed_slice()
    }
}

impl LoadingScreenPacket for RequestReskin {}
impl LoadingScreenPacket for RequestRename {}
impl LoadingScreenPacket for TeamRosterUpdate {}

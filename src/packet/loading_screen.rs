use rblitz_packets::packets::{loading_screen::*, PacketId};
use serde::Serialize;

pub trait LoadingScreenPacket: PacketId + Serialize + std::fmt::Debug {}
impl LoadingScreenPacket for RequestReskin {}
impl LoadingScreenPacket for RequestRename {}
impl LoadingScreenPacket for TeamRosterUpdate {}

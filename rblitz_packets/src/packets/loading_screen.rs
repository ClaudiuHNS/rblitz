use serde::{Deserialize, Serialize};

use super::packet_id;

#[packet_id(0x64)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct RequestJoinTeam {
    pub _pad: [u8; 3],
    pub client_id: u32,
    pub team_id: u32,
}

#[packet_id(0x65)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct RequestReskin {
    pub _pad: [u8; 7],
    pub player_id: u64,
    pub skin_id: u32,
    #[serde(with = "crate::sized_string_null")]
    pub name: String,
}

#[packet_id(0x66)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct RequestRename {
    pub _pad: [u8; 7],
    pub player_id: u64,
    pub skin_id: u32,
    #[serde(with = "crate::sized_string_null")]
    pub name: String,
}

#[packet_id(0x67)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct TeamRosterUpdate {
    pub _pad0: [u8; 3],
    pub team_size_order: u32,
    pub team_size_chaos: u32,
    pub _pad1: [u8; 4],
    pub order_player_ids: [u64; 24],
    pub chaos_player_ids: [u64; 24],
    pub current_team_size_order: u32,
    pub current_team_size_chaos: u32,
}

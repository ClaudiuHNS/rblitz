use serde::{Deserialize, Serialize};

use super::common::ConnectionInfo;
use super::packet_id;
use crate::Vector3;

#[packet_id(0x05)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CTutorialAudioEventFinished {
    pub audio_event_net_id: u32,
}

#[packet_id(0x08)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CSyncSimTime {
    pub time_last_server: f32,
    pub time_last_client: f32,
}

#[packet_id(0x19)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CPingLoadInfo {
    pub connection_info: ConnectionInfo,
}

#[packet_id(0x20)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CWriteNavFlagsAcc {
    pub sync_id: i32,
}

#[packet_id(0x30)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CWorldSendCameraServer {
    pub camera_position: Vector3,
    pub camera_direction: Vector3,
    pub client_id: u32,
    pub sync_id: u8,
}

#[packet_id(0x3F)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CUseObject {
    pub target_net_id: u32,
}

#[packet_id(0x4E)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CScoreBoardOpened;

#[packet_id(0x4C)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CPlayEmote {
    pub emote_id: u32,
}

#[packet_id(0x55)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CClientReady;

#[packet_id(0x5A)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CMapPing {
    pub position: Vector3,
    pub target_net_id: u32,
    pub ping_category: u8,
}

#[packet_id(0x60)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CShopOpened;

#[packet_id(0x70)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CTipEvent {
    pub tip_command: u8,
    pub tip_id: u32,
}

#[packet_id(0x91)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CClientFinished;

#[packet_id(0x94)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CExit;

#[packet_id(0x9A)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CClientConnectNamedPipe;

#[packet_id(0xA4)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CTeamSurrenderVote {
    pub voted_yes: bool,
}

#[packet_id(0xAC)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CReconnect {
    pub is_full_reconnect: bool,
}

#[packet_id(0xB7)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CSendSelectedObjID {
    pub client_id: u32,
    pub selected_net_id: u32,
}

#[packet_id(0xC5)]
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct CSyncVersion {
    pub time_last_client: f32,
    pub client_id: u32,
    #[serde(with = "crate::string_128")]
    pub version: String,
}

#[packet_id(0xC6)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CCharSelected;

#[packet_id(0xD5)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CTutorialPopupClosed;

#[packet_id(0xD6)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CQuestEvent {
    pub quest_event: u8,
    pub quest_id: u32,
}

#[packet_id(0xDF)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct CRespawnPointEvent {
    pub respawn_point_event: u8,
    pub respawn_point_ui_element_id: u8,
}

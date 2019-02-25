pub mod game;
pub mod loading_screen;

pub(in crate) use rblitz_packets_proc_macro::packet_id;

pub trait PacketId {
    const ID: u8;
}

pub mod chat {
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Clone, Debug, Default)]
    pub struct ChatPacket {
        pub client_id: u32,
        pub typ: u32,
        #[serde(with = "crate::sized_string_null")]
        pub message: String,
    }
}

pub mod chat;
pub mod game;
pub mod loading_screen;
pub mod packet_handler;

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
pub struct KeyCheck {
    pub action: u8,
    pad: [u8; 3],
    pub client_id: u32,
    pub player_id: u64,
    pub check_id: [u8; 8],
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Channel {
    Handshake = 0x0,
    ClientToServer = 0x1,
    SyncClock = 0x2,
    Broadcast = 0x3,
    BroadcastUnreliable = 0x4,
    Chat = 0x5,
    LoadingScreen = 0x6,
}

impl Channel {
    #[inline]
    fn try_from(u8: u8) -> Option<Self> {
        match u8 {
            0 => Some(Channel::Handshake),
            1 => Some(Channel::ClientToServer),
            2 => Some(Channel::SyncClock),
            3 => Some(Channel::Broadcast),
            4 => Some(Channel::BroadcastUnreliable),
            5 => Some(Channel::Chat),
            6 => Some(Channel::LoadingScreen),
            _ => None,
        }
    }
}

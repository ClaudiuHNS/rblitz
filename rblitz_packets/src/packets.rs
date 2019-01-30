pub mod game;
pub mod loading_screen;

pub(in crate) use rblitz_packets_proc_macro::packet_id;

pub trait PacketId {
    const ID: u8;
}

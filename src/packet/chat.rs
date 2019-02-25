use rblitz_packets::packets::chat::ChatPacket;
use shred::SystemData;
use specs::World;

use crate::{
    client::{ClientId, ClientMap},
    error::Result,
    packet::{dispatcher_sys::PacketSender, Channel},
};

pub(super) fn handle_chat_message(world: &World, cid: ClientId, data: &mut [u8]) -> Result<()> {
    let packet = rblitz_packets::from_bytes::<ChatPacket>(data)?;
    let sender = PacketSender::fetch(&world.res);
    match packet.typ {
        0 => sender.broadcast_all(Channel::Chat, data.to_owned().into_boxed_slice()),
        1 => {
            let clients = world.read_resource::<ClientMap>();
            let team = clients[&cid].team;
            sender.broadcast_group(
                Channel::Chat,
                clients
                    .iter()
                    .filter_map(|(cid, c)| if c.team == team { Some(*cid) } else { None })
                    .collect(),
                data.to_owned().into_boxed_slice(),
            )
        },
        v => log::warn!("[CHAT] invalid chat type received {:?}", v),
    }
    Ok(())
}

use crate::{
    client::{ClientId, ClientMap},
    packet::{game::GamePacket, Channel},
};
use crossbeam_channel::{Receiver, Sender};
use shred::{System, SystemData, WriteExpect};

/// A PacketSender just wraps a [`shred::Fetch`] of a ['crossbeam_channel::Sender'], it is used as
/// a SystemData for sending out packets at the end of a game cycle and is the only way to send out
/// packets to clients.
pub struct PacketSender<'a>(shred::Fetch<'a, Sender<Command>>);

impl<'a> PacketSender<'a> {
    /// Sends data to a specified [`ClientId`]. Shouldnt be needed outside from the client module
    pub fn single(&self, cid: ClientId, channel: Channel, data: Box<[u8]>) {
        if let Err(e) = self.0.send(Command::Single(cid, channel, data)) {
            log::warn!("{}", e);
        }
    }

    /// Sends a single packet to a specified [`ClientId`]
    pub fn single_packet<P>(&self, cid: ClientId, channel: Channel, sender_net_id: u32, packet: &P)
    where
        P: GamePacket,
    {
        log::trace!("[SENT][{}] {:?}", cid.0, packet);
        self.single(cid, channel, packet.to_bytes(sender_net_id));
    }

    /// Sends a single packet to all clients
    pub fn broadcast_all<P>(&self, channel: Channel, sender_net_id: u32, packet: &P)
    where
        P: GamePacket,
    {
        log::trace!("[BROADCAST] {:?}", packet);
        if let Err(e) = self.0.send(Command::BroadcastAll(
            channel,
            packet.to_bytes(sender_net_id),
        )) {
            log::warn!("{}", e);
        }
    }

    /// Sends a single packet to a range of clients
    pub fn broadcast_group<P>(
        &self,
        cids: Box<[ClientId]>,
        channel: Channel,
        sender_net_id: u32,
        packet: &P,
    ) where
        P: GamePacket,
    {
        log::trace!("[BROADCAST] {:?}", packet);
        if let Err(e) = self.0.send(Command::BroadcastGroup(
            cids,
            channel,
            packet.to_bytes(sender_net_id),
        )) {
            log::warn!("{}", e);
        }
    }
}

impl<'a> SystemData<'a> for PacketSender<'a> {
    fn setup(_: &mut shred::Resources) {}
    fn fetch(res: &'a shred::Resources) -> Self {
        PacketSender(res.fetch::<Sender<Command>>())
    }
    fn reads() -> Vec<shred::ResourceId> {
        vec![shred::ResourceId::new::<Sender<Command>>()]
    }
    fn writes() -> Vec<shred::ResourceId> {
        Vec::new()
    }
}

/// A "Command" for the PacketDispatcher, telling it what to do with the data it receives on the
/// crossbeam channel
pub enum Command {
    Single(ClientId, Channel, Box<[u8]>),
    BroadcastGroup(Box<[ClientId]>, Channel, Box<[u8]>),
    BroadcastAll(Channel, Box<[u8]>),
}

/// The PacketDispatcher is responsible for sending out the packets to the respective clients at
/// the end of a cycle
pub struct PacketDispatcher(Receiver<Command>);

impl PacketDispatcher {
    pub fn new(recv: Receiver<Command>) -> Self {
        PacketDispatcher(recv)
    }
}

impl<'a> System<'a> for PacketDispatcher {
    type SystemData = WriteExpect<'a, ClientMap>;

    fn run(&mut self, mut client_map: Self::SystemData) {
        for cmd in self.0.try_iter() {
            match cmd {
                Command::Single(cid, channel, mut packet) => {
                    if let Some(client) = client_map.get_mut(&cid) {
                        client.send_data(channel, &mut packet);
                    }
                },
                Command::BroadcastGroup(cids, channel, packet) => {
                    for cid in cids.iter() {
                        if let Some(client) = client_map.get_mut(cid) {
                            client.send_data(channel, &mut packet.clone());
                        }
                    }
                },
                Command::BroadcastAll(channel, packet) => {
                    for client in client_map.values_mut() {
                        client.send_data(channel, &mut packet.clone());
                    }
                },
            }
        }
    }
}

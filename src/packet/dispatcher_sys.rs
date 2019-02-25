use crate::{
    client::{ClientConnectionMap, ClientId},
    packet::{game::GamePacket, Channel},
};
use crossbeam_channel::{Receiver, Sender};
use shred::{System, SystemData, WriteExpect};

/// A PacketSender just wraps a [`shred::Fetch`] of a ['crossbeam_channel::Sender'], it is used as
/// a SystemData for sending out packets at the end of a game cycle and is the only way to send out
/// packets to clients.
pub struct PacketSender<'a>(shred::Fetch<'a, Sender<Command>>);

impl<'a> PacketSender<'a> {
    /// Sends data to a specified [`ClientId`].
    pub fn single(&self, channel: Channel, cid: ClientId, data: Box<[u8]>) {
        if let Err(e) = self.0.send(Command::Single(channel, cid, data)) {
            log::warn!("{}", e);
        }
    }

    /// Sends data to all clients.
    pub fn broadcast_all(&self, channel: Channel, data: Box<[u8]>) {
        if let Err(e) = self.0.send(Command::BroadcastAll(channel, data)) {
            log::warn!("{}", e);
        }
    }

    /// Sends data to a range of clients.
    pub fn broadcast_group(&self, channel: Channel, cids: Box<[ClientId]>, data: Box<[u8]>) {
        if let Err(e) = self.0.send(Command::BroadcastGroup(channel, cids, data)) {
            log::warn!("{}", e);
        }
    }

    /// Sends a single game packet to a specified [`ClientId`]
    pub fn gp_single<P>(&self, channel: Channel, cid: ClientId, sender_net_id: u32, packet: &P)
    where
        P: GamePacket,
    {
        log::trace!("[SENT][{}] {:?}", cid.0, packet);
        self.single(channel, cid, packet.to_bytes(sender_net_id));
    }

    /// Sends a single game packet to all clients
    pub fn gp_broadcast_all<P>(&self, channel: Channel, sender_net_id: u32, packet: &P)
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

    /// Sends a single game packet to a range of clients
    pub fn gp_broadcast_group<P>(
        &self,
        channel: Channel,
        cids: Box<[ClientId]>,
        sender_net_id: u32,
        packet: &P,
    ) where
        P: GamePacket,
    {
        log::trace!("[BROADCAST] {:?}", packet);
        if let Err(e) = self.0.send(Command::BroadcastGroup(
            channel,
            cids,
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
    Single(Channel, ClientId, Box<[u8]>),
    BroadcastGroup(Channel, Box<[ClientId]>, Box<[u8]>),
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
    type SystemData = WriteExpect<'a, ClientConnectionMap>;

    fn run(&mut self, mut connection_map: Self::SystemData) {
        for cmd in self.0.try_iter() {
            match cmd {
                Command::Single(channel, cid, mut packet) => {
                    connection_map.send_data(cid, channel, &mut packet);
                },
                Command::BroadcastGroup(channel, cids, packet) => {
                    for cid in cids.iter() {
                        connection_map.send_data(*cid, channel, &mut packet.clone());
                    }
                },
                Command::BroadcastAll(channel, packet) => {
                    let cids = connection_map.keys().cloned().collect::<Vec<_>>();
                    for cid in cids {
                        connection_map.send_data(cid, channel, &mut packet.clone());
                    }
                },
            }
        }
    }
}

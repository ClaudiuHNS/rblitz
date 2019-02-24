use crate::{
    client::{ClientId, ClientMap},
    packet::{game::GamePacket, Channel},
};
use crossbeam_channel::{Receiver, Sender};
use shred::{System, SystemData, WriteExpect};

pub struct PacketSender<'a>(shred::Fetch<'a, Sender<Command>>);

impl<'a> PacketSender<'a> {
    pub fn single(&self, cid: ClientId, channel: Channel, data: Box<[u8]>) {
        if let Err(e) = self.0.send(Command::Single(cid, channel, data)) {
            log::warn!("{}", e);
        }
    }

    pub fn single_packet<P>(&self, cid: ClientId, channel: Channel, sender_net_id: u32, packet: &P)
    where
        P: GamePacket,
    {
        log::trace!("[SENT][{}] {:?}", cid.0, packet);
        self.single(cid, channel, packet.to_bytes(sender_net_id));
    }

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

pub enum Command {
    Single(ClientId, Channel, Box<[u8]>),
    BroadcastGroup(Box<[ClientId]>, Channel, Box<[u8]>),
    BroadcastAll(Channel, Box<[u8]>),
}

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

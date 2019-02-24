use indexmap::IndexMap;
use shred::SystemData;
use specs::World;

use crate::{
    client::{ClientId, ClientMap, ClientStatus},
    lenet_server::{Event, LENetServer},
    packet::{
        dispatcher_sys::PacketSender,
        game::{PacketHandler, PacketHandlerDummy, PacketHandlerImpl, RawGamePacket},
        Channel,
    },
    world::components::{Team, UnitName},
};

/// The PacketHandlerSys works similar to a [`shred::System`] with the exception that it is being
/// used manually and not with the shred trait. The reason for that is that it should run before any
/// other system and that it uses uses [`PacketHandler`] as "subsystems" of its own which would not
/// be possible otherwise.
pub struct PacketHandlerSys<'r> {
    game_handlers: IndexMap<u8, Box<dyn for<'a> PacketHandler<'a> + 'r>>,
}

impl<'r> PacketHandlerSys<'r> {
    pub fn new() -> Self {
        let mut this = PacketHandlerSys {
            game_handlers: IndexMap::new(),
        };
        this.register_game_packet_handlers();
        this
    }

    fn handle_packet(&self, world: &World, channel: u8, cid: ClientId, data: &mut [u8]) {
        let channel = Channel::try_from(channel).expect("unknown channel received");
        world
            .read_resource::<ClientMap>()
            .get(&cid)
            .unwrap()
            .decrypt(data);
        match channel {
            //handled outside of this
            Channel::Handshake => (),
            Channel::ClientToServer
            | Channel::SyncClock
            | Channel::Broadcast
            | Channel::BroadcastUnreliable => {
                let packet = RawGamePacket::from_slice(data).unwrap();
                if let Some(handler) = self.game_handlers.get(&packet.id) {
                    handler
                        .handle(&world.res, cid, packet.sender_net_id, packet.data)
                        .unwrap();
                } else {
                    log::debug!(
                        "Unhandled Packet 0x{:X} received on channel {:?}",
                        packet.id,
                        channel,
                    );
                }
            },
            Channel::Chat => (),
            Channel::LoadingScreen => {
                use rblitz_packets::packets::{loading_screen::RequestJoinTeam, PacketId};
                if !data.is_empty() && RequestJoinTeam::ID == data[0] {
                    world.write_resource::<ClientMap>().send_roster_update(
                        PacketSender::fetch(&world.res),
                        &world.read_storage::<UnitName>(),
                        &world.read_storage::<Team>(),
                        cid,
                    );
                }
            },
        }
    }

    fn register_game_handler<P>(&mut self)
    where
        P: for<'a> PacketHandlerImpl<'a> + 'r,
    {
        assert!(
            self.game_handlers
                .insert(
                    P::ID,
                    Box::new(PacketHandlerDummy::<P>(core::marker::PhantomData))
                )
                .is_none(),
            "Game handler replaced for 0x{id:X}, check that it isn't being registered twice and\
             that the ID(0x{id:X}) is correct",
            id = P::ID
        );
    }

    pub fn run(&self, server: &mut LENetServer, world: &World) {
        loop {
            match server.service(0) {
                Ok(event) => match event {
                    Event::Connected(keycheck, peer) => {
                        let mut clients = world.write_resource::<ClientMap>();
                        let cid = clients
                            .iter_mut()
                            .find(|(_, c)| c.player_id == keycheck.player_id)
                            .and_then(|(cid, client)| {
                                if client.auth(*cid, keycheck, peer).is_ok() {
                                    client.status = ClientStatus::Loading;
                                    Some(*cid)
                                } else {
                                    None
                                }
                            });
                        match cid {
                            Some(cid) => clients.broadcast_keycheck(cid),
                            None => unsafe { enet_sys::enet_peer_disconnect_now(peer, 0) },
                        }
                    },
                    Event::Disconnected(cid) => {
                        log::info!("Disconnected: {:?}", cid);
                        let mut clients = world.write_resource::<ClientMap>();
                        let client = clients.get_mut(&cid).unwrap();
                        client.status = ClientStatus::Disconnected;
                        client.peer = None;
                        if clients
                            .values()
                            .all(|c| c.status == ClientStatus::Disconnected)
                        {
                            // FIXME shutdown server gracefully
                            panic!("All players lost connection, terminating server");
                        }
                    },
                    Event::Packet(cid, channel, mut packet) => {
                        self.handle_packet(world, channel, cid, &mut *packet)
                    },
                    Event::NoEvent => break,
                },
                Err(e) => log::error!("{:?}", e),
            }
        }
    }

    fn register_game_packet_handlers(&mut self) {
        self.game_handlers.reserve(32);

        use rblitz_packets::packets::game::{request::*, *};
        self.register_game_handler::<CQueryStatusReq>();
        self.register_game_handler::<CSyncVersion>();
        self.register_game_handler::<CCharSelected>();
        self.register_game_handler::<CPingLoadInfo>();
        self.register_game_handler::<CClientReady>();
        self.register_game_handler::<CWorldSendCameraServer>();
        self.register_game_handler::<CSendSelectedObjID>();
        self.register_game_handler::<CExit>();
        self.register_game_handler::<CWorldLockCameraServer>();
    }
}

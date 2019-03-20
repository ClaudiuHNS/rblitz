use enet_sys::ENetPeer;
use indexmap::IndexMap;
use shred::SystemData;
use specs::World;

use crate::{
    client::{ClientConnectionMap, ClientId, ClientMap, ClientStatus},
    lenet_server::{Event, LENetServer},
    packet::{
        chat,
        dispatcher_sys::PacketSender,
        game::{PacketHandler, PacketHandlerDummy, PacketHandlerImpl, RawGamePacket},
        Channel, KeyCheck,
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
            .write_resource::<ClientConnectionMap>()
            .decrypt_client(cid, data);
        match channel {
            //handled outside of this
            Channel::Handshake => (),
            Channel::ClientToServer
            | Channel::SyncClock
            | Channel::Broadcast
            | Channel::BroadcastUnreliable => {
                let packet = RawGamePacket::from_slice(data).unwrap();
                if let Some(handler) = self.game_handlers.get(&packet.id) {
                    let _ = handler
                        .handle(&world.res, cid, packet.sender_net_id, packet.data)
                        .map_err(|e| log::warn!("{}", e));
                // ignore CWorldSendCamera cause it spams the logs
                } else if packet.id != 0x30 {
                    log::debug!(
                        "Unhandled Packet 0x{:X} received on channel {:?}",
                        packet.id,
                        channel,
                    );
                }
            },
            Channel::Chat => {
                let _ = chat::handle_chat_message(world, cid, data);
            },
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

    fn on_connect(&self, world: &World, keycheck: KeyCheck, peer: *mut ENetPeer) {
        let mut clients = world.write_resource::<ClientMap>();
        let mut connections = world.write_resource::<ClientConnectionMap>();
        if let Some((cid, client)) = clients
            .iter_mut()
            .find(|(_, c)| c.player_id == keycheck.player_id)
        {
            let cid = *cid;
            match connections.auth(cid, client.player_id, keycheck, peer) {
                Ok(()) => {
                    client.status = ClientStatus::Loading;
                    let sender = PacketSender::fetch(&world.res);
                    for (cid_iter, client) in clients.iter() {
                        let mut check_id = client.player_id.to_le_bytes();
                        connections.encrypt_client(*cid_iter, &mut check_id);
                        sender.single(Channel::Handshake, cid, unsafe {
                            core::slice::from_raw_parts(
                                &KeyCheck {
                                    action: 0,
                                    pad: [0, 0, 0],
                                    client_id: cid_iter.0,
                                    player_id: client.player_id,
                                    check_id,
                                } as *const _ as *const u8,
                                core::mem::size_of::<KeyCheck>(),
                            )
                            .to_owned()
                            .into_boxed_slice()
                        });
                    }
                },
                Err(e) => {
                    log::warn!("{:?}", e);
                    unsafe { enet_sys::enet_peer_disconnect_now(peer, 0) };
                },
            }
        }
    }

    pub fn run(&self, server: &mut LENetServer, world: &World) {
        loop {
            match server.service(0) {
                Ok(event) => match event {
                    Event::Connected(keycheck, peer) => {
                        self.on_connect(world, keycheck, peer);
                    },
                    Event::Disconnected(cid) => {
                        log::info!("Disconnected: {:?}", cid);
                        let mut clients = world.write_resource::<ClientMap>();
                        let client = clients.get_mut(&cid).unwrap();
                        client.status = ClientStatus::Disconnected;
                        world
                            .write_resource::<ClientConnectionMap>()
                            .disconnect(cid);
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
        self.register_game_handler::<CSendSelectedObjID>();
        self.register_game_handler::<CExit>();
        self.register_game_handler::<CWorldLockCameraServer>();
        self.register_game_handler::<CSyncSimTime>();
        self.register_game_handler::<CWaypointAck>();
        self.register_game_handler::<CNpcIssueOrderReq>();
    }
}

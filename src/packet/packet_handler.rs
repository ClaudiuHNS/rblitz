use indexmap::IndexMap;
use shred_derive::SystemData;
use specs::{Read, Resources, System, WriteExpect};

use crate::client::{ClientId, ClientMap};
use crate::lenet_server::{Event, LENetServer};
use crate::packet::{
    game::{GameHandler, GamePacketHandler, RawGamePacket},
    loading_screen::{LoadingScreenHandler, LoadingScreenPacket, RawLoadingScreenPacket},
    Channel,
};

pub struct PacketHandler {
    loading_screen_handlers: IndexMap<u8, LoadingScreenHandler>,
    game_handlers: IndexMap<u8, GameHandler>,
}

impl PacketHandler {
    pub fn new() -> Self {
        PacketHandler {
            loading_screen_handlers: IndexMap::with_capacity(4),
            game_handlers: IndexMap::with_capacity(32),
        }
    }

    pub fn handle_packet(
        &self,
        world: &mut WorldData,
        channel: u8,
        cid: ClientId,
        data: &mut [u8],
    ) {
        let channel = Channel::try_from(channel).expect("unknown channel received");
        world.clients.get(&cid).unwrap().decrypt(data);
        match channel {
            //handled outside of this
            Channel::Handshake => (),
            Channel::ClientToServer
            | Channel::SyncClock
            | Channel::Broadcast
            | Channel::BroadcastUnreliable => {
                let packet = RawGamePacket::from_slice(data).unwrap();
                if let Some(handler) = self.game_handlers.get(&packet.id) {
                    handler(world, cid, packet.sender_net_id, packet.data).unwrap();
                } else {
                    log::debug!(
                        "Unhandled Packet 0x{:X} received on channel {:?}",
                        packet.id,
                        channel,
                    );
                }
            }
            Channel::Chat => (),
            Channel::LoadingScreen => {
                let packet = RawLoadingScreenPacket::from_slice(data).unwrap();
                if let Some(handler) = self.loading_screen_handlers.get(&packet.id) {
                    handler(world, cid, packet.data).unwrap();
                } else {
                    log::warn!(
                        "Unknown Packet 0x{:X} received on LoadingScreenChannel",
                        packet.id,
                    );
                }
            }
        }
    }

    pub fn register_loading_screen_handler<P>(&mut self)
    where
        P: LoadingScreenPacket,
    {
        assert!(
            self.loading_screen_handlers
                .insert(P::ID, P::handle)
                .is_none(),
            "Loading Screen handler replaced for 0x{id:X}, check that it isn't being registered\
             twice and that the ID(0x{id:X}) is correct",
            id = P::ID
        )
    }

    pub fn register_game_handler<P>(&mut self)
    where
        P: GamePacketHandler,
    {
        assert!(
            self.game_handlers.insert(P::ID, P::handle).is_none(),
            "Game handler replaced for 0x{id:X}, check that it isn't being registered twice and\
             that the ID(0x{id:X}) is correct",
            id = P::ID
        );
    }

    fn register_game_packets(&mut self) {
        use rblitz_packets::packets::game::{request::*, *};
        self.register_game_handler::<CQueryStatusReq>();
        self.register_game_handler::<CSyncVersion>();
        self.register_game_handler::<CCharSelected>();
        self.register_game_handler::<CPingLoadInfo>();
        self.register_game_handler::<CClientReady>();
    }

    fn register_loading_screen_packets(&mut self) {
        use rblitz_packets::packets::loading_screen::*;
        self.register_loading_screen_handler::<RequestJoinTeam>();
        self.register_loading_screen_handler::<RequestRename>();
        self.register_loading_screen_handler::<RequestReskin>();
        self.register_loading_screen_handler::<TeamRosterUpdate>();
    }
}

#[derive(SystemData)]
pub struct WorldData<'a> {
    pub time: Read<'a, crate::game_server::GameTime>,
    pub clients: WriteExpect<'a, ClientMap>,
    pub server: WriteExpect<'a, LENetServer>,
}

impl<'a> System<'a> for PacketHandler {
    type SystemData = WorldData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        match data.server.service(1) {
            Ok(event) => match event {
                Event::Connected(keycheck, peer) => {
                    let cid = ClientId(keycheck.player_id as u32);
                    if let Some(client) = data.clients.get_mut(&cid) {
                        client.auth(cid, keycheck, peer);
                    }
                }
                Event::Disconnected(cid) => log::info!("Disconnected: {:?}", cid),
                Event::Packet(cid, channel, mut packet) => {
                    self.handle_packet(&mut data, channel, cid, &mut *packet)
                }
                Event::NoEvent => (),
            },
            Err(e) => log::error!("{:?}", e),
        }
    }

    fn setup(&mut self, _res: &mut Resources) {
        //Self::SystemData::setup(res);
        self.register_loading_screen_packets();
        self.register_game_packets();
    }
}

use indexmap::IndexMap;
use specs::World;

use crate::client::{ClientId, ClientMap};
use crate::packet::{
    game::{GameHandler, GamePacketHandler, RawGamePacket},
    loading_screen::{LoadingScreenHandler, LoadingScreenPacket, RawLoadingScreenPacket},
    Channel,
};

pub struct PacketHandler {
    loading_screen_handlers: IndexMap<u8, LoadingScreenHandler>,
    game_handlers: [Option<GameHandler>; 255],
}

impl PacketHandler {
    pub fn new() -> Self {
        let mut this = PacketHandler {
            loading_screen_handlers: IndexMap::with_capacity(4),
            game_handlers: [None; 255],
        };
        this.register_all();
        this
    }

    pub fn handle_packet(&self, world: &mut World, channel: u8, cid: ClientId, data: &mut [u8]) {
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
                log::trace!("Handling 0x{:X} on channel {:?}", packet.id, channel,);
                if let Some(handler) = &self.game_handlers[packet.id as usize] {
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
            self.game_handlers[P::ID as usize]
                .replace(P::handle)
                .is_none(),
            "Game handler replaced for 0x{id:X}, check that it isn't being registered twice and\
             that the ID(0x{id:X}) is correct",
            id = P::ID
        );
    }

    fn register_all(&mut self) {
        self.register_loading_screen_packets();
        self.register_game_packets();
    }

    fn register_game_packets(&mut self) {
        use rblitz_packets::packets::game::*;
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

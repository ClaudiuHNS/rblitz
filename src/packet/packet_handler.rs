use indexmap::IndexMap;
use shred::DynamicSystemData;
use shred_derive::SystemData;
use specs::{Read, ReadStorage, Resources, System, WriteExpect};

use crate::{
    client::{ClientId, ClientMap, ClientStatus},
    lenet_server::{Event, LENetServer},
    packet::{
        game::{GameHandler, GamePacketHandler, RawGamePacket},
        Channel,
    },
    world::{
        components::{NetId, SummonerSpells, Team, UnitName},
        resources::GameTime,
    },
};

pub struct PacketHandler {
    game_handlers: IndexMap<u8, GameHandler>,
}

impl PacketHandler {
    pub fn new() -> Self {
        PacketHandler {
            game_handlers: IndexMap::new(),
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
                use rblitz_packets::packets::{loading_screen::RequestJoinTeam, PacketId};
                if !data.is_empty() && RequestJoinTeam::ID == data[0] {
                    world
                        .clients
                        .send_roster_update(&world.unit_names, &world.teams, cid);
                }
            }
        }
    }

    fn register_game_handler<P>(&mut self)
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

    fn register_game_packet_handlers(&mut self) {
        self.game_handlers.reserve(8);

        use rblitz_packets::packets::game::{request::*, *};
        self.register_game_handler::<CQueryStatusReq>();
        self.register_game_handler::<CSyncVersion>();
        self.register_game_handler::<CCharSelected>();
        self.register_game_handler::<CPingLoadInfo>();
        self.register_game_handler::<CClientReady>();
        self.register_game_handler::<CWorldSendCameraServer>();
        self.register_game_handler::<CSendSelectedObjID>();
        self.register_game_handler::<CExit>();
    }
}

#[derive(SystemData)]
pub struct WorldData<'a> {
    pub time: Read<'a, GameTime>,
    pub clients: WriteExpect<'a, ClientMap>,
    pub server: WriteExpect<'a, LENetServer>,
    pub unit_names: ReadStorage<'a, UnitName>,
    pub net_ids: ReadStorage<'a, NetId>,
    pub teams: ReadStorage<'a, Team>,
    pub summoner_spells: ReadStorage<'a, SummonerSpells>,
    // this could possibly all be replaced by just using shred::Resources?
    // but then
}

impl<'a> System<'a> for PacketHandler {
    type SystemData = WorldData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        loop {
            match data.server.service(0) {
                Ok(event) => match event {
                    Event::Connected(keycheck, peer) => {
                        let cid = data
                            .clients
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
                            Some(cid) => data.clients.broadcast_keycheck(cid),
                            None => unsafe { enet_sys::enet_peer_disconnect_now(peer, 0) },
                        }
                    }
                    Event::Disconnected(cid) => {
                        log::info!("Disconnected: {:?}", cid);
                        let client = data.clients.get_mut(&cid).unwrap();
                        client.status = ClientStatus::Disconnected;
                        client.peer = None;
                        if data
                            .clients
                            .values()
                            .all(|c| c.status == ClientStatus::Disconnected)
                        {
                            // FIXME shutdown server gracefully
                            panic!("All players lost connection, terminating server");
                        }
                    }
                    Event::Packet(cid, channel, mut packet) => {
                        self.handle_packet(&mut data, channel, cid, &mut *packet)
                    }
                    Event::NoEvent => break,
                },
                Err(e) => log::error!("{:?}", e),
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        <Self::SystemData as DynamicSystemData>::setup(&self.accessor(), res);
        self.register_game_packet_handlers();
    }
}

use specs::World;

use std::net::Ipv4Addr;
use std::time::Instant;

use crate::client::{Client, ClientId, ClientMap};
use crate::lenet_server::{Event, LENetServer};
use crate::packet::{packet_handler::PacketHandler, KeyCheck};

const TICK_RATE: f64 = 1.0 / 30.0;

pub struct GameTime(pub f64);

pub struct GameServer {
    server: LENetServer,
    packet_handler: PacketHandler,
    world: World,
}

fn to_enet_address(address: Ipv4Addr, port: u16) -> enet_sys::ENetAddress {
    let octets = address.octets();
    enet_sys::ENetAddress {
        host: (octets[0] as u32)
            | (octets[1] as u32) << 8
            | (octets[2] as u32) << 16
            | (octets[3] as u32) << 24,
        port,
    }
}

impl GameServer {
    #[allow(clippy::cast_lossless)]
    pub fn new(address: Ipv4Addr, port: u16, keys: [(u32, [u8; 16]); 12]) -> Result<Self, ()> {
        let server = LENetServer::new(to_enet_address(address, port));
        let mut world = World::new();
        world.add_resource(GameTime(0.0));
        world.add_resource(
            keys.iter()
                .map(|(cid, key)| (ClientId(*cid), Client::new(&key[..])))
                .collect::<ClientMap>(),
        );
        Ok(GameServer {
            server,
            packet_handler: PacketHandler::new(),
            world,
        })
    }

    pub fn run(&mut self) {
        let mut last_instant = Instant::now();
        let mut delta_sum = 0.0;
        loop {
            let elapsed = last_instant.elapsed();
            last_instant = Instant::now();
            let delta =
                (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64) / 1_000_000_000.0;
            delta_sum += delta;
            self.world.write_resource::<GameTime>().0 += delta;

            match self.server.service(1) {
                Ok(event) => {
                    match event {
                        Event::Connected(keycheck, peer) => self.auth_peer(keycheck, peer),
                        Event::Disconnected(cid) => self.player_disconnected(cid),
                        Event::Packet(cid, channel, mut packet) => self
                            .packet_handler
                            .handle_packet(&mut self.world, channel, cid, &mut *packet),
                        Event::NoEvent => (),
                    }
                }
                Err(e) => log::error!("{:?}", e),
            }

            if delta_sum >= TICK_RATE {
                delta_sum -= TICK_RATE;
                self.tick();
            }
        }
    }

    fn auth_peer(&mut self, packet: KeyCheck, peer: *mut enet_sys::ENetPeer) {
        let cid = ClientId(packet.player_id as u32);
        if let Some(client) = self.world.write_resource::<ClientMap>().get_mut(&cid) {
            client.auth(cid, packet, peer);
        }
    }

    pub fn tick(&mut self) {}

    fn player_disconnected(&mut self, cid: ClientId) {
        log::info!("Disconnected: {:?}", cid);
    }
}

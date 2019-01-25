use specs::{Dispatcher, DispatcherBuilder, World};

use std::net::Ipv4Addr;
use std::time::Instant;

use crate::client::{Client, ClientId, ClientMap};
use crate::lenet_server::LENetServer;
use crate::packet::packet_handler::PacketHandler;

const TICK_RATE: f64 = 1.0 / 30.0;

#[derive(Default)]
pub struct GameTime(pub f64);

pub struct GameServer<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

#[allow(clippy::cast_lossless)]
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

impl GameServer<'_, '_> {
    pub fn new(address: Ipv4Addr, port: u16, keys: [(u32, [u8; 16]); 12]) -> Result<Self, ()> {
        let server = LENetServer::new(to_enet_address(address, port));
        let mut world = World::new();
        world.add_resource(GameTime(0.0));
        world.add_resource(server);
        world.add_resource(
            keys.iter()
                .map(|(cid, key)| (ClientId(*cid), Client::new(&key[..])))
                .collect::<ClientMap>(),
        );
        let mut dispatcher = DispatcherBuilder::new()
            .with(PacketHandler::new(), "packet_handler", &[])
            .build();
        dispatcher.setup(&mut world.res);
        Ok(GameServer { world, dispatcher })
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
            self.dispatcher.dispatch(&self.world.res);

            if delta_sum >= TICK_RATE {
                delta_sum -= TICK_RATE;
                self.tick();
            }

            self.world.maintain();
        }
    }

    pub fn tick(&mut self) {}
}

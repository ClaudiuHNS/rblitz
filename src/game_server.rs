use specs::{Dispatcher, DispatcherBuilder, World};

use std::{net::Ipv4Addr, time::Instant};

use crate::{
    client::ClientMap,
    config::PlayerConfig,
    lenet_server::LENetServer,
    packet::packet_handler_system::PacketHandlerSys,
    world::{
        components::{NetId, SummonerSpells, Team, UnitName},
        resources::GameTime,
    },
};

const TICK_RATE: f64 = 1.0 / 30.0;

pub struct GameServer<'a, 'b> {
    world: World,
    server: LENetServer,
    packet_handler: PacketHandlerSys<'a>,
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

impl<'a, 'b> GameServer<'a, 'b> {
    pub fn new(address: Ipv4Addr, port: u16, players: Vec<PlayerConfig>) -> Result<Self, ()> {
        let server = LENetServer::new(to_enet_address(address, port));
        let mut world = World::new();
        world.add_resource(GameTime(0.0));
        world.register::<NetId>();
        world.register::<Team>();
        world.register::<UnitName>();
        world.register::<SummonerSpells>();
        let mut dispatcher = DispatcherBuilder::new().build();
        dispatcher.setup(&mut world.res);
        ClientMap::init_from_config(&mut world, players);
        Ok(GameServer {
            world,
            packet_handler: PacketHandlerSys::new(),
            server,
            dispatcher,
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
            self.packet_handler.run(&mut self.server, &self.world);
            self.dispatcher.dispatch_seq(&self.world.res);
            self.dispatcher.dispatch_thread_local(&self.world.res);

            if delta_sum >= TICK_RATE {
                delta_sum -= TICK_RATE;
                self.tick();
            }

            self.world.maintain();

            std::thread::sleep_ms(1);
        }
    }

    pub fn tick(&mut self) {}
}

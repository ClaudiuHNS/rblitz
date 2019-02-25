use specs::{Dispatcher, DispatcherBuilder, World};

use std::{net::Ipv4Addr, time::Instant};

use crate::{
    client::init_clients_from_config,
    config::PlayerConfig,
    lenet_server::LENetServer,
    packet::{dispatcher_sys::PacketDispatcher, handler_sys::PacketHandlerSys},
    world::{
        components::{NetId, SummonerSpells, Team, UnitName},
        resources::Time,
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
        world.add_resource(Time::new());
        // temporary
        {
            world.register::<NetId>();
            world.register::<Team>();
            world.register::<UnitName>();
            world.register::<SummonerSpells>();
        }
        let (packet_channel_send, packet_channel_receive) = crossbeam_channel::unbounded();
        world.add_resource(packet_channel_send);
        let mut dispatcher = DispatcherBuilder::new()
            .with_thread_local(PacketDispatcher::new(packet_channel_receive))
            .build();
        dispatcher.setup(&mut world.res);
        init_clients_from_config(&mut world, players);
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
            delta_sum += self
                .world
                .write_resource::<Time>()
                .set_delta(last_instant.elapsed());
            last_instant = Instant::now();

            self.packet_handler.run(&mut self.server, &self.world);

            if delta_sum >= TICK_RATE {
                delta_sum -= TICK_RATE;
                self.dispatcher.dispatch_seq(&self.world.res);
                self.dispatcher.dispatch_thread_local(&self.world.res);
            }

            self.world.maintain();

            std::thread::sleep_ms(1);
        }
    }
}

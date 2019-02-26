#![deny(bare_trait_objects)]
#![allow(clippy::cast_lossless)]

pub mod config;
pub mod game_server;

mod client;
mod error;
mod lenet_server;
//mod nav_grid;
mod packet;
mod systems;
mod world;

// The client is unable to handle more than this
const PLAYER_COUNT_MAX: usize = 12;

const TICKS_PER_SECOND: u64 = 30;
const TICK_RATE: f64 = 1.0 / TICKS_PER_SECOND as f64;

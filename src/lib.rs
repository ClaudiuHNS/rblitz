#![deny(bare_trait_objects)]
#![allow(clippy::cast_lossless)]

pub mod config;
pub mod game_server;

mod client;
mod components;
mod error;
mod lenet_server;
mod packet;
mod resources;

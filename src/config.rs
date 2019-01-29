use serde::Deserialize;

use crate::client::Team;
use std::{fs, io, path::Path};

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        // todo replace `expect` with error enum for io and toml error
        Ok(toml::from_str(&fs::read_to_string(path)?).expect("unexpected data"))
    }
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Pla3yerConfig {
    pub entries: Vec<PlayerConfig>,
}

#[derive(Deserialize)]
pub struct PlayerConfig {
    pub name: String,
    pub key: String,
    pub player_id: u64,
    pub team: Team,
    pub champion: String,
    pub skin_id: u32,
    pub summoner_level: u16,
    pub summoner_spell0: u32,
    pub summoner_spell1: u32,
    pub profile_icon: i32,
}

impl PlayerConfig {
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Vec<Self>> {
        Ok(ron::de::from_str(&fs::read_to_string(path)?).expect("unexpected data"))
    }
}

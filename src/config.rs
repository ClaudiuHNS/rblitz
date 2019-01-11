use serde::Deserialize;

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
    pub keys: Vec<String>,
}

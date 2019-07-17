use std::{
    path::Path,
    fs::File,
    io::Read,
    collections::HashMap,
};
use failure::prelude::*;
use toml;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsensusPeersConfig {
    pub peers: HashMap<String, PeerConfig>,
}

impl ConsensusPeersConfig {
    pub fn load_config<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        let mut file = File::open(path)
            .unwrap_or_else(|_| panic!("Cannot open Trusted Peers Config file {:?}", path));
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .unwrap_or_else(|_| panic!("Error reading Trusted Peers Config file {:?}", path));

        Self::parse(&contents)
    }

    fn parse(config_string: &str) -> Self {
        toml::from_str(config_string).expect("Unable to parse Config")
    }
}

pub fn load_consensus_peers_config() -> ConsensusPeersConfig {
    let dirEntry = std::fs::read_dir(crate::CONFIG_PATH).unwrap();
    for entry in dirEntry {
        let config_path = entry.unwrap().path();
        let config_path_str = config_path.to_str().unwrap();
        if config_path_str.find("consensus_peers.config.toml").is_some() {
            return ConsensusPeersConfig::load_config(config_path_str);
        }
    }
    panic!("Failed to load consensus peers config");
}
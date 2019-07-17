use std::{
    path::Path,
    fs::File,
    io::Read,
};
use failure::prelude::*;
use toml;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServiceConfig {
    pub peer_id: String,
    pub address: String,
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NodeConfig {
    pub service: ServiceConfig,
}

impl NodeConfig {
    pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self::load_template(&path)?)
    }

    /// Reads the config file and returns the configuration object
    pub fn load_template<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let mut file =
            File::open(path).with_context(|_| format!("Cannot open NodeConfig file {:?}", path))?;
        let mut config_string = String::new();
        file.read_to_string(&mut config_string)
            .with_context(|_| format!("Cannot read NodeConfig file {:?}", path))?;

        let config = Self::parse(&config_string)
            .with_context(|_| format!("Cannot parse NodeConfig file {:?}", path))?;

        Ok(config)
    }

    /// Parses the config file into a Config object
    pub fn parse(config_string: &str) -> Result<Self> {
        assert!(!config_string.is_empty());
        Ok(toml::from_str(config_string)?)
    }
}

pub fn load_node_config() -> NodeConfig {
    let dir_entry = std::fs::read_dir(crate::CONFIG_PATH).unwrap();
    for entry in dir_entry {
        let config_path = entry.unwrap().path();
        let config_path_str = config_path.to_str().unwrap();
        if config_path_str.find("node.config.toml").is_some() {
            return NodeConfig::load_config(config_path_str).expect("NodeConfig");
        }
    }
    panic!("Failed to load node config");
}

#[test]
fn test_config() {
    // This test verifies that all configs in data/config are valid
    let paths = std::fs::read_dir("config").expect("cannot read config dir");

    for path in paths {
        let config_path = path.unwrap().path();
        let config_path_str = config_path.to_str().unwrap();
        if config_path_str.find("node.config.toml").is_some() {
            println!("Loading {}", config_path_str);
            let _ = NodeConfig::load_config(config_path_str).expect("NodeConfig");
        } else {
            println!("Invalid file {} for verifying", config_path_str);
        }
    }
}
use crate::consensus_peers;
use crate::node_config;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MirConfig {
    pub consensus_config: consensus_peers::ConsensusPeersConfig,
    pub node_config: node_config::NodeConfig,
}

impl MirConfig {
    pub fn load() -> Self {
        MirConfig {
            consensus_config: consensus_peers::load_consensus_peers_config(),
            node_config: node_config::load_node_config(),
        }
    }
}

use crate::bucket::Bucket;
use crate::*;
use config::MirConfig;
use std::collections::HashMap;

pub struct Epoch {
    // number is the epoch number
    pub number: u64,
    pub nodes: Vec<NodeID>,
    pub buckets: HashMap<BucketID, Bucket>,
}

impl Epoch {
    pub fn new(config: MirConfig) -> Self {
        let mut buckets = HashMap::new();
        let mut nodes = Vec::new();
        for (id, _v) in config.consensus_config.peers {
            let id = id.parse::<u64>().unwrap();
            buckets.insert(id, Bucket::new(id, id));
            nodes.push(id);
        }
        Epoch {
            number: 0,
            nodes,
            buckets,
        }
    }
}

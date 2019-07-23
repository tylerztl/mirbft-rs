use std::collections::HashMap;
use crate::*;
use crate::bucket::Bucket;
use config::node_config::NodeConfig;

struct EpochConfig {}

pub struct Epoch {
    buckets: HashMap<BucketID, Bucket>,
}

impl Epoch {
    pub fn new(config: NodeConfig) -> Self {
        Epoch {
            buckets: HashMap::new(),
        }
    }
}
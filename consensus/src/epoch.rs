use crate::bucket::Bucket;
use crate::*;
use config::node_config::NodeConfig;
use std::collections::HashMap;

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

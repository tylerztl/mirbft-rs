use crate::epoch::Epoch;
use config::node_config::NodeConfig;
use crypto::hash::{hash as HashValue, Digest};

pub struct StateMachine {
    config: NodeConfig,
    msg_queues: Vec<Vec<u8>>,
    current_epoch: Epoch,
}

impl StateMachine {
    pub fn new(config: NodeConfig) -> Self {
        StateMachine {
            config: config.clone(),
            msg_queues: Vec::new(),
            current_epoch: Epoch::new(config),
        }
    }

    pub fn propose(&mut self, data: Vec<u8>) {
        //        let digest = HashValue(&data);
        self.msg_queues.push(data);
    }
}

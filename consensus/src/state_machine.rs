use crate::epoch::Epoch;
use crate::{BucketID, NodeID};
use byteorder::{BigEndian, ReadBytesExt};
use config::MirConfig;
use crypto::hash::{hash as HashValue, Digest};
use logger::prelude::*;

pub struct StateMachine {
    config: MirConfig,
    pub msg_queues: Vec<Vec<u8>>,
    pub current_epoch: Epoch,
}

impl StateMachine {
    pub fn new(config: MirConfig) -> Self {
        StateMachine {
            config: config.clone(),
            msg_queues: Vec::new(),
            current_epoch: Epoch::new(config),
        }
    }

    pub fn propose(&mut self, data: Vec<u8>) -> (bool, NodeID, BucketID) {
        let digest: Digest = HashValue(&data);
        let mut buf = &digest[0..8];
        let num = buf.read_u64::<BigEndian>().unwrap();
        let bucket_id: BucketID = num % self.current_epoch.buckets.len() as u64;
        let leader_node: NodeID = self.current_epoch.buckets[&bucket_id].leader;
        info!(
            "receive proposal bucket: {}, leader: {}",
            bucket_id, leader_node
        );
        if leader_node == self.config.node_config.service.peer_id {
            info!("receive new proposal add to msg queue");
            self.msg_queues.push(data);
        }
        (
            leader_node == self.config.node_config.service.peer_id,
            leader_node,
            bucket_id,
        )
    }
}

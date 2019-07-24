use crate::epoch::Epoch;
use crate::{BucketID, NodeID};
use byteorder::{BigEndian, ReadBytesExt};
use config::MirConfig;
use crypto::hash::{hash as HashValue, Digest};
use logger::prelude::*;
use proto::proto::mirbft::{Message, Preprepare};

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

    pub fn handle_batch(&mut self) -> Option<Message> {
        let mut queue_len = self.msg_queues.len();
        if queue_len == 0 {
            return None;
        }
        let max_len = self.config.consensus_config.consensus.batch_size;
        if queue_len > max_len {
            queue_len = max_len;
        }
        let mut message = Message::new();
        let mut preprepare_msg = Preprepare::new();
        let mut batch = Vec::new();

        for _i in 0..queue_len {
            let msg = self.msg_queues.remove(0);
            batch.push(msg);
        }
        preprepare_msg.set_batch(protobuf::RepeatedField::from_vec(batch));
        message.set_preprepare(preprepare_msg);
        Some(message)
    }
}

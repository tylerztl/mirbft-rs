use crate::epoch::Epoch;
use crate::sequence::Entry;
use crate::{BucketID, NodeID, SeqNo};
use byteorder::{BigEndian, ReadBytesExt};
use config::MirConfig;
use crypto::hash::{hash as HashValue, Digest};
use logger::prelude::*;
use proto::proto::mirbft::{Commit, Message, Prepare, Preprepare};

pub struct StateMachine {
    config: MirConfig,
    quorum: usize,
    next_seq: SeqNo,
    next_bucket: BucketID,
    pub msg_queues: Vec<Vec<u8>>,
    pub current_epoch: Epoch,
}

impl StateMachine {
    pub fn new(config: MirConfig) -> Self {
        StateMachine {
            config: config.clone(),
            quorum: 2 * config.consensus_config.consensus.f + 1,
            next_seq: 1,
            next_bucket: 0,
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
        let mut preprepare = Preprepare::new();
        let mut batch = Vec::new();

        for _i in 0..queue_len {
            let msg = self.msg_queues.remove(0);
            batch.push(msg);
        }
        preprepare.set_seq_no(self.next_seq);
        preprepare.set_epoch(self.current_epoch.number);
        preprepare.set_bucket(self.current_epoch.owned_buckets[self.next_seq as usize]);
        preprepare.set_batch(protobuf::RepeatedField::from_vec(batch));
        message.set_preprepare(preprepare);

        self.next_bucket = (self.next_bucket + 1) % self.current_epoch.owned_buckets.len() as u64;
        if self.next_bucket == 0 {
            self.next_seq += 1;
        }

        Some(message)
    }

    pub fn preprepare(&mut self, msg: Preprepare) -> Option<Message> {
        if msg.bucket > self.current_epoch.buckets.len() as u64 {
            error!("received message for bad bucket: {}", msg.bucket);
            return None;
        }
        let mut message = Message::new();
        let mut prepare = Prepare::new();

        let mut hashes = Vec::new();
        for data in msg.batch.iter() {
            hashes = hashes.iter().chain(data).cloned().collect();
        }
        let digest: Digest = HashValue(hashes);
        prepare.set_seq_no(msg.seq_no);
        prepare.set_epoch(msg.epoch);
        prepare.set_bucket(msg.bucket);
        prepare.set_digest(digest.iter().cloned().collect());
        message.set_prepare(prepare);

        self.current_epoch.apply_preprepare(
            digest,
            Entry {
                seq_no: msg.seq_no,
                epoch: msg.epoch,
                bucket_id: msg.bucket,
                batch: protobuf::RepeatedField::to_vec(&msg.batch),
            },
        );

        Some(message)
    }

    pub fn prepare(&mut self, source: NodeID, msg: Prepare) -> Option<Message> {
        let mut array = [0u8; 32];
        for (&x, p) in msg.digest.clone().iter().zip(array.iter_mut()) {
            *p = x;
        }

        let is_ok =
            self.current_epoch
                .apply_prepare(source, msg.seq_no, msg.bucket, array, self.quorum);
        if !is_ok {
            return None;
        }
        let mut message = Message::new();
        let mut commit = Commit::new();
        commit.set_seq_no(msg.seq_no);
        commit.set_epoch(msg.epoch);
        commit.set_bucket(msg.bucket);
        commit.set_digest(msg.digest);
        message.set_commit(commit);
        Some(message)
    }

    pub fn commit(&mut self, source: NodeID, msg: Commit) {
        let mut array = [0u8; 32];
        for (&x, p) in msg.digest.clone().iter().zip(array.iter_mut()) {
            *p = x;
        }

        if self
            .current_epoch
            .apply_commit(source, msg.seq_no, msg.bucket, array, self.quorum)
        {
            info!("Message:{:?} consensus succeed!", msg);
        }
    }
}

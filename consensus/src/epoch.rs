use crate::bucket::Bucket;
use crate::sequence::Entry;
use crate::*;
use config::MirConfig;
use crypto::hash::Digest;
use std::collections::HashMap;

pub struct Epoch {
    // number is the epoch number
    pub number: u64,
    // current node owned buckets
    pub owned_buckets: Vec<BucketID>,
    pub buckets: HashMap<BucketID, Bucket>,
}

impl Epoch {
    pub fn new(config: MirConfig) -> Self {
        let mut buckets = HashMap::new();
        let mut owned_buckets = Vec::new();
        let total_buckets =
            config.consensus_config.consensus.m * config.consensus_config.peers.len();
        for id in 0..total_buckets {
            let id = id as u64;
            let leader: NodeID = id % config.consensus_config.peers.len() as u64 + 1;
            if leader == config.node_config.service.peer_id {
                owned_buckets.push(leader);
            }
            buckets.insert(id, Bucket::new(leader, id));
        }
        Epoch {
            number: 0,
            owned_buckets,
            buckets,
        }
    }

    pub fn apply_preprepare(&mut self, digest: Digest, entry: Entry) {
        self.buckets
            .get_mut(&entry.bucket_id)
            .unwrap()
            .apply_preprepare(digest, entry);
    }

    pub fn apply_prepare(
        &mut self,
        source: NodeID,
        seq_no: SeqNo,
        bucket_id: BucketID,
        digest: Digest,
        quorum: usize,
    ) -> bool {
        self.buckets
            .get_mut(&bucket_id)
            .unwrap()
            .apply_prepare(source, seq_no, digest, quorum)
    }

    pub fn apply_commit(
        &mut self,
        source: NodeID,
        seq_no: SeqNo,
        bucket_id: BucketID,
        digest: Digest,
        quorum: usize,
    ) -> bool {
        self.buckets
            .get_mut(&bucket_id)
            .unwrap()
            .apply_commit(source, seq_no, digest, quorum)
    }
}

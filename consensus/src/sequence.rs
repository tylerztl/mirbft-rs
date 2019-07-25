use crate::{BucketID, NodeID, SeqNo};
use crypto::hash::Digest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SequenceState {
    Uninitialized,
    Preprepared,
    Prepared,
    Committed,
}

pub struct Entry {
    pub epoch: u64,
    pub seq_no: SeqNo,
    pub bucket_id: BucketID,
    pub batch: Vec<Vec<u8>>,
}

pub struct Sequence {
    pub state: SequenceState,
    pub entry: Entry,
    pub digest: Digest,
    pub prepares: HashMap<String, NodeID>,
    pub commits: HashMap<String, NodeID>,
}

impl Default for Sequence {
    fn default() -> Self {
        Sequence {
            state: SequenceState::Uninitialized,
            entry: Entry {
                epoch: 0,
                seq_no: 0,
                bucket_id: 0,
                batch: Vec::new(),
            },
            digest: [0u8; 32],
            prepares: HashMap::new(),
            commits: HashMap::new(),
        }
    }
}

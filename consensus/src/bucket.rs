use std::collections::HashMap;

use crate::sequence::Sequence;
use crate::*;

pub struct Bucket {
    leader: NodeID,
    id: BucketID,
    sequences: HashMap<SeqNo, Sequence>,
}

impl Bucket {
    pub fn new(leader: NodeID, id: BucketID) -> Self {
        Bucket {
            leader,
            id,
            sequences: HashMap::new(),
        }
    }
}

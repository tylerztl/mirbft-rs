use std::collections::HashMap;

use crate::sequence::Sequence;
use crate::*;

pub struct Bucket {
    pub leader: NodeID,
    pub id: BucketID,
    pub sequences: HashMap<SeqNo, Sequence>,
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

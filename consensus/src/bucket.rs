use crate::sequence::{Entry, Sequence, SequenceState};
use crate::*;
use crypto::hash::Digest;
use std::collections::HashMap;

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

    pub fn apply_preprepare(&mut self, digest: Digest, entry: Entry) {
        let mut sequence = Sequence::default();
        let seq_no = entry.seq_no;
        sequence.state = SequenceState::Preprepared;
        sequence.entry = entry;
        sequence.digest = digest;
        self.sequences.insert(seq_no, sequence);
    }

    pub fn apply_prepare(
        &mut self,
        source: NodeID,
        seq_no: SeqNo,
        digest: Digest,
        quorum: usize,
    ) -> bool {
        let sequence = self.sequences.get_mut(&seq_no);
        if sequence.is_none() {
            let mut sequence = Sequence::default();
            sequence.state = SequenceState::Preprepared;
            sequence.digest = digest.clone();
            self.sequences.insert(seq_no, sequence);
        }
        let sequence = self.sequences.get_mut(&seq_no).unwrap();
        let agreements = sequence.handle_prepares(digest, source);
        let is_ok = agreements >= quorum;
        if is_ok {
            sequence.state = SequenceState::Prepared;
        }
        is_ok
    }

    pub fn apply_commit(
        &mut self,
        source: NodeID,
        seq_no: SeqNo,
        digest: Digest,
        quorum: usize,
    ) -> bool {
        let sequence = self.sequences.get_mut(&seq_no).unwrap();
        let agreements = sequence.handle_commits(digest, source);
        let is_ok = agreements >= quorum;
        if is_ok {
            sequence.state = SequenceState::Prepared;
        }
        is_ok
    }
}

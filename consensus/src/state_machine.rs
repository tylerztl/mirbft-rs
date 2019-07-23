use std::collections::HashMap;
use crypto::hash::{
    Digest,
    hash as HashValue,
};

pub struct StateMachine {
    peer_id: usize,
    msg_queues: HashMap<Digest, Vec<u8>>,
}

impl StateMachine {
    pub fn new(peer_id: usize) -> Self {
        StateMachine {
            peer_id,
            msg_queues: HashMap::new(),
        }
    }

    pub fn propose(&mut self, data: Vec<u8>) {
        let digest = HashValue(&data);
        self.msg_queues.insert(digest, data.clone());
    }

}
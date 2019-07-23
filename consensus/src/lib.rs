#[macro_use]
extern crate crossbeam;

pub mod mirbft;
mod state_machine;
mod timer;
mod epoch;
mod bucket;
mod sequence;

pub type NodeID = u64;
pub type BucketID = u64;
pub type SeqNo = u64;
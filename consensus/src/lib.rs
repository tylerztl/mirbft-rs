#[macro_use]
extern crate crossbeam;

mod bucket;
mod epoch;
pub mod mirbft;
mod sequence;
mod state_machine;
mod timer;

pub type NodeID = u64;
pub type BucketID = u64;
pub type SeqNo = u64;

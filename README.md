# MirBFT Library For Rust

MirBFT is a library implementing the [Mir byzantine fault tolerant consensus protocol](https://arxiv.org/abs/1906.05552) in a network transport, storage, and cryptographic algorithm agnostic way.  MirBFT hopes to be a building block of a next generation of distributed systems, providing an implementation of [atomic broadcast](https://en.wikipedia.org/wiki/Atomic_broadcast) which can be utilized by any distributed system.

Mir improves on traditional atomic broadcast protocols like PBFT and Raft which always have a single active leader by allowing concurrent leaders and reconciling total order in a deterministic and provably safe way.  The multi-leader nature of Mir should lead to exceptional performance especially on wide area networks but should be suitable for LAN deployments as well.

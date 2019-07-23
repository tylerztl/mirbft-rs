use network::{
    GrpcServer,
    GrpcClient,
};
use std::{
    thread,
    time,
    sync::Arc,
    collections::HashMap,
};
use logger::prelude::*;
use config::{
    node_config::NodeConfig,
    consensus_peers::load_consensus_peers_config,
};
use crossbeam::crossbeam_channel::unbounded;
use consensus::mirbft::MirBft;
use consensus::timer::BatchTimer;

pub struct Node {
    pub config: NodeConfig,
    pub clients: HashMap<String, Arc<GrpcClient>>,
}

impl Node {
    pub fn new(config: NodeConfig) -> Self {
        Node {
            config,
            clients: setup_client(),
        }
    }

    pub fn run(&self) {
        logger::init();
        let connection_str = format!("{}:{}", self.config.service.address, self.config.service.port);
        let (msg_sender, msg_receiver) = unbounded();

        let mut server = GrpcServer::new(&connection_str, msg_sender);
        thread::spawn(move || {
            server.start();
            info!("mirbft server started on {} ", connection_str);
            loop {
                thread::sleep(time::Duration::from_millis(100));
            }
        });

        let (bft_sender, bft_receiver) = unbounded();
        MirBft::start(bft_sender, msg_receiver, self.config.service.peer_id);

//        let bft_info = bft_receiver.recv().unwrap();
//        info!("{:?}", bft_info);

        BatchTimer::start(self.config.consensus.batch_timeout_ms);

        loop {
            thread::park();
        }
    }
}

fn setup_client() -> HashMap<String, Arc<GrpcClient>> {
    let config = load_consensus_peers_config();
    let mut map = HashMap::new();
    for (key, val) in config.peers.iter() {
        let client = Arc::new(GrpcClient::new(val.address.as_str(), val.port));
        map.insert(key.clone(), client);
    }
    map
}





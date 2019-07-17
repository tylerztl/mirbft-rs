use network::{
    GrpcServer,
    GrpcClient,
};
use std::{
    thread,
    time,
    sync::{
        mpsc,
        Arc,
    },
    collections::HashMap,
};
use logger::prelude::*;
use config::{
    node_config::NodeConfig,
    consensus_peers::load_consensus_peers_config,
};

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
        info!("mirbft server started on {}:{} ", self.config.service.address, self.config.service.port);
        let (start_sender, start_receiver) = mpsc::channel();
        let mut server = GrpcServer::new(&connection_str);
        thread::spawn(move || {
            server.start();
            start_sender.send(()).unwrap();
            loop {
                thread::sleep(time::Duration::from_millis(100));
            }
        });
        start_receiver.recv().unwrap();

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





use config::MirConfig;
use consensus::mirbft::MirBft;
use consensus::NodeID;
use crossbeam::crossbeam_channel::unbounded;
use logger::prelude::*;
use network::{GrpcClient, GrpcServer};
use std::{collections::HashMap, sync::Arc, thread, time};

pub struct Node {
    pub config: MirConfig,
    pub clients: HashMap<NodeID, Arc<GrpcClient>>,
}

impl Node {
    pub fn new() -> Self {
        let config = MirConfig::load();
        let mut clients = HashMap::new();
        for (key, val) in config.consensus_config.peers.iter() {
            let client = Arc::new(GrpcClient::new(val.address.as_str(), val.port));
            let key = key.clone().parse::<u64>().unwrap();
            clients.insert(key, client);
        }
        Node { config, clients }
    }

    pub fn run(&self) {
        logger::init();
        let connection_str = format!(
            "{}:{}",
            self.config.node_config.service.address, self.config.node_config.service.port
        );
        let (msg_sender, msg_receiver) = unbounded();

        let mut server = GrpcServer::new(&connection_str, msg_sender);
        thread::spawn(move || {
            server.start();
            info!("mirbft server started on {} ", connection_str);
            loop {
                thread::sleep(time::Duration::from_millis(100));
            }
        });

        MirBft::start(msg_receiver, self.config.clone());

        loop {
            thread::park();
        }
    }
}

use config::MirConfig;
use consensus::mirbft::MirBft;
use crossbeam::crossbeam_channel::unbounded;
use logger::prelude::*;
use network::GrpcServer;
use std::{thread, time};

pub struct Node {
    pub config: MirConfig,
}

impl Node {
    pub fn new() -> Self {
        Node {
            config: MirConfig::load(),
        }
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

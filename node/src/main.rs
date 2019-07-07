use logger::prelude::*;
use network::GrpcServer;
use std::thread;

fn main() {
    logger::init();

    let addr = "127.0.0.1:8080";
    let mut server = GrpcServer::new(addr);
    server.start();
    info!("mirbft server started on {} ", addr);

    loop {
        thread::park();
    }
}

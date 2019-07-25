use crossbeam::crossbeam_channel::Sender;
use futures::future::Future;
use grpcio::{RpcContext, UnarySink};
use logger::prelude::*;
use proto::proto::{
    ab::{BroadcastResponse, Status},
    ab_grpc::AtomicBroadcast,
    mirbft::Message,
};

#[derive(Clone)]
pub struct BroadcastService {
    msg_sender: Sender<(u64, Message)>,
}

impl BroadcastService {
    pub fn new(msg_sender: Sender<(u64, Message)>) -> BroadcastService {
        BroadcastService { msg_sender }
    }
}

impl AtomicBroadcast for BroadcastService {
    fn broadcast(&mut self, ctx: RpcContext<'_>, req: Message, sink: UnarySink<BroadcastResponse>) {
        let (_key, val) = ctx.request_headers().get(0).unwrap();
        let node_id = std::str::from_utf8(val).unwrap();
        let node_id = node_id.parse::<u64>().unwrap();
        info!("receive broadcast message: {:?} from node {}", req, node_id);

        self.msg_sender.send((node_id, req)).unwrap();

        let mut resp = BroadcastResponse::new();
        resp.set_status(Status::SUCCESS);

        let f = sink
            .success(resp)
            .map_err(|e| error!("failed to handle request: {:?}", e));
        ctx.spawn(f);
    }
}

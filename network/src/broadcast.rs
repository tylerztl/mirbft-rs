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
    msg_sender: Sender<Message>,
}

impl BroadcastService {
    pub fn new(msg_sender: Sender<Message>) -> BroadcastService {
        BroadcastService { msg_sender }
    }
}

impl AtomicBroadcast for BroadcastService {
    fn broadcast(&mut self, ctx: RpcContext<'_>, req: Message, sink: UnarySink<BroadcastResponse>) {
        info!("receive broadcast message: {:?}", req);
        self.msg_sender.send(req).unwrap();

        let mut resp = BroadcastResponse::new();
        resp.set_status(Status::SUCCESS);

        let f = sink
            .success(resp)
            .map_err(|e| error!("failed to handle request: {:?}", e));
        ctx.spawn(f);
    }
}

use proto::proto::{
    mirbft::Message,
    ab_grpc::AtomicBroadcast,
    ab::{BroadcastResponse, Status},
};
use grpcio::{UnarySink, RpcContext};
use futures::future::Future;
use logger::prelude::*;

#[derive(Clone)]
pub struct BroadcastService {}

impl BroadcastService {
    pub fn new() -> BroadcastService {
        BroadcastService {}
    }
}

impl AtomicBroadcast for BroadcastService {
    fn broadcast(
        &mut self,
        ctx: RpcContext<'_>,
        req: Message,
        sink: UnarySink<BroadcastResponse>,
    ) {
        info!("receive broadcast message: {:?}", req);
        let mut resp = BroadcastResponse::new();
        resp.set_status(Status::SUCCESS);

        let f = sink
            .success(resp)
            .map_err(|e| error!("failed to handle request: {:?}", e));
        ctx.spawn(f);
    }
}
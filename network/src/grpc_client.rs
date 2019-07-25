use grpcio::{ChannelBuilder, EnvBuilder, MetadataBuilder};
use logger::prelude::*;
use proto::proto::{ab_grpc::AtomicBroadcastClient, mirbft::Message};
use std::sync::Arc;

pub struct GrpcClient {
    node_id: u64,
    address: String,
    client: AtomicBroadcastClient,
}

impl GrpcClient {
    pub fn new(node_id: u64, host: String, port: u16) -> Self {
        let address = format!("{}:{}", host, port);
        // Create a GRPC client
        let env = Arc::new(EnvBuilder::new().name_prefix("grpc-client-").build());
        let ch = ChannelBuilder::new(env).connect(&address);
        let client = AtomicBroadcastClient::new(ch);

        GrpcClient {
            node_id,
            address,
            client,
        }
    }

    pub fn broadcast(&mut self, source: &str, msg: &Message) {
        let opt = grpcio::CallOption::default();
        let mut builder = MetadataBuilder::new();
        builder.add_str("source", source).unwrap();
        let metadata = builder.build();
        let opt = opt.headers(metadata);
        let ret = self.client.broadcast_opt(msg, opt);
        if ret.is_err() {
            error!(
                "failed broadcast msg to node: {}, address: {}, err: {}",
                self.node_id,
                self.address,
                ret.err().unwrap()
            );
        }
    }
}

#[test]
fn test_client() {
    use proto::proto::mirbft::Proposal;
    let mut client = GrpcClient::new(1, String::from("127.0.0.1"), 8081);
    let mut msg = Message::new();
    let mut proposal = Proposal::new();
    proposal.set_payload(vec![1, 2, 3, 4]);
    msg.set_proposal(proposal);
    for _i in 0..1 {
        client.broadcast("0", &msg);
    }
}

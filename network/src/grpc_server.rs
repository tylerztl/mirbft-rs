use std::sync::Arc;
use std::str::FromStr;
use std::cmp::max;
use std::net::SocketAddr;

use proto::proto::mirbft::Message;
use proto::proto::ab_grpc::create_atomic_broadcast;
use grpcio_sys;
use grpcio::{ChannelBuilder, EnvBuilder, Server, ServerBuilder};
use crossbeam::crossbeam_channel::Sender;

const DEFAULT_GRPC_STREAM_INITIAL_WINDOW_SIZE: i32 = 2 * 1024 * 1024;
const DEFAULT_GRPC_CONCURRENT_STREAM: i32 = 1024;
const MAX_GRPC_MSG_LEN: i32 = 32 * 1024 * 1024;

pub struct GrpcServer {
    server: Server,
}

impl GrpcServer {
    pub fn new(addr: &str, msg_sender: Sender<Message>) -> Self {
        let address: SocketAddr = SocketAddr::from_str(addr).unwrap();

        let env = Arc::new(
            EnvBuilder::new()
                .name_prefix("grpc-server-")
                .cq_count(unsafe { max(grpcio_sys::gpr_cpu_num_cores() as usize / 2, 2) })
                .build(),
        );

        let channel_args = ChannelBuilder::new(Arc::clone(&env))
            .stream_initial_window_size(DEFAULT_GRPC_STREAM_INITIAL_WINDOW_SIZE)
            .max_concurrent_stream(DEFAULT_GRPC_CONCURRENT_STREAM)
            .max_send_message_len(MAX_GRPC_MSG_LEN)
            .max_receive_message_len(MAX_GRPC_MSG_LEN)
            .build_args();

        let service = super::broadcast::BroadcastService::new(msg_sender);
        let server = ServerBuilder::new(Arc::clone(&env))
            .bind(format!("{}", address.ip()), address.port())
            .channel_args(channel_args)
            .register_service(create_atomic_broadcast(service))
            .build()
            .expect("Unable to create grpc server");

        GrpcServer { server }
    }

    pub fn start(&mut self) {
        self.server.start();
    }

    pub fn shutdown(&mut self) {
        self.server.shutdown();
    }
}
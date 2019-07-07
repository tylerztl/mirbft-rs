mod broadcast;
mod grpc_client;
mod grpc_server;

pub type Result<T> = ::std::result::Result<T, failure::Error>;

pub use grpc_server::GrpcServer;

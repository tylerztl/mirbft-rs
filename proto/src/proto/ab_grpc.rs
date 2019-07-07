// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_ATOMIC_BROADCAST_BROADCAST: ::grpcio::Method<super::mirbft::Message, super::ab::BroadcastResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/proto.AtomicBroadcast/Broadcast",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct AtomicBroadcastClient {
    client: ::grpcio::Client,
}

impl AtomicBroadcastClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        AtomicBroadcastClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn broadcast_opt(&self, req: &super::mirbft::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::ab::BroadcastResponse> {
        self.client.unary_call(&METHOD_ATOMIC_BROADCAST_BROADCAST, req, opt)
    }

    pub fn broadcast(&self, req: &super::mirbft::Message) -> ::grpcio::Result<super::ab::BroadcastResponse> {
        self.broadcast_opt(req, ::grpcio::CallOption::default())
    }

    pub fn broadcast_async_opt(&self, req: &super::mirbft::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::ab::BroadcastResponse>> {
        self.client.unary_call_async(&METHOD_ATOMIC_BROADCAST_BROADCAST, req, opt)
    }

    pub fn broadcast_async(&self, req: &super::mirbft::Message) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::ab::BroadcastResponse>> {
        self.broadcast_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait AtomicBroadcast {
    fn broadcast(&mut self, ctx: ::grpcio::RpcContext, req: super::mirbft::Message, sink: ::grpcio::UnarySink<super::ab::BroadcastResponse>);
}

pub fn create_atomic_broadcast<S: AtomicBroadcast + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ATOMIC_BROADCAST_BROADCAST, move |ctx, req, resp| {
        instance.broadcast(ctx, req, resp)
    });
    builder.build()
}

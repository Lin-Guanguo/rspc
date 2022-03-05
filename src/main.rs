use bytes::Bytes;
use prost::Message;
use rspc::{
    example::{HelloServer, HelloServerImpl},
    protocol::frame::{FrameHeader, RequestHeader},
};

include!(concat!(env!("OUT_DIR"), "/rspc.hello.rs"));

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::TRACE)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut msg = HelloRequest::default();
    msg.name = String::from("Lin");
    let msg: Bytes = msg.encode_to_vec().into();

    let service_impl = Box::new(HelloServerImpl { count: 32 });
    let mut service: Box<dyn HelloServer> = service_impl;
    let refer = service.as_ref();
    let f = HelloServer::hello_service;

    let ret = f(refer, msg);

    println!("{:?}", ret);
}

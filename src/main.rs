use bytes::Bytes;
use prost::Message;
use rspc::{
    example::{HelloServer, HelloServerImpl},
    protocol::frame::{FrameHeader, RequestHeader},
    server::{error::ServerError, Server},
};

include!(concat!(env!("OUT_DIR"), "/rspc.hello.rs"));

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::TRACE)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut server = Server::new(8080).await?;
    let s1 = HelloServerImpl::new();
    let s2 = HelloServerImpl::new();
    server.register_service(s1);
    server.register_service(s2);
    println!("{:?}", server.list_service());

    Ok(())
}

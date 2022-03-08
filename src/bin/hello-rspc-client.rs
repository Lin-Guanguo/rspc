use futures::join;
use prost::Message;
use rspc::{
    client::{Channel, ClientError},
    example::{HelloClient, HelloClientImpl},
    protocol::frame::*,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::oneshot,
};

include!(concat!(env!("OUT_DIR"), "/rspc.hello.rs"));

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::TRACE)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let channel = Channel::new("127.0.0.1:8080").await?;
    let client = HelloClientImpl::new(&channel, 0);
    client.hello().await;

    Ok(())
}

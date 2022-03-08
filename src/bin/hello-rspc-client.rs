use futures::{join, try_join};
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

    let mut channel = Channel::new("127.0.0.1:8080").await?;
    let (run, channel) = channel.run();
    let client = HelloClientImpl::new(&channel, 0);
    let client2 = HelloClientImpl::new(&channel, 1);
    join!(
        run,
        client.hello(),
        client.hello(),
        client.hello(),
        client2.hello(),
        client2.hello()
    )
    .0?;

    Ok(())
}

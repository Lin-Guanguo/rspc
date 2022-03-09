use std::cell::Cell;

use async_trait::async_trait;
use futures::join;
use rspc::{
    client::{self, Channel, ClientError, ClientReaderWriter},
    example::HelloClient,
};
use tracing::info;

pub struct HelloClientImpl<'a> {
    channel: &'a client::RunningChannel,
    first_method_id: u32,
}

impl<'a> HelloClientImpl<'a> {
    pub fn new(channel: &'a client::RunningChannel, first_method_id: u32) -> Self {
        Self {
            channel,
            first_method_id,
        }
    }
}

#[async_trait(?Send)]
impl<'a> HelloClient for HelloClientImpl<'a> {
    async fn hello_impl(&self, mut stream: ClientReaderWriter) {
        stream.write("hello".into()).await.unwrap();
        stream.write_last("你好".into()).await.unwrap();
        while let Some(reply) = stream.read().await {
            info!(reply = ?reply)
        }
    }

    fn get_channel(&self) -> &client::RunningChannel {
        &self.channel
    }

    fn get_first_method_id(&self) -> u32 {
        self.first_method_id
    }
}

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

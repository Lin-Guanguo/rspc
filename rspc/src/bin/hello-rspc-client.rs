use futures::join;
use rspc::client::{Channel, ClientError, ClientStub};

#[rspc_macros::rspc_client(hello, stream hello_stream)]
pub struct HelloClient<'a> {
    channel: &'a rspc::client::RunningChannel,
    first_method_id: u32,
}

impl<'a> ClientStub for HelloClient<'a> {
    fn channel(&self) -> &'_ rspc::client::RunningChannel {
        self.channel
    }

    fn first_method_id(&self) -> u32 {
        self.first_method_id
    }
}

impl<'a> HelloClient<'a> {
    pub fn new(channel: &'a rspc::client::RunningChannel, first_method_id: u32) -> Self {
        Self {
            channel,
            first_method_id,
        }
    }

    async fn hello_stream_impl(&self, mut rw: rspc::client::ClientReaderWriter) {
        rw.write("stream hello1".into()).await.unwrap();
        rw.write_last("stream hello2".into()).await.unwrap();
        let r1 = rw.read().await;
        let r2 = rw.read().await;
        let r3 = rw.read().await;
        println!("reply1 {:?}", r1);
        println!("reply2 {:?}", r2);
        println!("reply3 {:?}", r3);
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

    let channel = Channel::new("127.0.0.1:8080").await?;
    let (run, channel) = channel.run();
    let client = HelloClient::new(&channel, 0);
    let client2 = HelloClient::new(&channel, 2);

    let f1 = client2.hello_stream();
    let f2 = async {
        let t = client.hello("hello".into()).await;
        println!("normal reply {:?}", t)
    };
    let f3 = async {
        let t = client2.hello("hello".into()).await;
        println!("normal reply {:?}", t)
    };

    join!(run, f1, f2, f3).0?;
    Ok(())
}

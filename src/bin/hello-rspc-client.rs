use futures::join;
use prost::Message;
use rspc::protocol::RequestFrame;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::oneshot,
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
    msg.name = String::from("hello");

    let msg = msg.encode_to_vec();

    let chan = rspc::client::channel::Channel::new("127.0.0.1:8080")
        .await
        .unwrap();
    let (f, w) = chan.run();

    let r = join!(f, async move {
        let (back_tx, back_rx) = oneshot::channel();
        w.write(1, msg.into(), back_tx).await.unwrap();
        let reply = back_rx.await.unwrap();
        println!("{:?}", reply)
    });
    println!("{:?}", r)
}

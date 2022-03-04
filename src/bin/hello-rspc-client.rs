use futures::join;
use prost::Message;
use rspc::protocol::RequestMsg;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::oneshot,
};

include!(concat!(env!("OUT_DIR"), "/rspc.hello.rs"));

#[tokio::main]
async fn main() {
    let mut msg = HelloRequest::default();
    msg.name = String::from("hello");

    let msg = msg.encode_to_vec();
    let header = rspc::protocol::RequestHeader::new(54, 1, msg.len() as u32);
    let msg = RequestMsg {
        header,
        body: msg.into(),
    };

    let chan = rspc::client::channel::Channel::new("127.0.0.1:8080")
        .await
        .unwrap();
    let (f, w) = chan.run();

    let r = join!(f, async move {
        let (back_tx, back_rx) = oneshot::channel();
        w.write(msg, back_tx).await.unwrap();
        let reply = back_rx.await.unwrap();
        println!("{:?}", reply)
    });
    println!("{:?}", r)
}

use prost::Message;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

include!(concat!(env!("OUT_DIR"), "/rspc.hello.rs"));

#[tokio::main]
async fn main() {
    let mut channel = rspc::client::ClientChannel::new("127.0.0.1:8080").await;
    let mut msg = HelloRequest::default();
    msg.name = String::from("hello");

    let msg = msg.encode_to_vec();
    println!("{:?}", msg);
    let request = rspc::protocol::RequestMsg::new(54, 1, msg.len() as u32, msg);
    println!("{:?}", request);
    let header = request.encode_header();

    if let Ok(mut channel) = channel {
        let r = channel.tcp.write_all(&header).await;
        let r = channel.tcp.write_all(&request.msg_body).await;
        println!("write return {:?}", r)
    }

    let tbuf = vec![0u8; 0];
    let t = HelloRequest::decode(&tbuf[..]);
    println!("{:?}", t)
}

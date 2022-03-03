include!(concat!(env!("OUT_DIR"), "/rspc.hello.rs"));

#[tokio::main]
async fn main() {
    let request = HelloRequest::default();
    let reply = HelloReply::default();
    println!("{:?}", request);
    println!("{:?}", reply);
}

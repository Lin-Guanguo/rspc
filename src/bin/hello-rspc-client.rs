#[tokio::main]
async fn main() {
    let channel = rspc::client::ClientChannel::new("127.0.0.1:8080").await;
    println!("{:?}", channel);
}

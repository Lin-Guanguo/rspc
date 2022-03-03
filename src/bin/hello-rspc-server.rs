#[tokio::main]
async fn main() {
    let server = rspc::server::Server::new(8080);
    let r = server.run().await;
    println!("{:?}", r);
}

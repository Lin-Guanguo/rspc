#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::TRACE)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // let mut server = rspc::server::Server::new(8080);
    // server.register_service(1, rspc::example::hello_service);
    // let r = server.run().await;
    // println!("{:?}", r);
}

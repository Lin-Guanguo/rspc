use bytes::Bytes;
use prost::Message;
use rspc::{
    example::{HelloServer, HelloServerImpl},
    protocol::frame::{FrameHeader, RequestHeader},
    server::{error::ServerError, Server},
};
use tokio::task;

include!(concat!(env!("OUT_DIR"), "/rspc.hello.rs"));

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::TRACE)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut server = Server::new(8080).await?;
    let s1 = HelloServerImpl::new();
    let s2 = HelloServerImpl::new();
    server.register_service(s1);
    server.register_service(s2);
    println!("{:?}", server.list_service());

    let local = task::LocalSet::new();
    local
        .run_until(async move {
            let mut server = server;
            loop {
                let c = server.accept().await?;
                let _ = task::spawn_local(async move {
                    let mut c = c;
                    c.run().await
                });
            }
            Result::<(), ServerError>::Ok(())
        })
        .await?;

    Ok(())
}

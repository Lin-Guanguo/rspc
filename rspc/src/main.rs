use bytes::Bytes;
use prost::Message;
use rspc::server::ServerError;
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

    Ok(())
}

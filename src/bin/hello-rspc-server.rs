use std::cell::Cell;

use async_trait::async_trait;
use rspc::{
    example::HelloServer,
    server::{service::ServerReaderWriter, Server, ServerError},
};
use tokio::task;

pub struct HelloServerImpl {
    share_states: Cell<i32>,
}

#[async_trait(?Send)]
impl HelloServer for HelloServerImpl {
    async fn hello(&self, stream: ServerReaderWriter) {
        let t = self.share_states.get();
        self.share_states.set(t + 1);
        tokio::spawn(async move {
            let mut stream = stream;
            while let Some(r) = stream.read().await {
                stream.write(0, r).await.unwrap();
            }
            stream
                .write_last(0, format!("id={} end", t).into())
                .await
                .unwrap();
        });
    }
}

impl HelloServerImpl {
    pub fn new() -> Self {
        Self {
            share_states: Cell::new(1),
        }
    }
}

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

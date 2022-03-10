use std::cell::Cell;

use async_trait::async_trait;
use rspc::server::{service::ServerReaderWriter, Server, ServerError};
use tokio::task;

// macros generate template
pub struct HelloServer {
    share_states: Cell<i32>,
}

#[async_trait::async_trait(?Send)]
impl rspc::server::Service for HelloServer {
    async fn call_method(
        &self,
        fn_n: u32,
        mut stream: rspc::server::ServerReaderWriter,
    ) -> Result<(), rspc::server::ServerError> {
        if fn_n < 1 {
            if let Some(request) = stream.read().await {
                let reply = match fn_n {
                    0 => self.hello(request).await,
                    _ => return Err(rspc::server::ServerError::NormalRpcMethodError()),
                };

                stream.write(reply.0, reply.1).await?;
                Ok(())
            } else {
                Err(rspc::server::ServerError::NormalRpcMethodError())
            }
        } else {
            match fn_n {
                1 => self.hello_stream(stream).await,
                _ => Err(rspc::server::ServerError::StreamRpcMethodError()),
            }
        }
    }

    fn service_name(&self) -> &'static str {
        "HelloServer"
    }

    fn methods_name(&self) -> &'static [&'static str] {
        &["hello", "hello_stream"]
    }

    fn methods_len(&self) -> usize {
        2
    }
}

impl HelloServer {
    pub fn new() -> Self {
        Self {
            share_states: Cell::default(),
        }
    }

    async fn hello(&self, request: bytes::Bytes) -> (u32, bytes::Bytes) {
        println!("read request {:?}", request);
        let count = self.share_states.get();
        self.share_states.set(count + 1);
        (0, format!("{} hello reply", count).into())
    }

    async fn hello_stream(
        &self,
        mut stream: rspc::server::ServerReaderWriter,
    ) -> Result<(), rspc::server::ServerError> {
        while let Some(r) = stream.read().await {
            println!("read request {:?}", r);
            stream.write(0, r).await?;
        }
        let count = self.share_states.get();
        self.share_states.set(count + 1);
        stream
            .write_last(0, format!("{} stream end", count).into())
            .await?;
        Ok(())
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
    let s1 = HelloServer::new();
    let s2 = HelloServer::new();
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

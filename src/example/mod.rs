use std::sync::Arc;

use async_trait::async_trait;

use crate::server::service::{ServerReaderWriter, Service};

#[async_trait]
pub trait HelloServer: Sync + Send {
    const METHOD_NAMES: [&'static str; 1] = ["hello"];

    async fn hello(&self, stream: ServerReaderWriter);
}

#[async_trait]
impl<S: HelloServer> Service for S {
    async fn call_method(&self, fn_n: usize, stream: ServerReaderWriter) {
        match fn_n {
            0 => self.hello(stream).await,
            n => panic!("error method id {}", n),
        }
    }

    fn method_names(&self) -> &'static [&'static str] {
        &Self::METHOD_NAMES
    }

    fn num_of_methods(&self) -> usize {
        1
    }
}

pub struct HelloServerImpl {}

#[async_trait]
impl HelloServer for HelloServerImpl {
    async fn hello(&self, stream: ServerReaderWriter) {
        tokio::spawn(async move {
            let mut stream = stream;
            while let Some(r) = stream.read().await {
                stream.write(0, r).await.unwrap();
            }
            stream
                .write_last(0, String::from("end").into())
                .await
                .unwrap();
        });
    }
}

pub struct HelloClient {}

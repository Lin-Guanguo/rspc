use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};

use async_trait::async_trait;

use crate::server::service::{ServerReaderWriter, Service};

#[async_trait(?Send)]
pub trait HelloServer {
    const METHOD_NAMES: [&'static str; 1] = ["hello"];
    const SERVICE_NAME: &'static str = "HelloServer";

    async fn hello(&self, stream: ServerReaderWriter);
}

#[async_trait(?Send)]
impl<S: HelloServer> Service for S {
    async fn call_method(&self, fn_n: u32, stream: ServerReaderWriter) {
        match fn_n {
            0 => self.hello(stream).await,
            n => panic!("error method id {}", n),
        }
    }

    fn service_name(&self) -> &'static str {
        &Self::SERVICE_NAME
    }

    fn method_names(&self) -> &'static [&'static str] {
        &Self::METHOD_NAMES
    }

    fn num_of_methods(&self) -> usize {
        1
    }
}

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

pub struct HelloClient {}

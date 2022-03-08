use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};

use async_trait::async_trait;

use crate::{
    client::{self, service::ClientReaderWriter},
    server::service::{ServerReaderWriter, Service},
};

// should be generate
#[async_trait(?Send)]
pub trait HelloServer {
    const METHOD_NAMES: [&'static str; 1] = ["hello"];
    const SERVICE_NAME: &'static str = "HelloServer";

    async fn hello(&self, stream: ServerReaderWriter);
}

// should be generate
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

// user implement
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

// should be generate
#[async_trait]
pub trait HelloClient {
    async fn hello_impl(&self, stream: ClientReaderWriter);

    fn get_channel(&self) -> &'_ client::Channel;

    fn get_first_method_id(&self) -> u32;

    async fn hello(&self) {
        let c = self.get_channel();
        let m = self.get_first_method_id();
        let rw = c.call_method(m);
        self.hello_impl(rw).await
    }
}

// user implement
pub struct HelloClientImpl {
    channel: client::Channel,
    first_method_id: u32,
}

impl HelloClientImpl {
    pub fn new(channel: client::Channel, first_method_id: u32) -> Self {
        Self {
            channel,
            first_method_id,
        }
    }
}

#[async_trait]
impl HelloClient for HelloClientImpl {
    async fn hello_impl(&self, stream: ClientReaderWriter) {
        todo!()
    }

    fn get_channel(&self) -> &'_ client::Channel {
        &self.channel
    }

    fn get_first_method_id(&self) -> u32 {
        self.first_method_id
    }
}

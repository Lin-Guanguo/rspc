use std::{
    cell::{Cell, RefCell},
    sync::Arc,
};

use async_trait::async_trait;
use tracing::info;

use crate::{
    client::{self, ClientReaderWriter},
    server::service::{ServerReaderWriter, Service},
};

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

#[async_trait(?Send)]
pub trait HelloClient {
    async fn hello_impl(&self, stream: ClientReaderWriter);

    fn get_channel(&self) -> &client::RunningChannel;

    fn get_first_method_id(&self) -> u32;

    async fn hello(&self) {
        let c = self.get_channel();
        let m = self.get_first_method_id();
        let rw = c.call_method(m);
        self.hello_impl(rw).await
    }
}

mod channel;
mod error;
mod service;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use error::ServerError;
use tokio::{io::AsyncReadExt, net};
use tracing::{debug, info, span, trace, Level};

use crate::protocol;

use self::service::*;

#[derive(Debug)]
pub struct Server {
    port: i32,
    service_table: ServiceTable,
}

impl Server {
    pub fn new(port: i32) -> Server {
        Server {
            port,
            service_table: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register_service(&mut self, method_id: u32, service_fn: ServiceFn) {
        let mut map = (*self.service_table).write().unwrap();
        map.insert(method_id, service_fn);
    }

    #[tracing::instrument(name = "server", skip_all)]
    pub async fn run(self) -> Result<(), ServerError> {
        let listen = net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        info!("listening on port {}", self.port);

        loop {
            let (tcp, addr) = listen.accept().await?;
            info!("accept from {}", addr);
            let service_table = self.service_table.clone();
            let _ = tokio::spawn(async move {
                let channel = channel::Channel::new(tcp, service_table);
                let channel_ret = channel.handle_channel().await;
            });
        }

        Ok(())
    }
}

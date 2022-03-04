mod channel;
mod error;
mod service;

use tokio::net::{self, TcpListener};
use tracing::{info, instrument};

use crate::server::channel::Channel;

use self::{
    error::ServerError,
    service::{ServiceFn, ServiceTable},
};

pub struct Server {
    port: i32,
    service_table: ServiceTable,
}

impl Server {
    pub fn new(port: i32) -> Server {
        Server {
            port,
            service_table: ServiceTable::new(),
        }
    }

    pub fn register_service(&mut self, method_id: u32, service_fn: ServiceFn) {
        self.service_table.insert(method_id, service_fn);
    }

    #[instrument(name = "server", skip_all)]
    pub async fn run(self) -> Result<(), ServerError> {
        let listen = TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        info!("listening on port {}", self.port);

        loop {
            let (tcp, addr) = listen.accept().await?;
            info!("accept from {}", addr);
            let service_table = self.service_table.clone();
            let _ = tokio::spawn(async move {
                let channel = Channel::new(tcp, service_table);
                let channel_ret = channel.handle_channel().await;
            });
        }

        Ok(())
    }
}

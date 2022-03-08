use std::sync::{Arc, RwLock};

use tokio::net::TcpListener;
use tracing::info;

use self::service::ServiceTable;

pub mod channel;
pub mod error;
pub mod service;

pub use channel::Channel;
pub use error::ServerError;
pub use service::Service;

pub struct Server {
    listener: TcpListener,
    service_table: Arc<RwLock<ServiceTable>>,
}

impl Server {
    pub async fn new(port: u32) -> Result<Self, ServerError> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        Ok(Self {
            listener,
            service_table: Arc::new(RwLock::new(ServiceTable::new())),
        })
    }

    pub fn register_service<S: 'static + Service>(&mut self, service: S) {
        self.service_table
            .write()
            .expect("Service Table RwLock write error")
            .register_service(service);
    }

    pub fn list_service(&self) -> Vec<(&'static str, &'static str)> {
        self.service_table
            .read()
            .expect("Service Table RwLock write error")
            .list_service()
    }

    pub async fn accept(&mut self) -> Result<Channel, ServerError> {
        let (stream, addr) = self.listener.accept().await?;
        info!("accept connection {:?}", addr);
        Ok(Channel::new(stream, self.service_table.clone()))
    }
}

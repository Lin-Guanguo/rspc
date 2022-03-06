use tokio::net::{TcpListener, TcpStream};
use tracing::info;

use crate::server::channel::Channel;

use self::error::ServerError;

pub mod channel;
pub mod error;
#[allow(dead_code)]
pub mod service;

struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn new(port: u32) -> Result<Self, ServerError> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        Ok(Self { listener })
    }

    pub async fn accept(&mut self) -> Result<Channel, ServerError> {
        let (stream, addr) = self.listener.accept().await?;
        info!("accept connection {:?}", addr);
        Ok(Channel::new(stream))
    }
}

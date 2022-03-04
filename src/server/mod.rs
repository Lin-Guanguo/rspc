mod channel;
mod error;

use error::ServerError;
use tokio::{io::AsyncReadExt, net};
use tracing::{debug, info, span, trace, Level};

use crate::protocol;

#[derive(Debug)]
pub struct Server {
    port: i32,
}

impl Server {
    pub fn new(port: i32) -> Server {
        Server { port }
    }

    #[tracing::instrument(name = "server", skip_all)]
    pub async fn run(&self) -> Result<(), ServerError> {
        let listen = net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        info!("listening on port {}", self.port);

        loop {
            let (tcp, addr) = listen.accept().await?;
            info!("accept from {}", addr);

            let _ = tokio::spawn(async move {
                let mut channel = channel::Channel::new(tcp);
                let channel_ret = channel.handle_channel().await;
            });
        }

        Ok(())
    }
}

mod error;

use error::ServerError;
use tokio::net;

#[derive(Debug)]
pub struct Server {
    port: i32,
}

impl Server {
    pub fn new(port: i32) -> Server {
        Server { port }
    }

    pub async fn run(&self) -> Result<(), ServerError> {
        let listen = net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;

        loop {
            let (tcp, addr) = listen.accept().await?;
            println!("accept from {}", addr);

            let _ = tokio::spawn(async move {
                let result = Server::handle_channel(tcp).await;
                println!("disconnect from {}, result {:?}", addr, result);
            });
        }

        Ok(())
    }

    async fn handle_channel(tcp: net::TcpStream) -> Result<(), ServerError> {
        Ok(())
    }
}

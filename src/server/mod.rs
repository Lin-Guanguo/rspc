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
                let result = Server::handle_channel(tcp).await;
                println!("disconnect from {}, result {:?}", addr, result);
            });
        }

        Ok(())
    }

    #[tracing::instrument(name = "channel", skip_all, fields(peer = ?tcp.peer_addr().unwrap()))]
    async fn handle_channel(tcp: net::TcpStream) -> Result<(), ServerError> {
        let mut tcp = tcp;
        let mut header_buf = [0u8; protocol::REQUEST_HEADER_BYTES];
        tcp.read_exact(&mut header_buf).await?;
        let mut request = protocol::RequestHeader::decode(&header_buf);
        let mut bytesMut = bytes::BytesMut::with_capacity(request.body_len as usize);
        tcp.read_exact(&mut bytesMut).await?;

        debug!("channel income request {:?}, {:?}", request, bytesMut);
        Ok(())
    }
}

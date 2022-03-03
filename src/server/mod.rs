mod error;

use error::ServerError;
use tokio::{io::AsyncReadExt, net};

use crate::protocol;

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
        let mut tcp = tcp;
        let mut header_buf = [0u8; protocol::REQUEST_HEADER_BYTES];
        tcp.read_exact(&mut header_buf).await?;
        let mut request = protocol::RequestMsg::decode_header(&header_buf);
        request.msg_body = vec![0; request.body_len as usize];
        tcp.read_exact(&mut request.msg_body[0..]).await?;

        println!("server read request body: {:?}", request);
        Ok(())
    }
}

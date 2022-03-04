pub mod error;

use tokio::net::{TcpStream, ToSocketAddrs};

use self::error::ClientError;

#[derive(Debug)]
pub struct ClientChannel {
    // TODO: remove pub
    pub tcp: TcpStream,
}

impl ClientChannel {
    pub async fn new<A: ToSocketAddrs>(addr: A) -> Result<ClientChannel, ClientError> {
        let tcp = TcpStream::connect(addr).await?;
        println!("connect");
        Ok(ClientChannel { tcp })
    }
}

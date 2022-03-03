mod error;

use error::ClientError;
use tokio::net;

#[derive(Debug)]
pub struct ClientChannel {
    // TODO: remove pub
    pub tcp: net::TcpStream,
}

impl ClientChannel {
    pub async fn new<A: net::ToSocketAddrs>(addr: A) -> Result<ClientChannel, ClientError> {
        let tcp = net::TcpStream::connect(addr).await?;
        println!("connect");
        Ok(ClientChannel { tcp })
    }
}

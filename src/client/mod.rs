use tokio::net::{TcpStream, ToSocketAddrs};

// pub mod channel;
pub mod error;
pub mod service;

pub use error::ClientError;
pub use service::ClientReaderWriter;

const REQUEST_BUF_N: usize = 32;
const REQUEST_ID_START: u64 = 128;

pub struct Channel {
    tcp: TcpStream,
}

impl Channel {
    pub async fn new<A>(addr: A) -> Result<Self, ClientError>
    where
        A: ToSocketAddrs,
    {
        let tcp = TcpStream::connect(addr).await?;
        Ok(Self { tcp })
    }

    pub fn call_method(&self, fn_n: u32) -> ClientReaderWriter {
        todo!()
    }
}

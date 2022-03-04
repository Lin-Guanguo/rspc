use super::error::ServerError;
use crate::protocol::*;
use bytes::BytesMut;
use tokio::{io::AsyncReadExt, net};
use tracing::{debug, info, instrument, span, trace, Level};

pub struct Channel {
    tcp: net::TcpStream,
}

impl Channel {
    pub fn new(tcp: net::TcpStream) -> Self {
        Self { tcp }
    }

    #[instrument(name="channel", skip_all, fields(peer=?self.tcp.peer_addr().unwrap()))]
    pub async fn handle_channel(&mut self) -> Result<(), ServerError> {
        let mut header_buf = [0u8; REQUEST_HEADER_BYTES];
        loop {
            let _ = self.tcp.read_exact(&mut header_buf).await?;
            let header = RequestHeader::decode(&header_buf);
            let mut msg_body = BytesMut::with_capacity(header.body_len as usize);
            self.tcp.read_exact(&mut msg_body).await?;

            debug!("Received reqeust {:?}", header);
        }
    }
}

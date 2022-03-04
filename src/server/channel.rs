use super::error::ServerError;
use crate::protocol::*;
use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net,
    sync::mpsc,
    try_join,
};
use tracing::{debug, info, instrument, span, trace, Level};

const REPLY_BUF_SIZE: usize = 32;

pub struct Channel {
    tcp: net::TcpStream,
}

impl Channel {
    pub fn new(tcp: net::TcpStream) -> Self {
        Self { tcp }
    }

    #[instrument(name="channel", skip_all, fields(peer=?self.tcp.peer_addr().unwrap()))]
    pub async fn handle_channel(&mut self) -> Result<(), ServerError> {
        let (read_half, write_half) = self.tcp.split();
        let (reply_tx, reply_rx) = mpsc::channel(REPLY_BUF_SIZE);

        let read_handler = async move {
            let mut read_half = read_half;
            let reply_tx = reply_tx;
            let mut header_buf = [0u8; REQUEST_HEADER_BYTES];
            loop {
                read_half.read_exact(&mut header_buf).await?;
                let header = RequestHeader::decode(&header_buf);
                let mut body = BytesMut::with_capacity(header.body_len as usize);
                read_half.read_exact(&mut body).await?;
                debug!("Received reqeust {:?}", header);

                let request = RequestMsg {
                    header,
                    body: body.into(),
                };

                tokio::spawn(Self::handle_request(request, reply_tx.clone()));
            }
            // TODO: special request id to exit
            Result::<(), ServerError>::Ok(())
        };

        let write_handler = async move {
            let mut write_half = write_half;
            let mut reply_rx = reply_rx;
            while let Some(reply) = reply_rx.recv().await {
                write_half.write_all(&reply.header.encode()).await?;
                write_half.write_all(&reply.body).await?;
            }
            Result::<(), ServerError>::Ok(())
        };

        let _ = try_join!(read_handler, write_handler)?;
        Ok(())
    }

    pub async fn handle_request(request: RequestMsg, reply_tx: mpsc::Sender<ReplyMsg>) {}
}

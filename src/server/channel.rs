use super::{error::ServerError, service::*};
use crate::protocol::*;
use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc,
    try_join,
};
use tracing::{debug, instrument};

const REPLY_BUF_SIZE: usize = 32;

pub struct Channel {
    tcp: TcpStream,
    service_table: ServiceTable,
}

impl Channel {
    pub fn new(tcp: TcpStream, service_table: ServiceTable) -> Self {
        Self { tcp, service_table }
    }

    #[instrument(name="channel", skip_all, fields(peer=?self.tcp.peer_addr().unwrap()))]
    pub async fn handle_channel(self) -> Result<(), ServerError> {
        let Self {
            mut tcp,
            service_table,
        } = self;

        let (read_half, write_half) = tcp.split();
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

                tokio::spawn(Self::handle_request(
                    request,
                    reply_tx.clone(),
                    service_table.clone(),
                ));
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

    #[instrument(name="request", skip_all, fields(reuqest_id=?request.header.request_id, method_id=?request.header.method_id))]
    pub async fn handle_request(
        request: RequestMsg,
        reply_tx: mpsc::Sender<ReplyMsg>,
        service_table: ServiceTable,
    ) {
        let service_fn = service_table.get(request.header.method_id);

        debug!("call fn {:?}", service_fn);
        if let Some(service_fn) = service_fn {
            let ret = service_fn(request.body);
            let reply = ReplyMsg {
                header: ReplyHeader::new(request.header.request_id, ret.0, ret.1.len() as u32),
                body: ret.1,
            };
            reply_tx.send(reply).await; // TODO: error check
        } else {
            todo!()
        }
    }
}

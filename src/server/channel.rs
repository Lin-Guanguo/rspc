use super::{error::ServerError, service::*, REPLY_BUF_N};
use crate::protocol::*;
use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc,
    try_join,
};
use tracing::{debug, instrument};

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
        let (reply_tx, reply_rx) = mpsc::channel(REPLY_BUF_N);

        let read_handler = async move {
            let mut read_half = read_half;
            let reply_tx = reply_tx;
            let mut header_buf = [0u8; REQUEST_FRAME_HEADER_LEN];
            loop {
                read_half.read_exact(&mut header_buf).await?;
                let header = RequestHeader::decode(&header_buf[..]).unwrap();
                let mut body = vec![0u8; header.body_len as usize];
                let read_n = read_half.read(&mut body).await?;
                assert_eq!(read_n, header.body_len as usize);

                let request = RequestFrame {
                    header,
                    body: body.into(),
                };

                debug!("Received reqeust {:?}", request.header);

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
                write_half
                    .write_all(&reply.header.encode_to_array())
                    .await?;
                write_half.write_all(&reply.body).await?;
            }
            Result::<(), ServerError>::Ok(())
        };

        let _ = try_join!(read_handler, write_handler)?;
        Ok(())
    }

    #[instrument(name="request", skip_all, fields(reuqest_id=?request.header.request_id, method_id=?request.header.method_id))]
    async fn handle_request(
        request: RequestFrame,
        reply_tx: mpsc::Sender<ReplyFrame>,
        service_table: ServiceTable,
    ) {
        let service_fn = service_table.get(request.header.method_id);

        if let Some(service_fn) = service_fn {
            let ret = service_fn(request.body);
            let reply = ReplyFrame {
                header: ReplyHeader {
                    request_id: request.header.request_id,
                    status_code: ret.0,
                    body_len: ret.1.len() as u32,
                },
                body: ret.1,
            };

            debug!("call fn {:?} reply: {:?}", service_fn, reply.header);

            reply_tx.send(reply).await; // TODO: error check
        } else {
            todo!()
        }
    }
}

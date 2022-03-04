use std::{cell::RefCell, collections::HashMap};

use bytes::BytesMut;
use futures::{future, join, Future};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
    sync::{mpsc, oneshot},
};
use tracing::info;

use crate::protocol::{ReplyHeader, ReplyMsg, RequestMsg, REPLY_HEADER_BYTES};

use super::error::ClientError;

const REQUEST_BUF_SIZE: usize = 32;

pub struct Channel {
    tcp: TcpStream,
}

#[derive(Debug)]
pub struct ChannelRequest(RequestMsg, oneshot::Sender<ReplyMsg>);

#[derive(Clone)]
pub struct ChannelWriter {
    write_tx: mpsc::Sender<ChannelRequest>,
}

impl Channel {
    pub async fn new<A: ToSocketAddrs>(addr: A) -> Result<Self, ClientError> {
        let tcp = TcpStream::connect(addr).await?;
        info!("channel connect {}", tcp.peer_addr().unwrap());
        Ok(Channel { tcp })
    }

    pub fn run(
        self,
    ) -> (
        impl Future<Output = (Result<(), ClientError>, Result<(), ClientError>)>,
        ChannelWriter,
    ) {
        let (write_tx, mut write_rx) = mpsc::channel::<ChannelRequest>(REQUEST_BUF_SIZE);

        let mut tcp = self.tcp;
        let run_future = async move {
            let (mut tcp_read, mut tcp_write) = tcp.split();

            let record = RefCell::new(HashMap::<u64, oneshot::Sender<ReplyMsg>>::new());
            let record1 = &record;
            let record2 = &record;

            let read = async move {
                let mut header_buf = [0u8; REPLY_HEADER_BYTES];
                loop {
                    tcp_read.read_exact(&mut header_buf).await?;
                    let header = ReplyHeader::decode(&header_buf);
                    let mut body = BytesMut::with_capacity(header.body_len as usize);
                    tcp_read.read_exact(&mut body).await?;
                    let msg = ReplyMsg {
                        header,
                        body: body.into(),
                    };

                    let mut record = record1.borrow_mut();
                    if let Some(chan) = record.remove(&msg.header.request_id) {
                        chan.send(msg).expect("oneshot chan send error");
                    } else {
                        todo!()
                    }
                }
                Result::<(), ClientError>::Ok(())
            };

            let write = async move {
                while let Some(ChannelRequest(request, back)) = write_rx.recv().await {
                    // TODO: use once write
                    tcp_write.write_all(&request.header.encode()).await?;
                    tcp_write.write_all(&request.body).await?;

                    let mut record = record2.borrow_mut();
                    record.insert(request.header.request_id, back);
                }
                Result::<(), ClientError>::Ok(())
            };

            let r = join!(read, write);
            info!("channel running over");
            r
        };

        let writer = ChannelWriter { write_tx };
        (run_future, writer)
    }
}

impl ChannelWriter {
    pub async fn write(
        &self,
        request: RequestMsg,
        back: oneshot::Sender<ReplyMsg>,
    ) -> Result<(), ClientError> {
        self.write_tx.send(ChannelRequest(request, back)).await?;
        Ok(())
    }
}

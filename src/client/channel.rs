use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{atomic::AtomicU64, Arc},
};

use bytes::{Bytes, BytesMut};
use futures::{future, join, Future};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
    sync::{mpsc, oneshot},
};
use tracing::{debug, info};

use crate::protocol::{
    ReplyFrame, ReplyHeader, RequestFrame, RequestHeader, REPLY_FRAME_HEADER_LEN,
};

use super::{error::ClientError, REQUEST_BUF_N, REQUEST_ID_START};

pub struct Channel {
    tcp: TcpStream,
}

#[derive(Debug)]
pub struct ChannelRequest(RequestFrame, oneshot::Sender<ReplyFrame>);

#[derive(Clone)]
pub struct ChannelWriter {
    next_request_id: Arc<AtomicU64>,
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
        let (write_tx, mut write_rx) = mpsc::channel::<ChannelRequest>(REQUEST_BUF_N);

        let mut tcp = self.tcp;
        let run_future = async move {
            let (mut tcp_read, mut tcp_write) = tcp.split();

            let record = RefCell::new(HashMap::<u64, oneshot::Sender<ReplyFrame>>::new());
            let record1 = &record;
            let record2 = &record;

            let read = async move {
                let mut header_buf = [0u8; REPLY_FRAME_HEADER_LEN];
                loop {
                    tcp_read.read_exact(&mut header_buf).await?;
                    let header = ReplyHeader::decode(&header_buf);
                    let mut body = vec![0u8; header.body_len as usize];
                    let read_n = tcp_read.read_exact(&mut body).await?;
                    assert_eq!(read_n, header.body_len as usize);

                    let msg = ReplyFrame {
                        header,
                        body: body.into(),
                    };

                    debug!("channel read {:?}", msg.header);

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

                    debug!("channel write {:?}", request.header);

                    let mut record = record2.borrow_mut();
                    record.insert(request.header.request_id, back);
                }
                Result::<(), ClientError>::Ok(())
            };

            let r = join!(read, write);
            info!("channel running over");
            r
        };

        let writer = ChannelWriter {
            next_request_id: Arc::new(AtomicU64::new(REQUEST_ID_START)),
            write_tx,
        };
        (run_future, writer)
    }
}

impl ChannelWriter {
    pub async fn write(
        &self,
        method_id: u32,
        msg: Bytes,
        back: oneshot::Sender<ReplyFrame>,
    ) -> Result<(), ClientError> {
        let request_id = self
            .next_request_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let request = RequestFrame {
            header: RequestHeader {
                request_id,
                method_id,
                body_len: msg.len() as u32,
            },
            body: msg,
        };
        self.write_tx.send(ChannelRequest(request, back)).await?;
        Ok(())
    }
}

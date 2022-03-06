use std::{cell::RefCell, collections::HashMap};

use bytes::Bytes;
use tokio::{io::AsyncReadExt, net::TcpStream, sync::mpsc};

use crate::protocol::frame::{FrameHeader, RequestFrame, RequestHeader, REQUEST_FRAME_HEADER_LEN};

use super::{error::ServerError, service::WriteInfo};

const CHANNEL_REPLY_BUF_SIZE: usize = 32;

pub struct Channel {
    stream: TcpStream,
}

impl Channel {
    pub fn new(stream: TcpStream) -> Self {
        Channel { stream }
    }

    pub async fn init(&mut self) {
        todo!()
    }

    pub async fn run(self) {
        let (tcp_reader, tcp_writer) = self.stream.into_split();
        let working: RefCell<HashMap<u32, mpsc::Sender<Bytes>>> = RefCell::default();
        let (reply_tx, reply_rx) = mpsc::channel(CHANNEL_REPLY_BUF_SIZE);

        let reader = Self::channel_reader(tcp_reader, &working, reply_tx);
        let writer = Self::channel_writer(tcp_writer, reply_rx);

        let ret = futures::join!(reader, writer);
    }

    async fn channel_reader(
        mut tcp_reader: tokio::net::tcp::OwnedReadHalf,
        working: &RefCell<HashMap<u32, mpsc::Sender<Bytes>>>,
        reply_tx: mpsc::Sender<WriteInfo>,
    ) -> Result<(), ServerError> {
        let mut header_buf = [0u8; REQUEST_FRAME_HEADER_LEN];
        loop {
            tcp_reader.read_exact(&mut header_buf).await?;
            let header = RequestHeader::decode(&header_buf[..])?;
            let mut body = vec![0u8; header.body_len as usize];
            tcp_reader.read_exact(&mut body).await?;

            let mut working = working.borrow_mut();
            if let Some(chan) = working.get(&header.request_id) {
                if !header.flag.is_signal() {
                    chan.send(body.into());
                }
                if header.flag.is_eos() {
                    working.remove(&header.request_id);
                }
            } else {
                todo!()
            }
        }
    }

    async fn channel_writer(
        mut tcp_writer: tokio::net::tcp::OwnedWriteHalf,
        reply_rx: mpsc::Receiver<WriteInfo>,
    ) -> Result<(), ServerError> {
        todo!()
    }
}

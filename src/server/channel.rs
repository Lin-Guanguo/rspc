use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use bytes::Bytes;
use futures::lock::Mutex;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{tcp, TcpStream},
    sync::mpsc,
};

use crate::{
    protocol::frame::{FrameHeader, RequestFrame, RequestHeader, REQUEST_FRAME_HEADER_LEN},
    server::service::ServerReaderWriter,
};

use super::{
    error::ServerError,
    service::{Service, WriteInfo},
};

const CHANNEL_REPLY_BUF_SIZE: usize = 32;
const CHANNEL_REQUEST_BUF_SIZE: usize = 32;

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

    pub async fn run(&mut self) -> Result<(), ServerError> {
        let (tcp_reader, tcp_writer) = self.stream.split();
        let tcp_reader = BufReader::new(tcp_reader);
        let tcp_writer = BufWriter::new(tcp_writer);

        let working: RefCell<HashMap<u32, mpsc::Sender<Bytes>>> = RefCell::default();
        let (reply_tx, reply_rx) = mpsc::channel(CHANNEL_REPLY_BUF_SIZE);

        let reader = Self::channel_reader(tcp_reader, &working, reply_tx);
        let writer = Self::channel_writer(tcp_writer, reply_rx);

        let ret = futures::try_join!(reader, writer)?;
        Ok(())
    }

    async fn channel_reader(
        mut tcp_reader: BufReader<tcp::ReadHalf<'_>>,
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
            if let Some(request_tx) = working.get(&header.request_id) {
                if !header.flag.is_signal() {
                    request_tx.send(body.into()).await?;
                }
                if header.flag.is_eos() {
                    working.remove(&header.request_id);
                }
            } else {
                let (request_tx, request_rx) = mpsc::channel(CHANNEL_REQUEST_BUF_SIZE);
                let rw = ServerReaderWriter::new(reply_tx.clone(), request_rx);

                let service = Self::get_service(header.method_id);
                // TODO:
                let _ = tokio::task::spawn_local(async move { service.call_method(0, rw).await });

                if !header.flag.is_signal() {
                    request_tx.send(body.into()).await?;
                }
                if !header.flag.is_eos() {
                    working.insert(header.request_id, request_tx);
                }
            }
        }
    }

    async fn channel_writer(
        mut tcp_writer: BufWriter<tcp::WriteHalf<'_>>,
        reply_rx: mpsc::Receiver<WriteInfo>,
    ) -> Result<(), ServerError> {
        todo!()
    }

    fn get_service(method_id: u32) -> Rc<dyn Service> {
        todo!()
    }
}

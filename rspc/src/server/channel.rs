use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, RwLock},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{tcp, TcpStream},
    sync::mpsc,
    task,
};
use tracing::{debug, info};

use crate::{
    protocol::frame::{
        FrameFlag, FrameHeader, ReplyFrame, RequestFlagBit, RequestFrame, RequestHeader,
        REQUEST_FRAME_HEADER_LEN,
    },
    server::service::ServerReaderWriter,
};

use super::{
    error::ServerError,
    service::{Service, ServiceTable},
};

const CHANNEL_REPLY_BUF_SIZE: usize = 32;
const CHANNEL_REQUEST_BUF_SIZE: usize = 32;
const CHANNEL_SERVICE_BUF_SIZE: usize = 8;

pub struct Channel {
    stream: TcpStream,
    service_table: Rc<RefCell<ServiceTable>>,
}

impl Channel {
    pub fn new(stream: TcpStream, service_table: Rc<RefCell<ServiceTable>>) -> Self {
        Channel {
            stream,
            service_table,
        }
    }

    pub async fn init(&mut self) {
        todo!()
    }

    pub async fn run(&mut self) -> Result<(), ServerError> {
        let (tcp_reader, tcp_writer) = self.stream.split();
        let tcp_reader = BufReader::new(tcp_reader);
        let tcp_writer = BufWriter::new(tcp_writer);

        let (reply_tx, reply_rx) = mpsc::channel(CHANNEL_REPLY_BUF_SIZE);
        let (request_tx, request_rx) = mpsc::channel(CHANNEL_REQUEST_BUF_SIZE);

        let reader = Self::channel_reader(tcp_reader, request_tx);
        let request_handler = Self::request_handler(request_rx, reply_tx, &self.service_table);
        let writer = Self::channel_writer(tcp_writer, reply_rx);

        let local = task::LocalSet::new();
        let ret = local
            .run_until(futures::future::try_join3(reader, request_handler, writer))
            .await?;

        Ok(())
    }

    async fn channel_reader(
        mut tcp_reader: BufReader<tcp::ReadHalf<'_>>,
        request_tx: mpsc::Sender<RequestFrame>,
    ) -> Result<(), ServerError> {
        let mut header_buf = [0u8; REQUEST_FRAME_HEADER_LEN];
        loop {
            tcp_reader.read_exact(&mut header_buf).await?;
            let header = RequestHeader::decode(&header_buf[..])?;
            let mut body = vec![0u8; header.body_len as usize];
            tcp_reader.read_exact(&mut body).await?;
            let frame = RequestFrame {
                header,
                body: body.into(),
            };

            debug!(read_frame = %frame);

            request_tx.send(frame).await?;
        }
        todo!()
    }

    async fn request_handler(
        mut request_rx: mpsc::Receiver<RequestFrame>,
        reply_tx: mpsc::Sender<ReplyFrame>,
        service_table: &Rc<RefCell<ServiceTable>>,
    ) -> Result<(), ServerError> {
        // working service request stream record
        let working: RefCell<HashMap<u32, mpsc::Sender<RequestFrame>>> = RefCell::default();

        while let Some(frame) = request_rx.recv().await {
            let RequestFrame {
                header:
                    RequestHeader {
                        request_id,
                        method_id,
                        ref flag,
                        body_len,
                    },
                ref body,
            } = frame;

            use RequestFlagBit::*;
            // 3 flag: FIRST, EOS, SIGNAL
            //
            // FIRST && !EOS    run service and record
            // FIRST && EOS     run service not record
            // !FIRST && EOS    remove from record
            // !FIRST && !EOS   get from record
            // !SIGNAL          send message
            let service_tx = if flag.is(FIRST) {
                let service = service_table.borrow().get_service(method_id)?; // TODO: method_id Error handling

                info!(
                    service = service.service_name(),
                    method = service.method_name(),
                    "call service method"
                );

                let (service_tx, service_rx) = mpsc::channel(CHANNEL_SERVICE_BUF_SIZE);
                let rw = ServerReaderWriter::new(reply_tx.clone(), service_rx, request_id);
                task::spawn_local(async move {
                    let service = service;
                    service.call(rw).await
                });

                if !flag.is(EOS) {
                    let mut working = working.borrow_mut();
                    working.insert(request_id, service_tx.clone());
                }
                service_tx
            } else if flag.is(EOS) {
                let mut working = working.borrow_mut();
                working
                    .remove(&request_id)
                    .ok_or(ServerError::ServiceRecordError())?
            } else {
                let working = working.borrow_mut();
                working
                    .get(&request_id)
                    .map(|s| s.clone())
                    .ok_or(ServerError::ServiceRecordError())?
            };

            if !flag.is(SIGNAL) {
                // TODO: congestion handle, let one service method will not stuck whole server
                service_tx.send(frame).await?;
            }
        }
        todo!();
    }

    async fn channel_writer(
        mut tcp_writer: BufWriter<tcp::WriteHalf<'_>>,
        mut reply_rx: mpsc::Receiver<ReplyFrame>,
    ) -> Result<(), ServerError> {
        while let Some(frame) = reply_rx.recv().await {
            tcp_writer
                .write_all(&frame.header.encode_to_array())
                .await?;
            tcp_writer.write_all(&frame.body).await?;
            tcp_writer.flush().await?;

            debug!(write_frame = %frame);
        }
        todo!()
    }
}

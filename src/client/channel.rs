use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

use futures::TryFutureExt;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf, ReadHalf, WriteHalf},
        TcpStream, ToSocketAddrs,
    },
    sync::mpsc,
};
use tracing::debug;

use crate::protocol::frame::*;

use super::{ClientError, ClientReaderWriter};

const CHANNEL_REPLY_BUF_SIZE: usize = 32;
const CHANNEL_REQUEST_BUF_SIZE: usize = 32;
const CHANNEL_SERVICE_BUF_SIZE: usize = 8;

pub struct Channel {
    tcp: TcpStream,
}

pub struct RunningChannel {
    working: Rc<RefCell<HashMap<u32, mpsc::Sender<ReplyFrame>>>>,
    request_tx: mpsc::Sender<RequestFrame>,
    next_request_id: Cell<u32>,
}

impl RunningChannel {
    pub fn call_method(&self, method_id: u32) -> ClientReaderWriter {
        let writer_chan = self.request_tx.clone();
        let (service_tx, reader_chan) = mpsc::channel(CHANNEL_SERVICE_BUF_SIZE);
        let request_id = self.next_request_id.get();
        self.next_request_id.set(request_id + 1);
        self.working.borrow_mut().insert(request_id, service_tx);

        ClientReaderWriter::new(writer_chan, reader_chan, request_id, method_id)
    }
}

impl Channel {
    pub async fn new<A>(addr: A) -> Result<Self, ClientError>
    where
        A: ToSocketAddrs,
    {
        let tcp = TcpStream::connect(addr).await?;
        Ok(Self { tcp })
    }

    pub fn run<'a>(
        self,
    ) -> (
        impl futures::Future<Output = Result<(), ClientError>> + 'a,
        RunningChannel,
    ) {
        let (tcp_reader, tcp_writer) = self.tcp.into_split();
        let tcp_reader = BufReader::new(tcp_reader);
        let tcp_writer = BufWriter::new(tcp_writer);

        let (request_tx, request_rx) = mpsc::channel(CHANNEL_REQUEST_BUF_SIZE);
        let (reply_tx, reply_rx) = mpsc::channel(CHANNEL_REPLY_BUF_SIZE);

        let working = Rc::new(RefCell::new(HashMap::default()));

        let writer = Self::channel_writer(tcp_writer, request_rx);
        let reader = Self::channel_reader(tcp_reader, reply_tx);
        let reply_handler = Self::reply_handler(reply_rx, working.clone());

        let ret = async move {
            futures::future::try_join3(writer, reader, reply_handler).await?;
            Result::<(), ClientError>::Ok(())
        };
        (
            ret,
            RunningChannel {
                working,
                request_tx,
                next_request_id: Cell::new(0),
            },
        )
    }

    async fn channel_writer(
        mut tcp_writer: BufWriter<OwnedWriteHalf>,
        mut request_rx: mpsc::Receiver<RequestFrame>,
    ) -> Result<(), ClientError> {
        while let Some(frame) = request_rx.recv().await {
            tcp_writer
                .write_all(&frame.header.encode_to_array())
                .await?;
            tcp_writer.write_all(&frame.body).await?;
            tcp_writer.flush().await?;

            debug!(write_frame = ?frame);
        }
        todo!()
    }

    async fn channel_reader(
        mut tcp_reader: BufReader<OwnedReadHalf>,
        reply_tx: mpsc::Sender<ReplyFrame>,
    ) -> Result<(), ClientError> {
        let mut header_buf = [0u8; REPLY_FRAME_HEADER_LEN];
        loop {
            tcp_reader.read_exact(&mut header_buf).await?;
            let header = ReplyHeader::decode(&header_buf[..])?;
            let mut body = vec![0u8; header.body_len as usize];
            tcp_reader.read_exact(&mut body).await?;
            let frame = ReplyFrame {
                header,
                body: body.into(),
            };

            debug!(read_frame = ?frame);

            reply_tx.send(frame).await?;
        }
        todo!()
    }

    async fn reply_handler(
        mut reply_rx: mpsc::Receiver<ReplyFrame>,
        working: Rc<RefCell<HashMap<u32, mpsc::Sender<ReplyFrame>>>>,
    ) -> Result<(), ClientError> {
        while let Some(frame) = reply_rx.recv().await {
            let ReplyFrame {
                header:
                    ReplyHeader {
                        request_id,
                        status_code,
                        ref flag,
                        body_len,
                    },
                ref body,
            } = frame;

            use ReplyFlagBit::*;
            let service_tx = if flag.is(EOS) {
                working.borrow_mut().remove(&request_id)
            } else {
                working.borrow_mut().get(&request_id).map(|tx| tx.clone())
            }
            .ok_or(ClientError::ClientRecordError())?;

            if !flag.is(SIGNAL) {
                service_tx.send(frame).await?;
            }
        }
        Ok(())
    }
}

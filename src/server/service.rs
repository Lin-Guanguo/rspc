use async_trait::async_trait;
use bytes::Bytes;
use tokio::sync::mpsc;

use crate::protocol::frame::*;

use super::error::ServerError;

#[async_trait(?Send)]
pub trait Service {
    async fn call_method(&self, fn_n: usize, stream: ServerReaderWriter);

    fn method_names(&self) -> &'static [&'static str];

    fn num_of_methods(&self) -> usize;
}

pub struct ServerReaderWriter {
    writer: ServerWriter,
    reader: ServerReader,
}

impl ServerReaderWriter {
    pub fn new(
        writer_chan: mpsc::Sender<ReplyFrame>,
        reader_chan: mpsc::Receiver<RequestFrame>,
        request_id: u32,
    ) -> Self {
        Self {
            writer: ServerWriter {
                writer_chan,
                request_id,
            },
            reader: ServerReader { reader_chan },
        }
    }

    pub async fn write(&self, status_code: u32, reply_body: Bytes) -> Result<(), ServerError> {
        self.writer.write(status_code, reply_body).await
    }

    pub async fn write_last(&self, status_code: u32, reply_body: Bytes) -> Result<(), ServerError> {
        self.writer.write_last(status_code, reply_body).await
    }

    pub async fn write_complete(&self) -> Result<(), ServerError> {
        self.writer.write_complete().await
    }

    pub async fn read(&mut self) -> Option<Bytes> {
        self.reader.read().await
    }

    pub fn split(self) -> (ServerReader, ServerWriter) {
        (self.reader, self.writer)
    }
}

#[derive(Clone)]
pub struct ServerWriter {
    writer_chan: mpsc::Sender<ReplyFrame>,
    request_id: u32,
}

impl ServerWriter {
    fn new(writer_chan: mpsc::Sender<ReplyFrame>, request_id: u32) -> Self {
        Self {
            writer_chan,
            request_id,
        }
    }

    pub async fn write(&self, status_code: u32, reply_body: Bytes) -> Result<(), ServerError> {
        self.write_msg(ReplyFrame {
            header: ReplyHeader {
                request_id: self.request_id,
                flag: ReplyFlag::default(),
                status_code,
                body_len: reply_body.len() as u32,
            },
            body: reply_body,
        })
        .await
    }

    pub async fn write_last(&self, status_code: u32, reply_body: Bytes) -> Result<(), ServerError> {
        use ReplyFlagBit::*;
        self.write_msg(ReplyFrame {
            header: ReplyHeader {
                request_id: self.request_id,
                flag: ReplyFlag::default().set(EOS),
                status_code,
                body_len: reply_body.len() as u32,
            },
            body: reply_body,
        })
        .await
    }

    pub async fn write_complete(&self) -> Result<(), ServerError> {
        use ReplyFlagBit::*;
        self.write_msg(ReplyFrame {
            header: ReplyHeader {
                request_id: self.request_id,
                flag: ReplyFlag::default().set(EOS).set(SIGNAL),
                status_code: 0,
                body_len: 0,
            },
            body: Bytes::new(),
        })
        .await
    }

    async fn write_msg(&self, msg: ReplyFrame) -> Result<(), ServerError> {
        Ok(self.writer_chan.send(msg).await?)
    }
}

pub struct ServerReader {
    reader_chan: mpsc::Receiver<RequestFrame>,
}

impl ServerReader {
    fn new(reader_chan: mpsc::Receiver<RequestFrame>) -> Self {
        Self { reader_chan }
    }

    pub async fn read(&mut self) -> Option<Bytes> {
        let frame = self.reader_chan.recv().await;
        frame.map(|frame| frame.body)
    }
}

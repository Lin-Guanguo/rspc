use bytes::Bytes;
use tokio::sync::mpsc;

use crate::protocol::frame::*;

use super::ClientError;

pub trait ClientStub {
    fn channel(&self) -> &'_ crate::client::RunningChannel;

    fn first_method_id(&self) -> u32;
}

pub struct ClientReaderWriter {
    writer: ClientWriter,
    reader: ClientReader,
}

impl ClientReaderWriter {
    pub fn new(
        writer_chan: mpsc::Sender<RequestFrame>,
        reader_chan: mpsc::Receiver<ReplyFrame>,
        request_id: u32,
        method_id: u32,
    ) -> Self {
        Self {
            writer: ClientWriter::new(writer_chan, request_id, method_id),
            reader: ClientReader::new(reader_chan),
        }
    }

    pub async fn write(&mut self, reply_body: Bytes) -> Result<(), ClientError> {
        self.writer.write(reply_body).await
    }

    pub async fn write_last(&mut self, reply_body: Bytes) -> Result<(), ClientError> {
        self.writer.write_last(reply_body).await
    }

    pub async fn write_complete(&mut self) -> Result<(), ClientError> {
        self.writer.write_complete().await
    }

    pub async fn read(&mut self) -> Option<(u32, Bytes)> {
        self.reader.read().await
    }

    pub fn split(self) -> (ClientReader, ClientWriter) {
        (self.reader, self.writer)
    }
}

pub struct ClientReader {
    reader_chan: mpsc::Receiver<ReplyFrame>,
}

impl ClientReader {
    pub fn new(reader_chan: mpsc::Receiver<ReplyFrame>) -> Self {
        Self { reader_chan }
    }

    pub async fn read(&mut self) -> Option<(u32, Bytes)> {
        let frame = self.reader_chan.recv().await;
        frame.map(|frame| (frame.header.status_code, frame.body))
    }
}

pub struct ClientWriter {
    writer_chan: mpsc::Sender<RequestFrame>,
    have_write: bool,
    request_id: u32,
    method_id: u32,
}

impl ClientWriter {
    pub fn new(writer_chan: mpsc::Sender<RequestFrame>, request_id: u32, method_id: u32) -> Self {
        Self {
            writer_chan,
            have_write: false,
            request_id,
            method_id,
        }
    }

    pub async fn write(&mut self, request_body: Bytes) -> Result<(), ClientError> {
        self.write_msg(RequestFrame {
            header: RequestHeader {
                request_id: self.request_id,
                flag: RequestFlag::default(),
                method_id: self.method_id,
                body_len: request_body.len() as u32,
            },
            body: request_body,
        })
        .await
    }

    pub async fn write_last(&mut self, request_body: Bytes) -> Result<(), ClientError> {
        use RequestFlagBit::*;
        self.write_msg(RequestFrame {
            header: RequestHeader {
                request_id: self.request_id,
                flag: RequestFlag::default().set(EOS),
                method_id: self.method_id,
                body_len: request_body.len() as u32,
            },
            body: request_body,
        })
        .await
    }

    pub async fn write_complete(&mut self) -> Result<(), ClientError> {
        use RequestFlagBit::*;
        self.write_msg(RequestFrame {
            header: RequestHeader {
                request_id: self.request_id,
                flag: RequestFlag::default().set(EOS).set(SIGNAL),
                method_id: self.method_id,
                body_len: 0,
            },
            body: Bytes::new(),
        })
        .await
    }

    async fn write_msg(&mut self, mut msg: RequestFrame) -> Result<(), ClientError> {
        use RequestFlagBit::*;
        if !self.have_write {
            msg.header.flag.set_in_place(FIRST);
            self.have_write = true;
        }
        Ok(self.writer_chan.send(msg).await?)
    }
}

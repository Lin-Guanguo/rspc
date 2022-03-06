use async_trait::async_trait;
use bytes::Bytes;
use tokio::sync::mpsc;

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
    fn new(writer_chan: mpsc::Sender<WriteInfo>, reader_chan: mpsc::Receiver<Bytes>) -> Self {
        Self {
            writer: ServerWriter::new(writer_chan),
            reader: ServerReader::new(reader_chan),
        }
    }

    pub async fn write(&self, status_code: u32, request: Bytes) -> Result<(), ServerError> {
        self.writer.write(status_code, request).await
    }

    pub async fn write_last(&self, status_code: u32, request: Bytes) -> Result<(), ServerError> {
        self.writer.write_last(status_code, request).await
    }

    pub async fn write_complete(&self) -> Result<(), ServerError> {
        self.writer.write_complete().await
    }

    pub async fn read(&mut self) -> Option<Bytes> {
        self.reader.read().await
    }
}

#[derive(Clone)]
pub struct ServerWriter {
    writer_chan: mpsc::Sender<WriteInfo>,
}

#[derive(Debug)]
pub struct WriteInfo {
    eos: bool, // end of stream flag
    status_code: u32,
    body: Option<Bytes>, // message body, if None mean Signal msg
}

impl ServerWriter {
    fn new(writer_chan: mpsc::Sender<WriteInfo>) -> Self {
        Self { writer_chan }
    }

    pub async fn write(&self, status_code: u32, request: Bytes) -> Result<(), ServerError> {
        self.write_msg(WriteInfo {
            eos: false,
            status_code,
            body: Some(request),
        })
        .await
    }

    pub async fn write_last(&self, status_code: u32, request: Bytes) -> Result<(), ServerError> {
        self.write_msg(WriteInfo {
            eos: true,
            status_code,
            body: Some(request),
        })
        .await
    }

    pub async fn write_complete(&self) -> Result<(), ServerError> {
        self.write_msg(WriteInfo {
            eos: true,
            status_code: 0,
            body: None,
        })
        .await
    }

    pub async fn write_msg(&self, msg: WriteInfo) -> Result<(), ServerError> {
        Ok(self.writer_chan.send(msg).await?)
    }
}

pub struct ServerReader {
    reader_chan: mpsc::Receiver<Bytes>,
}

impl ServerReader {
    fn new(reader_chan: mpsc::Receiver<Bytes>) -> Self {
        Self { reader_chan }
    }

    pub async fn read(&mut self) -> Option<Bytes> {
        self.reader_chan.recv().await
    }
}

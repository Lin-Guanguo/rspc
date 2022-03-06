use bytes::Bytes;
use tokio::sync::mpsc;

use super::error::ServerError;

pub trait Service {
    fn call_method(&self, fn_n: usize, stream: ServerReaderWriter);

    fn method_names(&self) -> &'static [&'static str];

    fn num_of_methods(&self) -> usize;
}

pub struct ServerReaderWriter {
    writer: ServerWriter,
    reader: ServerReader,
}

impl ServerReaderWriter {
    fn new(writer_chan: mpsc::Sender<WriteMsg>, reader_chan: mpsc::Receiver<Bytes>) -> Self {
        Self {
            writer: ServerWriter::new(writer_chan),
            reader: ServerReader::new(reader_chan),
        }
    }

    async fn write(&self, request: Bytes) -> Result<(), ServerError> {
        self.writer.write(request).await
    }

    async fn write_last(&self, request: Bytes) -> Result<(), ServerError> {
        self.writer.write_last(request).await
    }

    pub async fn write_complete(&self) -> Result<(), ServerError> {
        self.writer.write_complete().await
    }

    async fn read(&mut self) -> Option<Bytes> {
        self.reader.read().await
    }
}

#[derive(Clone)]
pub struct ServerWriter {
    writer_chan: mpsc::Sender<WriteMsg>,
}

#[derive(Debug)]
pub struct WriteMsg {
    eos: bool,           // end of stream flag
    body: Option<Bytes>, // message body, if None mean Signal msg
}

impl ServerWriter {
    fn new(writer_chan: mpsc::Sender<WriteMsg>) -> Self {
        Self { writer_chan }
    }

    async fn write(&self, request: Bytes) -> Result<(), ServerError> {
        self.write_msg(WriteMsg {
            eos: false,
            body: Some(request),
        })
        .await
    }

    async fn write_last(&self, request: Bytes) -> Result<(), ServerError> {
        self.write_msg(WriteMsg {
            eos: true,
            body: Some(request),
        })
        .await
    }

    async fn write_complete(&self) -> Result<(), ServerError> {
        self.write_msg(WriteMsg {
            eos: true,
            body: None,
        })
        .await
    }

    pub async fn write_msg(&self, msg: WriteMsg) -> Result<(), ServerError> {
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

    async fn read(&mut self) -> Option<Bytes> {
        self.reader_chan.recv().await
    }
}

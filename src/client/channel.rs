use std::{cell::RefCell, collections::HashMap};

use tokio::{
    io::{BufReader, BufWriter},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream, ToSocketAddrs,
    },
    sync::mpsc,
};

use crate::protocol::frame::{ReplyFrame, RequestFrame};

use super::{ClientError, ClientReaderWriter};

pub struct Channel {
    working: RefCell<HashMap<u32, mpsc::Sender<ReplyFrame>>>,
}

impl Channel {
    pub async fn new<A>(addr: A) -> Result<Self, ClientError>
    where
        A: ToSocketAddrs,
    {
        let tcp = TcpStream::connect(addr).await?;
        let (tcp_reader, tcp_writer) = tcp.into_split();

        todo!()
    }

    pub fn call_method(&self, fn_n: u32) -> ClientReaderWriter {
        todo!()
    }

    async fn channel_writer(
        mut tcp_writer: BufWriter<OwnedWriteHalf>,
        request_rx: mpsc::Receiver<RequestFrame>,
    ) {
    }

    async fn channel_reader(
        mut tcp_reader: BufReader<OwnedReadHalf>,
        reply_tx: mpsc::Sender<ReplyFrame>,
    ) {
    }

    async fn reply_handler(
        reply_rx: mpsc::Receiver<ReplyFrame>,
        working: &RefCell<HashMap<u32, mpsc::Sender<ReplyFrame>>>,
    ) {
    }
}

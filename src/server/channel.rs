use tokio::net::TcpStream;

use crate::protocol::frame::{RequestFrame, RequestHeader};

pub struct Channel {
    stream: TcpStream,
}

impl Channel {
    pub fn new(stream: TcpStream) -> Self {
        Channel { stream }
    }

    async fn init(&mut self) {
        todo!()
    }

    async fn run(&mut self) {
        let (tcp_reader, tcp_writer) = self.stream.split();
    }
}

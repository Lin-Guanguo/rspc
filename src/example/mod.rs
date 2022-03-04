use bytes::Bytes;
use prost::Message;

include!(concat!(env!("OUT_DIR"), "/rspc.hello.rs"));

pub fn hello_service(reqeust: Bytes) -> (u32, Bytes) {
    let reqeust = HelloRequest::decode(&reqeust[..]).unwrap();
    let mut reply = HelloReply::default();
    reply.msg = format!("hello {}", reqeust.name);
    (0, reply.encode_to_vec().into())
}

pub trait HelloServer {
    fn hello_service(&mut self, reqeust: Bytes) -> (u32, Bytes);
}

struct HelloServerImpl {
    count: i32,
}

impl HelloServer for HelloServerImpl {
    fn hello_service(&mut self, reqeust: Bytes) -> (u32, Bytes) {
        let reqeust = HelloRequest::decode(&reqeust[..]).unwrap();
        let mut reply = HelloReply::default();
        reply.msg = format!("hello {}, count = {}", reqeust.name, self.count);
        (0, reply.encode_to_vec().into())
    }
}

struct HelloServerStub {
    // channel: Channel;
}

impl HelloServer for HelloServerStub {
    fn hello_service(&mut self, reqeust: Bytes) -> (u32, Bytes) {
        todo!()
    }
}

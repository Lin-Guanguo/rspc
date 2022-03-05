use bytes::Bytes;
use prost::Message;

use crate::protocol::service::Service;

include!(concat!(env!("OUT_DIR"), "/rspc.hello.rs"));

pub trait HelloService {
    const METHOD_NAMES: [&'static str; 2] = ["hello", "greeting"];

    const METHODS: [for<'r> fn(&'r Self, bytes::Bytes) -> (u32, bytes::Bytes); 2] =
        [Self::hello, Self::greeting];

    fn hello(&self, reqeust: Bytes) -> (u32, Bytes);

    fn greeting(&self, reqeust: Bytes) -> (u32, Bytes);
}

impl<S: HelloService> Service for S {
    fn call_method(&self, fn_n: usize, reqeust: Bytes) -> (u32, Bytes) {
        Self::METHODS[fn_n](&self, reqeust)
    }

    fn method_names(&self) -> &'static [&'static str] {
        &Self::METHOD_NAMES
    }

    fn num_of_methods(&self) -> usize {
        Self::METHOD_NAMES.len()
    }
}

pub struct HelloServerStub {
    // channel: Channel;
}

pub struct HelloServerImpl {
    pub count: i32,
}

impl HelloService for HelloServerImpl {
    fn hello(&self, reqeust: Bytes) -> (u32, Bytes) {
        let reqeust = HelloRequest::decode(&reqeust[..]).unwrap();
        let mut reply = HelloReply::default();
        reply.msg = format!("hello {}, count = {}", reqeust.name, self.count);
        (0, reply.encode_to_vec().into())
    }

    fn greeting(&self, reqeust: Bytes) -> (u32, Bytes) {
        let reqeust = HelloRequest::decode(&reqeust[..]).unwrap();
        let mut reply = HelloReply::default();
        reply.msg = format!("greeting {}, count = {}", reqeust.name, self.count);
        (0, reply.encode_to_vec().into())
    }
}

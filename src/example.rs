use bytes::Bytes;
use prost::Message;

include!(concat!(env!("OUT_DIR"), "/rspc.hello.rs"));

pub fn hello_service(reqeust: Bytes) -> (u32, Bytes) {
    let mut reply = HelloReply::default();
    reply.msg = String::from("Lin");
    (0, reply.encode_to_vec().into())
}

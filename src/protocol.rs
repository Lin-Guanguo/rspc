// channel transfer protocol

// request header bytes
pub const REQUEST_HEADER_BYTES: usize = 16;

// reply header bytes
pub const REPLY_HEADER_BYTES: usize = 16;

pub struct RequestMsg {
    request_id: u64,
    method_id: u32,
    body_len: u32,
    msg_body: Vec<u8>,
}

pub struct ReplyMsg {
    request_id: u64,
    status_code: u32,
    body_len: u32,
    msg_body: Vec<u8>,
}

use byteorder::ByteOrder;

impl RequestMsg {
    pub fn encode_header(request: RequestMsg) -> [u8; REQUEST_HEADER_BYTES] {
        let mut ret = [0; REQUEST_HEADER_BYTES];
        byteorder::NetworkEndian::write_u64(&mut ret, request.request_id);
        byteorder::NetworkEndian::write_u32(&mut ret[8..], request.method_id);
        byteorder::NetworkEndian::write_u32(&mut ret[8 + 4..], request.body_len);
        ret
    }

    pub fn decode_header(buf: [u8; REQUEST_HEADER_BYTES]) -> RequestMsg {
        RequestMsg {
            request_id: byteorder::NetworkEndian::read_u64(&buf),
            method_id: byteorder::NetworkEndian::read_u32(&buf[8..]),
            body_len: byteorder::NetworkEndian::read_u32(&buf[8 + 4..]),
            msg_body: vec![],
        }
    }
}

impl ReplyMsg {
    pub fn encode_header(reply: ReplyMsg) -> [u8; REPLY_HEADER_BYTES] {
        let mut ret = [0; REQUEST_HEADER_BYTES];
        byteorder::NetworkEndian::write_u64(&mut ret, reply.request_id);
        byteorder::NetworkEndian::write_u32(&mut ret[8..], reply.status_code);
        byteorder::NetworkEndian::write_u32(&mut ret[8 + 4..], reply.body_len);
        ret
    }

    pub fn decode_header(buf: [u8; REPLY_HEADER_BYTES]) -> ReplyMsg {
        ReplyMsg {
            request_id: byteorder::NetworkEndian::read_u64(&buf),
            status_code: byteorder::NetworkEndian::read_u32(&buf[8..]),
            body_len: byteorder::NetworkEndian::read_u32(&buf[8 + 4..]),
            msg_body: vec![],
        }
    }
}

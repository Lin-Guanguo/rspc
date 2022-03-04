use byteorder::ByteOrder;
use bytes::Bytes;

// request header bytes
pub const REQUEST_HEADER_BYTES: usize = 16;

// reply header bytes
pub const REPLY_HEADER_BYTES: usize = 16;

#[derive(Debug)]
pub struct RequestHeader {
    pub request_id: u64,
    pub method_id: u32,
    pub body_len: u32,
}

#[derive(Debug)]
pub struct RequestMsg {
    pub header: RequestHeader,
    pub body: Bytes,
}

#[derive(Debug)]
pub struct ReplyHeader {
    pub request_id: u64,
    pub status_code: u32,
    pub body_len: u32,
}

#[derive(Debug)]
pub struct ReplyMsg {
    pub header: ReplyHeader,
    pub body: Bytes,
}

impl RequestHeader {
    pub fn new(request_id: u64, method_id: u32, body_len: u32) -> Self {
        Self {
            request_id,
            method_id,
            body_len,
        }
    }

    pub fn decode(buf: &[u8; REQUEST_HEADER_BYTES]) -> Self {
        Self {
            request_id: byteorder::NetworkEndian::read_u64(buf),
            method_id: byteorder::NetworkEndian::read_u32(&buf[8..]),
            body_len: byteorder::NetworkEndian::read_u32(&buf[8 + 4..]),
        }
    }

    pub fn encode(&self) -> [u8; REQUEST_HEADER_BYTES] {
        let mut ret = [0; REQUEST_HEADER_BYTES];
        byteorder::NetworkEndian::write_u64(&mut ret, self.request_id);
        byteorder::NetworkEndian::write_u32(&mut ret[8..], self.method_id);
        byteorder::NetworkEndian::write_u32(&mut ret[8 + 4..], self.body_len);
        ret
    }
}

impl ReplyHeader {
    pub fn new(request_id: u64, status_code: u32, body_len: u32) -> Self {
        Self {
            request_id,
            status_code,
            body_len,
        }
    }

    pub fn decode(buf: &[u8; REPLY_HEADER_BYTES]) -> Self {
        Self {
            request_id: byteorder::NetworkEndian::read_u64(buf),
            status_code: byteorder::NetworkEndian::read_u32(&buf[8..]),
            body_len: byteorder::NetworkEndian::read_u32(&buf[8 + 4..]),
        }
    }

    pub fn encode(&self) -> [u8; REPLY_HEADER_BYTES] {
        let mut ret = [0; REQUEST_HEADER_BYTES];
        byteorder::NetworkEndian::write_u64(&mut ret, self.request_id);
        byteorder::NetworkEndian::write_u32(&mut ret[8..], self.status_code);
        byteorder::NetworkEndian::write_u32(&mut ret[8 + 4..], self.body_len);
        ret
    }
}

use bytes::{Buf, BufMut, Bytes};

pub const REQUEST_FRAME_HEADER_LEN: usize = 16;
pub const REPLY_FRAME_HEADER_LEN: usize = 16;

#[derive(Debug)]
pub struct RequestHeader {
    pub request_id: u64,
    pub method_id: u32,
    pub body_len: u32,
}

#[derive(Debug)]
pub struct RequestFrame {
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
pub struct ReplyFrame {
    pub header: ReplyHeader,
    pub body: Bytes,
}

#[derive(Debug, thiserror::Error)]
pub enum FrameError {
    #[error("decode error")]
    DecodeBufNotEnough,

    #[error("encode error")]
    EncodeBufNotEnough,
}

pub trait FrameHeader {
    fn decode<B>(buf: B) -> Result<Self, FrameError>
    where
        B: Buf,
        Self: Sized;

    fn encode<B>(&self, buf: &mut B) -> Result<(), FrameError>
    where
        B: BufMut;
}

impl FrameHeader for RequestHeader {
    fn decode<B>(mut buf: B) -> Result<Self, FrameError>
    where
        B: Buf,
    {
        if buf.remaining() < REQUEST_FRAME_HEADER_LEN {
            Err(FrameError::DecodeBufNotEnough)
        } else {
            Ok(Self {
                request_id: buf.get_u64(),
                method_id: buf.get_u32(),
                body_len: buf.get_u32(),
            })
        }
    }

    fn encode<B>(&self, buf: &mut B) -> Result<(), FrameError>
    where
        B: BufMut,
    {
        if buf.remaining_mut() < REQUEST_FRAME_HEADER_LEN {
            Err(FrameError::EncodeBufNotEnough)
        } else {
            buf.put_u64(self.request_id);
            buf.put_u32(self.method_id);
            buf.put_u32(self.body_len);
            Ok(())
        }
    }
}

impl FrameHeader for ReplyHeader {
    fn decode<B>(mut buf: B) -> Result<Self, FrameError>
    where
        B: Buf,
    {
        if buf.remaining() < REQUEST_FRAME_HEADER_LEN {
            Err(FrameError::DecodeBufNotEnough)
        } else {
            Ok(Self {
                request_id: buf.get_u64(),
                status_code: buf.get_u32(),
                body_len: buf.get_u32(),
            })
        }
    }

    fn encode<B>(&self, buf: &mut B) -> Result<(), FrameError>
    where
        B: BufMut,
    {
        if buf.remaining_mut() < REQUEST_FRAME_HEADER_LEN {
            Err(FrameError::EncodeBufNotEnough)
        } else {
            buf.put_u64(self.request_id);
            buf.put_u32(self.status_code);
            buf.put_u32(self.body_len);
            Ok(())
        }
    }
}

impl RequestHeader {
    pub fn encode_to_array(&self) -> [u8; REQUEST_FRAME_HEADER_LEN] {
        let mut ret = [0u8; REQUEST_FRAME_HEADER_LEN];
        let mut buf_mut = &mut ret[..];
        BufMut::put_u64(&mut buf_mut, self.request_id);
        BufMut::put_u32(&mut buf_mut, self.method_id);
        BufMut::put_u32(&mut buf_mut, self.body_len);
        ret
    }
}

impl ReplyHeader {
    pub fn encode_to_array(&self) -> [u8; REQUEST_FRAME_HEADER_LEN] {
        let mut ret = [0u8; REQUEST_FRAME_HEADER_LEN];
        let mut buf_mut = &mut ret[..];
        BufMut::put_u64(&mut buf_mut, self.request_id);
        BufMut::put_u32(&mut buf_mut, self.status_code);
        BufMut::put_u32(&mut buf_mut, self.body_len);
        ret
    }
}

use bytes::{Buf, BufMut, Bytes};

pub const REQUEST_FRAME_HEADER_LEN: usize = 16;
pub const REPLY_FRAME_HEADER_LEN: usize = 16;

#[derive(Debug)]
pub struct RequestHeader {
    pub request_id: u32,
    pub flag: RequestFlag,
    pub method_id: u32,
    pub body_len: u32,
}

#[derive(Debug)]
pub struct ReplyHeader {
    pub request_id: u32,
    pub flag: ReplyFlag,
    pub status_code: u32,
    pub body_len: u32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct RequestFlag {
    flag: u32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ReplyFlag {
    flag: u32,
}

pub enum RequestFlagBit {
    EOS = 0,
    SIGNAL = 1,
    FIRST = 2,
}

pub enum ReplyFlagBit {
    EOS = 0,
    SIGNAL = 1,
}

#[derive(Debug)]
pub struct ReplyFrame {
    pub header: ReplyHeader,
    pub body: Bytes,
}

#[derive(Debug)]
pub struct RequestFrame {
    pub header: RequestHeader,
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
                request_id: buf.get_u32(),
                flag: RequestFlag::decode(buf.get_u32()),
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
            buf.put_u32(self.request_id);
            buf.put_u32(self.flag.encode());
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
                request_id: buf.get_u32(),
                flag: ReplyFlag::decode(buf.get_u32()),
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
            buf.put_u32(self.request_id);
            buf.put_u32(self.flag.encode());
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
        BufMut::put_u32(&mut buf_mut, self.request_id);
        BufMut::put_u32(&mut buf_mut, self.flag.encode());
        BufMut::put_u32(&mut buf_mut, self.method_id);
        BufMut::put_u32(&mut buf_mut, self.body_len);
        ret
    }

    pub fn is_eos(&self) -> bool {
        todo!()
    }

    pub fn is_signal(&self) -> bool {
        todo!()
    }
}

impl ReplyHeader {
    pub fn encode_to_array(&self) -> [u8; REQUEST_FRAME_HEADER_LEN] {
        let mut ret = [0u8; REQUEST_FRAME_HEADER_LEN];
        let mut buf_mut = &mut ret[..];
        BufMut::put_u32(&mut buf_mut, self.request_id);
        BufMut::put_u32(&mut buf_mut, self.flag.encode());
        BufMut::put_u32(&mut buf_mut, self.status_code);
        BufMut::put_u32(&mut buf_mut, self.body_len);
        ret
    }
}

pub trait FrameFlag {
    fn decode(flag: u32) -> Self
    where
        Self: Sized;

    fn encode(&self) -> u32;

    type Bit;

    fn set(self, bit: Self::Bit) -> Self;

    fn clear(self, bit: Self::Bit) -> Self;

    fn is(&self, bit: Self::Bit) -> bool;
}

impl FrameFlag for RequestFlag {
    fn decode(flag: u32) -> Self
    where
        Self: Sized,
    {
        Self { flag }
    }

    fn encode(&self) -> u32 {
        self.flag
    }

    type Bit = RequestFlagBit;

    fn set(mut self, bit: Self::Bit) -> Self {
        self.flag |= 1 << (bit as i32);
        self
    }

    fn clear(mut self, bit: Self::Bit) -> Self {
        self.flag &= !(1 << (bit as i32));
        self
    }

    fn is(&self, bit: Self::Bit) -> bool {
        (self.flag & (1 << (bit as i32))) != 0
    }
}

impl FrameFlag for ReplyFlag {
    fn decode(flag: u32) -> Self
    where
        Self: Sized,
    {
        Self { flag }
    }

    fn encode(&self) -> u32 {
        self.flag
    }

    type Bit = ReplyFlagBit;

    fn set(mut self, bit: Self::Bit) -> Self {
        self.flag |= 1 << (bit as i32);
        self
    }

    fn clear(mut self, bit: Self::Bit) -> Self {
        self.flag &= !(1 << (bit as i32));
        self
    }

    fn is(&self, bit: Self::Bit) -> bool {
        (self.flag & (1 << (bit as i32))) != 0
    }
}

use tokio::sync::mpsc;

use crate::protocol::frame::{FrameError, ReplyFrame, RequestFrame};

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("IoError")]
    IoError(#[from] std::io::Error),

    #[error("Client write request to channel error")]
    RequestChannelSendError(#[from] mpsc::error::SendError<RequestFrame>),

    #[error("Client write Reply to channel error")]
    ReplyChannelSendError(#[from] mpsc::error::SendError<ReplyFrame>),

    #[error("framing error")]
    FrameError(#[from] FrameError),

    #[error("read reply but it's request_id can't find in record")]
    ClientRecordError(),
}

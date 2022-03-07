use bytes::Bytes;
use tokio::sync::mpsc;

use crate::protocol::frame::FrameError;

use super::service::WriteInfo;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("IoError")]
    IoError(#[from] std::io::Error),

    #[error("Service write reply to channel error")]
    ReplyChannelSendError(#[from] mpsc::error::SendError<WriteInfo>),

    #[error("Service write request to channel error")]
    RequestChannelSendError(#[from] mpsc::error::SendError<Bytes>),

    #[error("framing error")]
    FrameError(#[from] FrameError),
}

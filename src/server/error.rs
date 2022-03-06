use tokio::sync::mpsc;

use crate::protocol::frame::FrameError;

use super::service::WriteInfo;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("IoError")]
    IoError(#[from] std::io::Error),

    #[error("Service write mpsc send error")]
    ServiceWriteError(#[from] mpsc::error::SendError<WriteInfo>),

    #[error("framing error")]
    FrameError(#[from] FrameError),
}

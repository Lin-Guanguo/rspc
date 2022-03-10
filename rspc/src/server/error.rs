use tokio::sync::mpsc;

use crate::protocol::frame::{FrameError, ReplyFrame, RequestFrame};

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("IoError")]
    IoError(#[from] std::io::Error),

    #[error("Server write reply to buf channel error")]
    ReplyChannelSendError(#[from] mpsc::error::SendError<ReplyFrame>),

    #[error("Server write request to channel error")]
    RequestChannelSendError(#[from] mpsc::error::SendError<RequestFrame>),

    #[error("framing error")]
    FrameError(#[from] FrameError),

    #[error("not FIRST request but can't search in record table")]
    ServiceRecordError(),

    #[error("get service method with unregister id")]
    ErrorServiceMethodId(),

    #[error("normal rpc method run error")]
    NormalRpcMethodError(),

    #[error("stream rpc method run error")]
    StreamRpcMethodError(),
}

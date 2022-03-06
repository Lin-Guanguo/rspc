use tokio::sync::mpsc;

use super::service::WriteMsg;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("IoError")]
    IoError(#[from] std::io::Error),

    #[error("Service write mpsc send error")]
    ServiceWriteError(#[from] mpsc::error::SendError<WriteMsg>),
}

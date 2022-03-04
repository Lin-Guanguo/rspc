use tokio::sync::mpsc;

use super::channel::ChannelRequest;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("IoError")]
    IoError(#[from] std::io::Error),

    #[error("mpsc send error")]
    ChannelWriteError(#[from] mpsc::error::SendError<ChannelRequest>),
}

use tokio::sync::mpsc;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("IoError")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("IoError")]
    IoError(#[from] std::io::Error),
}

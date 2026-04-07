use thiserror::Error;

#[derive(Debug, Error)]
pub enum SdkError {
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    #[error("transport error: {0}")]
    Transport(String),
}

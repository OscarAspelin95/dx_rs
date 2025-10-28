use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProcessorError {
    #[error("Failed to connect to MinIO client")]
    MinIoClientError(String),
}

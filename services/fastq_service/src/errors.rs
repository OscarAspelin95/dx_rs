use log::SetLoggerError;
use shared::{minio::MinIoError, nats::NatsError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FastqError {
    #[error("Failed to filter fastq file with fastq_rs")]
    FastqRsFilterError(String),

    #[error("Serialization error")]
    SerializationError(String),

    #[error("Failed to fetch messages from consumer.")]
    GetConsumerMessagesError(String),

    #[error("Failed to acknowledge message")]
    MessageAckError(String),

    #[error("Failed to initialize logger")]
    LoggerInitializationError(String),

    #[error("Failed to create directory")]
    CreateDirectoryError(String),

    #[error("Failed to run fastq_rs")]
    FastqRsError(String),

    #[error(transparent)]
    MinIo(#[from] MinIoError),

    #[error(transparent)]
    Nats(#[from] NatsError),
}

impl From<serde_json::Error> for FastqError {
    fn from(err: serde_json::Error) -> Self {
        self::FastqError::SerializationError(err.to_string())
    }
}

impl From<SetLoggerError> for FastqError {
    fn from(err: SetLoggerError) -> Self {
        self::FastqError::LoggerInitializationError(err.to_string())
    }
}

impl From<std::io::Error> for FastqError {
    fn from(err: std::io::Error) -> Self {
        self::FastqError::CreateDirectoryError(err.to_string())
    }
}

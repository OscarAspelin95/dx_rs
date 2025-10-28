use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProcessorError {
    #[error("Failed to connect to MinIO client")]
    MinIoClientError(String),

    #[error("Failed to filter fastq file with fastq_rs")]
    FastqRsFilterError(String),
}

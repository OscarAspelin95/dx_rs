use thiserror::Error;

#[derive(Debug, Error)]
pub enum FastqError {
    #[error("Failed to filter fastq file with fastq_rs")]
    FastqRsFilterError(String),
}

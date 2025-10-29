use std::env::VarError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MinIoError {
    #[error("Unknown error occurred")]
    UnknownError(String),

    #[error("Missing environment variable")]
    MissingEnvironmentVariableError(String),

    #[error("Failed to connect to client")]
    ConnectionError(String),

    #[error("Failed to parse minio url")]
    UrlParseError(String),

    #[error("Failed to write contenst to file")]
    FileWriteError(String),
}

impl From<minio::s3::error::Error> for MinIoError {
    fn from(err: minio::s3::error::Error) -> Self {
        return self::MinIoError::ConnectionError(err.to_string());
    }
}

impl From<std::io::Error> for MinIoError {
    fn from(err: std::io::Error) -> Self {
        return self::MinIoError::FileWriteError(err.to_string());
    }
}

impl From<VarError> for MinIoError {
    fn from(err: VarError) -> Self {
        return self::MinIoError::MissingEnvironmentVariableError(err.to_string());
    }
}

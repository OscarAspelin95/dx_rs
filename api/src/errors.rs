use std::env::VarError;

use async_nats::{self, ConnectErrorKind, jetstream::context::CreateStreamErrorKind};
use axum::{extract::multipart::MultipartError, http::StatusCode};
use thiserror::Error;

use axum::response::{IntoResponse, Response};

#[derive(Error, Debug)]
pub enum ApiError {
    // Any unknown error.
    #[error("Unknown error")]
    UnknownError(String),

    // SurrealDB related.
    #[error("Database connection error")]
    DatabaseConnectionTimeoutError(),

    #[error("Database record creation failed")]
    DatabaseRecordCreateError(String),

    #[error("Database is unhealthy")]
    DatabaseUnhealthyError(String),

    // Api related.
    #[error("Missing environment variables")]
    MissingEnvironmentVariable(String),

    #[error("Invalid multiform")]
    InvalidMultiFormError(String),

    // MinIO Related.
    #[error("MinIO connection error")]
    MinioConnectionError(String),

    // Nats related.
    #[error("NATS connection error")]
    NatsConnectionError(String),

    #[error("NATS stream creation error")]
    NatsStreamError(String),

    #[error("NATS publish error")]
    NatsPublishError(String),
}

// Custom error handling for UnknownError (misc error).
impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        return self::ApiError::UnknownError(err.kind().to_string());
    }
}

// Custom error handling for inserting a record in surrealdb.
impl From<surrealdb::Error> for ApiError {
    fn from(err: surrealdb::Error) -> Self {
        return self::ApiError::DatabaseRecordCreateError(err.to_string());
    }
}

impl From<VarError> for ApiError {
    fn from(err: VarError) -> Self {
        return self::ApiError::MissingEnvironmentVariable(err.to_string());
    }
}

impl From<MultipartError> for ApiError {
    fn from(err: MultipartError) -> Self {
        return self::ApiError::InvalidMultiFormError(err.to_string());
    }
}

impl From<minio::s3::error::Error> for ApiError {
    fn from(err: minio::s3::error::Error) -> Self {
        return self::ApiError::MinioConnectionError(err.to_string());
    }
}

impl From<async_nats::error::Error<ConnectErrorKind>> for ApiError {
    fn from(err: async_nats::error::Error<ConnectErrorKind>) -> Self {
        return self::ApiError::NatsConnectionError(err.to_string());
    }
}

impl From<async_nats::error::Error<CreateStreamErrorKind>> for ApiError {
    fn from(err: async_nats::error::Error<CreateStreamErrorKind>) -> Self {
        return self::ApiError::NatsStreamError(err.to_string());
    }
}

impl From<async_nats::error::Error<async_nats::jetstream::context::PublishErrorKind>> for ApiError {
    fn from(
        err: async_nats::error::Error<async_nats::jetstream::context::PublishErrorKind>,
    ) -> Self {
        return self::ApiError::NatsPublishError(err.to_string());
    }
}

/// Not sure we should still call it ApiError. Maybe AppError?
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::UnknownError(e) => (StatusCode::BAD_REQUEST, format!("{}", e)),
            ApiError::DatabaseConnectionTimeoutError() => (
                StatusCode::BAD_REQUEST,
                format!("Database connection retries exceeded"),
            ),
            ApiError::DatabaseRecordCreateError(s) => (
                StatusCode::BAD_REQUEST,
                format!("Failed to create record: {:?}", s),
            ),
            ApiError::DatabaseUnhealthyError(s) => (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Database in unhealthy: {}", s),
            ),
            ApiError::MissingEnvironmentVariable(s) => (
                StatusCode::FAILED_DEPENDENCY,
                format!("Missing environment variable: {}", s),
            ),
            ApiError::InvalidMultiFormError(s) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid multiform error {}", s),
            ),
            ApiError::MinioConnectionError(s) => (
                StatusCode::BAD_REQUEST,
                format!("Minio connection error {}", s),
            ),
            ApiError::NatsConnectionError(s) => (
                StatusCode::BAD_REQUEST,
                format!("Nats connection error {}", s),
            ),
            ApiError::NatsStreamError(s) => (
                StatusCode::BAD_REQUEST,
                format!("Nats stream creation error {}", s),
            ),
            ApiError::NatsPublishError(s) => {
                (StatusCode::BAD_REQUEST, format!("Nats publish error {}", s))
            }
        };

        (status, error_message).into_response()
    }
}

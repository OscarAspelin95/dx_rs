use std::env::VarError;

use async_nats::{self, ConnectErrorKind, jetstream::context::CreateStreamErrorKind};
use axum::{extract::multipart::MultipartError, http::StatusCode};
use thiserror::Error;

use axum::response::{IntoResponse, Response};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Some Error")]
    SomeError(String),

    #[error("Database connection error")]
    DatabaseConnectionTimeoutError(),

    #[error("Database record creation failed")]
    DatabaseRecordCreateError(String),

    #[error("Database is unhealthy")]
    DatabaseUnhealthyError(String),

    #[error("Missing environment variables")]
    MissingEnvironmentVariable(String),

    #[error("Invalid multiform")]
    InvalidMultiFormError(String),

    #[error("MinIO connection error")]
    MinioConnectionError(String),

    #[error("NATS connection error")]
    NatsConnectionError(String),

    #[error("NATS stream creation error")]
    NatsStreamError(String),
}

// Custom error handling for SomeError (misc error).
impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        return self::ApiError::SomeError(err.kind().to_string());
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

// We might need to split errors into separate (Api, Internal, etc).
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::SomeError(e) => (StatusCode::BAD_REQUEST, format!("{}", e)),
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
        };

        (status, error_message).into_response()
    }
}

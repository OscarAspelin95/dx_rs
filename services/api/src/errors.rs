use std::env::VarError;

use axum::response::{IntoResponse, Response};
use axum::{extract::multipart::MultipartError, http::StatusCode};

use shared::database::DatabaseError;
use shared::minio::MinIoError;
use shared::nats::NatsError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Unknown error")]
    UnknownError(String),

    #[error("Missing environment variables")]
    MissingEnvironmentVariable(String),

    #[error("Invalid multiform")]
    InvalidMultiFormError(String),

    #[error("Failed to insert db record")]
    DatabaseRecordInsertError(String),

    // Shared errors
    #[error(transparent)]
    MinIo(#[from] MinIoError),

    #[error(transparent)]
    Nats(#[from] NatsError),

    #[error(transparent)]
    Database(#[from] DatabaseError),
}

// Not sure where this fits in.
impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        return self::ApiError::UnknownError(err.kind().to_string());
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

impl From<surrealdb::Error> for ApiError {
    fn from(err: surrealdb::Error) -> Self {
        self::ApiError::DatabaseRecordInsertError(err.to_string())
    }
}

// Implement Axum API request reponse error for our defined API errors.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::UnknownError(e) => (StatusCode::BAD_REQUEST, format!("{}", e)),

            ApiError::MissingEnvironmentVariable(s) => (
                StatusCode::FAILED_DEPENDENCY,
                format!("Missing environment variable: {}", s),
            ),
            ApiError::InvalidMultiFormError(s) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid multiform error {}", s),
            ),
            ApiError::DatabaseRecordInsertError(s) => {
                (StatusCode::BAD_REQUEST, format!("Database error: {:?}", s))
            }
            ApiError::MinIo(s) => (StatusCode::BAD_REQUEST, format!("MinIO error: {:?}", s)),
            ApiError::Nats(s) => (StatusCode::BAD_REQUEST, format!("Nats error: {:?}", s)),
            ApiError::Database(s) => (StatusCode::BAD_REQUEST, format!("Database error: {:?}", s)),
        };

        (status, error_message).into_response()
    }
}

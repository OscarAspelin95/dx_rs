use axum::http::StatusCode;
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
        };

        (status, error_message).into_response()
    }
}

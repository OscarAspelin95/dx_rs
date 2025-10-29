use std::env::VarError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Unknown error")]
    UnknownError(String),

    #[error("Missing environment variable")]
    MissingEnvironmentVariableError(String),

    #[error("Database connection error")]
    DatabaseConnectionTimeoutError(),

    #[error("Database record creation failed")]
    DatabaseRecordCreateError(String),

    #[error("Database is unhealthy")]
    DatabaseUnhealthyError(String),
}

impl From<VarError> for DatabaseError {
    fn from(err: VarError) -> Self {
        return self::DatabaseError::MissingEnvironmentVariableError(err.to_string());
    }
}

impl From<surrealdb::Error> for DatabaseError {
    fn from(err: surrealdb::Error) -> Self {
        return self::DatabaseError::DatabaseRecordCreateError(err.to_string());
    }
}

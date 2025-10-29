use async_nats::{
    self, ConnectErrorKind,
    jetstream::context::{CreateStreamErrorKind, GetStreamErrorKind},
};
use serde::ser::StdError;
use std::env::VarError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NatsError {
    #[error("Unknown error")]
    UnknownError(String),

    #[error("Missing environment variable")]
    MissingEnvironmentVariableError(String),

    #[error("Client connection error")]
    NatsConnectionError(String),

    #[error("Failed to create stream")]
    CreateStreamError(String),

    #[error("Failed to get stream")]
    GetStreamError(String),

    #[error("Failed to get consumer")]
    GetConsumerError(String),

    #[error("NATS publish error")]
    NatsPublishError(String),
}

impl From<VarError> for NatsError {
    fn from(err: VarError) -> Self {
        return self::NatsError::MissingEnvironmentVariableError(err.to_string());
    }
}

impl From<async_nats::error::Error<ConnectErrorKind>> for NatsError {
    fn from(err: async_nats::error::Error<ConnectErrorKind>) -> Self {
        return self::NatsError::NatsConnectionError(err.to_string());
    }
}

impl From<async_nats::error::Error<CreateStreamErrorKind>> for NatsError {
    fn from(err: async_nats::error::Error<CreateStreamErrorKind>) -> Self {
        return self::NatsError::CreateStreamError(err.to_string());
    }
}

impl From<async_nats::error::Error<async_nats::jetstream::context::PublishErrorKind>>
    for NatsError
{
    fn from(
        err: async_nats::error::Error<async_nats::jetstream::context::PublishErrorKind>,
    ) -> Self {
        return self::NatsError::NatsPublishError(err.to_string());
    }
}

impl From<async_nats::error::Error<GetStreamErrorKind>> for NatsError {
    fn from(err: async_nats::error::Error<GetStreamErrorKind>) -> Self {
        self::NatsError::GetStreamError(err.to_string())
    }
}

impl From<Box<dyn StdError + std::marker::Send + std::marker::Sync>> for NatsError {
    fn from(err: Box<dyn StdError + std::marker::Send + std::marker::Sync>) -> Self {
        self::NatsError::GetConsumerError(err.to_string())
    }
}

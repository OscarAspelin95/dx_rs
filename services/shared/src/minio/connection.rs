use minio::s3::{Client as MinioClient, ClientBuilder, creds::StaticProvider};

use crate::minio::errors::MinIoError;

pub async fn connect_minio() -> Result<MinioClient, MinIoError> {
    let static_provider = StaticProvider::new(
        &std::env::var("MINIO_ROOT_USER")?,
        &std::env::var("MINIO_ROOT_PASSWORD")?,
        None,
    );

    let client = ClientBuilder::new(std::env::var("MINIO_HTTP_ENDPOINT")?.parse()?)
        .provider(Some(Box::new(static_provider)))
        .build();

    match client {
        Ok(client) => Ok(client),
        Err(e) => Err(MinIoError::UnknownError(e.to_string())),
    }
}

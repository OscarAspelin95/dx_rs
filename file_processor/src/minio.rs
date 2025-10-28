use crate::errors::ProcessorError;
use minio::s3::{Client as MinioClient, ClientBuilder, creds::StaticProvider};

pub async fn connect_minio() -> Result<MinioClient, ProcessorError> {
    let static_provider = StaticProvider::new(
        &std::env::var("MINIO_ROOT_USER").unwrap(),
        &std::env::var("MINIO_ROOT_PASSWORD").unwrap(),
        None,
    );

    // Improve by .build()?, impl From<minio::s3::Error> for ProcessorError.
    let client = ClientBuilder::new(
        std::env::var("MINIO_HTTP_ENDPOINT")
            .expect("Missing Minio HTTP Endpoint in environment.")
            .parse()
            .expect("Failed to parse minio base url"),
    )
    .provider(Some(Box::new(static_provider)))
    .build();

    match client {
        Ok(client) => Ok(client),
        Err(e) => Err(ProcessorError::MinIoClientError(e.to_string())),
    }
}

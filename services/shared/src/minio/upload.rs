use crate::minio::errors::MinIoError;
use log::{error, info};
use minio::s3::{Client, segmented_bytes::SegmentedBytes, types::S3Api};

pub async fn create_bucket_if_not_exists(bucket: &str, client: &Client) -> Result<(), MinIoError> {
    match client.bucket_exists(bucket).send().await {
        Ok(response) => match response.exists {
            false => {
                info!("Bucket does not exist. Creating...");
                client.create_bucket(bucket).send().await?;
                info!("Succeeded.");
                Ok(())
            }
            true => {
                info!("Bucket already exists.");
                Ok(())
            }
        },
        Err(e) => {
            error!("{:?}", e);
            Err(MinIoError::UnknownError(
                "Failed to check if minio bucket exists.".into(),
            ))
        }
    }
}

pub async fn minio_upload(
    client: &Client,
    bucket: &str,
    key: &str,
    file_contents: SegmentedBytes,
) -> Result<String, MinIoError> {
    create_bucket_if_not_exists(bucket, client).await?;

    info!("Uploading {} to bucket {}", key, bucket);
    let response = client.put_object(bucket, key, file_contents).send().await?;
    info!("MinIO response: {:?}", response);

    // Does not seem like minio really returns a url, so we construct it here.
    let url = format!("http://{}/{bucket}/{key}", std::env::var("MINIO_ENDPOINT")?);
    Ok(url)
}

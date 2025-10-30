use crate::minio::errors::MinIoError;
use bytes::Bytes;
use log::{error, info};
use minio::s3::{Client, segmented_bytes::SegmentedBytes, types::S3Api};
use std::io::Read;
use std::path::PathBuf;

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

/// Rename to minio upload bytes.
pub async fn minio_upload_bytes(
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

/// This essentially is just some boilerplate for converting
/// a file (path) to segmented bytes before MinIO upload.
pub async fn minio_upload_file(
    client: &Client,
    bucket: &str,
    key: &str,
    file: PathBuf,
) -> Result<String, MinIoError> {
    let mut file_handle = std::fs::File::open(&file)?;

    // This can have potential side-effects for large files.
    let mut buf: Vec<u8> = Vec::new();
    file_handle.read_to_end(&mut buf)?;

    // There probably is a better way. NOTE - MinIO has a lower
    // limit of 5Mb per chunk (except for the last one). Without
    // explicitly checking this, we might run into issues.
    let segbuf: SegmentedBytes = Bytes::from_owner(buf).into();

    //
    minio_upload_bytes(client, bucket, key, segbuf).await
}

use axum::body::Bytes;
use log::{error, info};
use minio::s3::{Client, types::S3Api};

use crate::{errors::ApiError, schema::file_upload::UploadField, utils::time_now};
use uuid;

async fn create_bucket_if_not_exists(bucket: &str, minio_client: &Client) -> Result<(), ApiError> {
    match minio_client.bucket_exists(bucket).send().await {
        Ok(response) => match response.exists {
            false => {
                info!("Bucket does not exist. Creating...");
                minio_client.create_bucket(bucket).send().await?;
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
            Err(ApiError::SomeError(
                "Failed to check if minio bucket exists.".into(),
            ))
        }
    }
}

pub async fn file_upload(
    bucket: &str,
    key: &str,
    file_contents: Bytes,
    minio_client: &Client,
) -> Result<UploadField, ApiError> {
    create_bucket_if_not_exists(bucket, minio_client).await?;

    // Add uuid to make sure key is unique.
    let file_uuid = uuid::Uuid::now_v7().to_string();
    let minio_key = format!("{}/{}", file_uuid, key);

    // Actual file upload
    minio_client
        .put_object(bucket, &minio_key, file_contents.into())
        .send()
        .await?;

    Ok(UploadField {
        file_name: key.into(),
        url: format!("http://minio:9000/{bucket}/{minio_key}"),
        created_at: time_now(),
        uuid: file_uuid,
    })
}

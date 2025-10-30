use axum::body::Bytes;
use minio::s3::Client;

use crate::schema::file_upload::UploadField;
use shared::minio::{errors::MinIoError, upload::minio_upload_bytes};
use shared::utils::time::time_now;
use uuid;

pub async fn file_upload(
    bucket: &str,
    key: &str,
    file_contents: Bytes,
    minio_client: &Client,
) -> Result<UploadField, MinIoError> {
    // Add uuid to make sure key is unique.
    let file_uuid = uuid::Uuid::now_v7().to_string();
    let minio_key = format!("{}/{}", file_uuid, key);

    let url = minio_upload_bytes(minio_client, bucket, &minio_key, file_contents.into()).await?;

    Ok(UploadField {
        file_name: key.into(),
        url: url,
        created_at: time_now(),
        uuid: file_uuid,
    })
}

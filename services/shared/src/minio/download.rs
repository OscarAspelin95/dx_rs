use crate::minio::errors::MinIoError;
use crate::utils::url::parse_url;
use log::info;
use minio::s3::Client;
use minio::s3::types::S3Api;
use std::path::PathBuf;

pub async fn minio_download<'a>(
    client: &'a Client,
    url: &'a str,
    dst: &'a PathBuf,
) -> Result<&'a PathBuf, MinIoError> {
    // Parse url
    let parsed_url = parse_url(url).ok_or(MinIoError::UrlParseError(url.to_string()))?;

    info!("Parsed url: {:?}", parsed_url);

    // Download file with MinIO...
    let object_response = client
        .get_object(parsed_url.bucket, parsed_url.key)
        .send()
        .await?;

    let written_bytes = object_response.content.to_file(dst).await?;
    info!("Wrote {} bytes to file {:?}", written_bytes, dst);

    // Probably overkill.
    assert!(dst.exists());
    info!("Successfully downloaded file to {:?}", dst);

    Ok(dst)
}

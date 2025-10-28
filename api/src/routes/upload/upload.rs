use axum::{
    Json,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use log::info;
use serde_json::json;

use crate::minio_upload::file_upload;
use crate::schema::file_upload::UploadField;
use crate::state::ConnectionState;
use crate::{errors::ApiError, nats::publisher::file_upload::nats_publish_upload};

pub async fn upload_file(
    State(state): State<ConnectionState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    let db = state.surrealdb.client;
    let minio = state.minio.client;
    let nats = state.nats.client;

    // Might not be ideal to do stuff here in the while loop.
    // Alternatively, we defined Optional placeholder values first,
    // Then when we have captured everything we need, we run after the while loop:
    // * file_upload.
    // * create database record.
    // * Publish to NATS.

    // Temp solution.
    let mut url: Option<String> = None;

    while let Some(field) = multipart.next_field().await? {
        let name = field.name();
        match name {
            Some("file") => {
                let file_name = field.file_name().expect("").to_string();

                let file_contents = field.bytes().await?;
                let upload_field =
                    file_upload("my-bucket", &file_name, file_contents, &minio).await?;

                // Temp solution.
                url = Some(upload_field.url.clone());

                // Write url to database.
                let response: Option<UploadField> =
                    db.create("file_upload").content(upload_field).await?;

                info!("{:?}", response);
            }
            _ => {
                info!("Unexpected field: {:?}", name);
            }
        }
    }

    // Async publish to NATS.
    // Not sure we need async though.
    tokio::spawn(async move {
        nats_publish_upload(nats, url.unwrap())
            .await
            .expect("Failed to publish nats upload message")
    })
    .await
    // Fix.
    .map_err(|err| ApiError::UnknownError(err.to_string()))?;

    // multipart ... something.
    Ok((StatusCode::OK, Json(json!({"upload": "success"}))))
}

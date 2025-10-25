use axum::{
    Json,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::minio_upload::file_upload;
use crate::state::ConnectionState;
use crate::{errors::ApiError, schema::UploadField};

#[derive(Debug, Serialize, Deserialize)]
struct NatsFileMessage {
    url: String,
}

pub async fn upload_file(
    State(state): State<ConnectionState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    let db = state.surrealdb;
    let minio = state.minio;
    let nats = state.nats;

    // Might not be ideal to do stuff here in the while loop.
    // Alternatively, we defined Optional placeholder values first,
    // Then when we have captured everything we need, we run after the while loop:
    // * file_upload.
    // * create database record.
    // * Publish to NATS.
    while let Some(field) = multipart.next_field().await? {
        let name = field.name();
        match name {
            Some("file") => {
                let file_name = field.file_name().expect("").to_string();

                let file_contents = field.bytes().await?;
                let upload_field =
                    file_upload("my-bucket", &file_name, file_contents, &minio).await?;

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

    let nats_file_message = NatsFileMessage {
        url: "my_url".into(),
    };

    // Async publish to NATS.
    tokio::spawn(async move {
        let ack = nats
            .publish(
                "file-uploaded.process",
                serde_json::to_string(&nats_file_message)
                    .expect("Failed to serialize NATS message.")
                    // Fix.
                    .into(),
            )
            .await
            .expect("Failed to publish NATS message.");

        ack.await
            .expect("Message could not be acknowledged by the server.");
    })
    .await
    // Fix.
    .map_err(|err| ApiError::SomeError(err.to_string()))?;

    // multipart ... something.
    Ok((StatusCode::OK, Json(json!({"upload": "success"}))))
}

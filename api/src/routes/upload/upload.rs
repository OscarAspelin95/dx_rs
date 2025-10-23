use axum::{
    Json,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use log::info;
use serde_json::json;

use crate::minio_upload::file_upload;
use crate::state::ConnectionState;
use crate::{errors::ApiError, schema::UploadField};

pub async fn upload_file(
    State(state): State<ConnectionState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    let db = state.surrealdb;
    let minio = state.minio;

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

    // multipart ... something.
    Ok((StatusCode::OK, Json(json!({"upload": "success"}))))
}

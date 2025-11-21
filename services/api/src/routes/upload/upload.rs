use axum::{
    Json,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use bytes::Bytes;
use log::info;
use serde_json::json;
use shared::{
    database::schemas::{
        User,
        fastq_sample::{FastqSample, FastqSampleConfig, FastqSampleData},
    },
    nats::schema::fastq_service::FastqMessage,
    schema::schema::{Pipeline, Status},
    utils::time::time_now,
};

use crate::errors::ApiError;
use crate::minio_upload::file_upload;
use crate::nats::publisher::file_upload::nats_publish_upload;
use crate::state::ConnectionState;

pub async fn upload_file(
    State(state): State<ConnectionState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    let db = state.surrealdb.client;
    let minio = state.minio.client;
    let nats = state.nats.client;

    // Temp solution.
    let mut pipeline: Option<Pipeline> = None;
    let mut file_name: Option<String> = None;
    let mut file_contents: Option<Bytes> = None;

    while let Some(field) = multipart.next_field().await? {
        let name = field.name();

        match name {
            Some("file") => {
                file_name = Some(field.file_name().unwrap().to_string());
                file_contents = Some(field.bytes().await?);
            }
            Some("pipeline") => {
                let pipeline_parsed = serde_json::from_str(&field.text().await?)
                    .expect("Failed to serialize pipeline field.");

                pipeline = Some(pipeline_parsed);
            }
            _ => {
                info!("Unexpected field: {:?}", name);
            }
        }
    }

    // We should have better error handling here for
    // cases where pipeline, file_name or file_contents are None.
    // For now, we just unwrap.

    // Upload to MinIO
    let upload_field = file_upload(
        "my-bucket",
        &file_name.as_ref().unwrap(),
        file_contents.unwrap(),
        &minio,
    )
    .await?;

    // Construct our db fastq sample.
    let fastq_sample = FastqSample {
        id: None,
        data: FastqSampleData {
            name: file_name.unwrap().to_string(),
            status: Status::Done,
            url: upload_field.url.clone(),
            pipeline: pipeline.unwrap(),
            config: FastqSampleConfig::mock(),
            created_at: time_now(),
            updated_at: time_now(),
        },
    };

    // MOVE database write to somewhere else later on.
    // First, create a user (mock new user everytime for now).
    let user_response: User = db.create("users").content(User::mock()).await?.unwrap();

    // Then, write fastq sample to database.
    let sample_response: FastqSample = db
        .create("fastq_samples")
        .content(fastq_sample)
        .await?
        .unwrap();

    // Lastly, create a relation between user and fastq sample.
    let relation_response = db
        .query("RELATE $user->uploaded->$fastq_sample")
        .bind(("user", user_response.id.as_ref().unwrap().surrealdb_id()?))
        .bind((
            "fastq_sample",
            sample_response.id.as_ref().unwrap().surrealdb_id()?,
        ))
        .await?;

    info!("Relation response: {:?}", relation_response);

    // Send message to fastq preprocessor.
    let fastq_message = FastqMessage {
        url: upload_field.url,
        fastq_sample_id: sample_response.id.unwrap(),
    };
    nats_publish_upload(nats, &fastq_message).await?;

    // multipart ... something.
    Ok((StatusCode::OK, Json(json!({"upload": "success"}))))
}

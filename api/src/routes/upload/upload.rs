use axum::{
    Json,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::errors::ApiError;
use crate::state::ConnectionState;

pub async fn upload_file(
    State(state): State<ConnectionState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    let db = state.surrealdb;

    while let Ok(Some(field)) = multipart.next_field().await {
        println!("{:?}", field);

        // Check for field with "file" as name.
    }

    // multipart ... something.
    Ok((StatusCode::OK, Json({})))
}

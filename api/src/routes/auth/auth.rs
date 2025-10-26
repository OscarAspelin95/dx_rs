use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::errors::ApiError;
use crate::state::ConnectionState;

pub async fn test_auth(
    State(state): State<ConnectionState>,
) -> Result<impl IntoResponse, ApiError> {
    let db = state.surrealdb;

    Ok((StatusCode::OK, Json({})))
}

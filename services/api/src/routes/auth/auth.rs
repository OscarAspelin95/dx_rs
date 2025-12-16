use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use log::{error, info};

use crate::{errors::ApiError, state::ConnectionState};

pub async fn test_auth(
    State(state): State<ConnectionState>,
) -> Result<impl IntoResponse, ApiError> {
    let db = state.surrealdb.client;

    match db.health().await {
        Ok(()) => info!("Database healthy!"),
        Err(e) => error!("{:?}", e),
    };

    Ok((StatusCode::OK, Json(serde_json::json!({"status": "authenticated"}))))
}

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use log::info;

use crate::errors::ApiError;
use crate::schema::ToDoItem;
use crate::state::ConnectionState;

pub async fn get_tasks(
    State(state): State<ConnectionState>,
) -> Result<impl IntoResponse, ApiError> {
    let db = state.surrealdb;

    // We just select all tasks for now.
    let tasks: Vec<ToDoItem> = db.select("todo").await?;

    //
    Ok((StatusCode::OK, Json(tasks)))
}

// NOTE - extractors must come first, then Body, Json, Form, Multipart, etc.
pub async fn add_task(
    State(state): State<ConnectionState>,
    Json(payload): Json<ToDoItem>,
) -> Result<impl IntoResponse, ApiError> {
    let db = state.surrealdb;

    let response: Option<ToDoItem> = db.create("todo").content(payload).await?;

    let response = match response {
        Some(response) => response,
        None => {
            return Err(ApiError::DatabaseRecordCreateError(
                "Failed to insert into todo table.".into(),
            ));
        }
    };

    //
    Ok((StatusCode::OK, Json(response)))
}

/// Here, we also want to implement remove taks.
pub async fn remove_task(
    State(state): State<ConnectionState>,
    Path(uuid): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let db = state.surrealdb;

    // This is a bit straight to the point. Potentially, we first
    // query the record, then remove it based on its id. That way,
    // we can potentially find records with duplicated UUID (not likely)
    // and avoid removing both.
    let response = db
        .query("DELETE FROM todo WHERE uuid = $uuid")
        .bind(("uuid", uuid))
        .await?;

    info!("{:?}", response);

    Ok((StatusCode::OK, Json({})))
}

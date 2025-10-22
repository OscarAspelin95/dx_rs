use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::schema::ToDoItem;
use crate::state::ConnectionState;
use crate::{errors::ApiError, schema::Status};
use log::info;

use serde_json::json;

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

pub async fn remove_task(
    State(state): State<ConnectionState>,
    Path(uuid): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let db = state.surrealdb;

    let response = db
        .query("DELETE FROM todo WHERE uuid = $uuid")
        .bind(json!({"uuid":uuid}))
        .await?;

    info!("{:?}", response);

    Ok((StatusCode::OK, Json({})))
}

pub async fn remove_all_tasks(
    State(state): State<ConnectionState>,
) -> Result<impl IntoResponse, ApiError> {
    let db = state.surrealdb;

    let response: Vec<ToDoItem> = db.delete("todo").await?;
    info!("{:?}", response);

    Ok((StatusCode::OK, Json({})))
}

/// We can do this better through a single atomic query.
/// Something like "UPDATE todo SET status = IF status = 'completed' then 'created' else 'completed' END WHERE uuid..."
pub async fn toggle_task(
    State(state): State<ConnectionState>,
    Path(uuid): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let db = state.surrealdb;

    // Get the todo item from db.
    // Realistically, we only need the status column, not the entire object.
    let mut response = db
        .query("SELECT * FROM todo WHERE uuid = $uuid")
        .bind(json!({"uuid": uuid}))
        .await?;
    info!("{:?}", response);

    let task: Option<ToDoItem> = response.take(0)?;

    info!("{:?}", task);

    //
    match task {
        Some(task) => {
            // New status.
            let new_status = match task.status {
                Status::Completed => Status::Created,
                Status::Created => Status::Completed,
            };
            info!("{:?}", new_status);

            // Update db again
            let response = db
                .query("UPDATE todo SET status = $status WHERE uuid = $uuid")
                .bind(json!({"status": new_status, "uuid": task.uuid}))
                .await?;

            info!("{:?}", response);
        }
        None => {}
    }

    Ok((StatusCode::OK, Json({})))
}

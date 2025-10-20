use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use log::{error, info};

use crate::errors::ApiError;
use crate::schema::{Person, Status, ToDoItem};
use crate::state::ConnectionState;

pub async fn create_person(
    State(state): State<ConnectionState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    let db = state.surrealdb;

    info!("Checking db health...");
    match db.health().await {
        Ok(_) => info!("Db connection healthy"),
        Err(e) => error!("{:?}", e),
    };

    // Probably move to database connection part upstream...
    info!("Checking namespace and database...");
    db.use_ns("SurrealDB")
        .use_db("SurrealDB")
        .await
        .expect("Failed to use database.");

    let p = Person {
        first_name: "Oscar".into(),
        last_name: "Aspelin".into(),
        email: "oscar.aspelin@gmail.com".into(),
    };

    // Here, create("person") creates a table name called "person".
    let person: Option<Person> = db.create("person").content(p).await?;

    let person = match person {
        Some(person) => person,
        None => {
            return Err(ApiError::DatabaseRecordCreateError(
                "Failed to insert into person table".into(),
            ));
        }
    };

    //
    Ok((StatusCode::OK, Json(person)))
}

pub async fn get_tasks(
    State(state): State<ConnectionState>,
) -> Result<impl IntoResponse, ApiError> {
    let db = state.surrealdb;

    // Probably move to database connection part upstream...
    info!("Checking namespace and database...");
    db.use_ns("SurrealDB")
        .use_db("SurrealDB")
        .await
        .expect("Failed to use database.");

    // We just select all tasks for now.
    let tasks: Vec<ToDoItem> = db.select("todo").await?;

    //
    Ok((StatusCode::OK, Json(tasks)))
}

pub async fn add_task(State(state): State<ConnectionState>) -> Result<impl IntoResponse, ApiError> {
    let db = state.surrealdb;

    // Probably move to database connection part upstream...
    info!("Checking namespace and database...");
    db.use_ns("SurrealDB")
        .use_db("SurrealDB")
        .await
        .expect("Failed to use database.");

    // Mock for now.
    let task = ToDoItem {
        name: "To Do".to_string(),
        status: Status::Created,
        task_id: 0u64,
    };

    let response: Option<ToDoItem> = db.create("todo").content(task).await?;

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

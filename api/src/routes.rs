use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use log::{error, info};

use crate::errors::ApiError;
use crate::schema::Person;
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
        None => return Err(ApiError::DatabaseRecordCreateError("".into())),
    };

    //
    Ok((StatusCode::OK, Json(person)))
}

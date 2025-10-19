use std::time::Duration;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
};
use log::{error, info};
use simple_logger::SimpleLogger;
use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};
use thiserror::Error;
use tokio::{self, net::TcpListener};
use tower_http::cors::{Any, CorsLayer};

#[derive(Error, Debug)]
enum CustomError {
    #[error("Some Error")]
    SomeError(String),
}

#[derive(Debug, Clone)]
pub struct ConnectionState {
    surrealdb: Surreal<Client>,
}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> Self {
        return self::CustomError::SomeError(err.kind().to_string());
    }
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CustomError::SomeError(e) => (StatusCode::BAD_REQUEST, format!("{}", e)),
        };

        (status, error_message).into_response()
    }
}

/// Move to "router.rs" later on.
async fn create_person(
    State(state): State<ConnectionState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, CustomError> {
    info!("{id}");

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

    // Here, we define a random table to test query...
    info!("Running test query...");
    db.query(
        "
        DEFINE TABLE IF NOT EXISTS person SCHEMALESS;
        DEFINE FIELD IF NOT EXISTS name ON TABLE person TYPE string;
        DEFINE INDEX IF NOT EXISTS unique_name ON TABLE user FIELDS name UNIQUE;
        DEFINE ACCESS IF NOT EXISTS account ON DATABASE TYPE RECORD
        ",
    )
    .await
    .expect("Query failed");

    Ok((StatusCode::OK, Json("{}")))
}

fn app(state: ConnectionState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(false);

    let router = Router::new()
        .route("/create_person/{:id}", get(create_person))
        .layer(cors)
        .with_state(state);

    return router;
}

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    SimpleLogger::new().init().expect("");

    let db: Surreal<Client> = loop {
        info!("Attempting to connect to ws://db:8000");

        match Surreal::new::<Ws>("db:8000").await {
            Ok(db) => {
                info!("Connected successfully!");
                break db;
            }
            Err(e) => {
                info!("Connection failed: {:?}", e);
                info!("Retrying in 5 seconds...");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    };

    info!("Signing in.");
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("Failed to sign in");

    let state = ConnectionState { surrealdb: db };
    let app = app(state);
    let listener = TcpListener::bind("0.0.0.0:8001").await?;

    axum::serve(listener, app).await?;
    Ok(())
}

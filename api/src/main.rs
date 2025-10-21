use axum::{
    Router,
    http::Method,
    routing::{delete, get, post},
};
use simple_logger::SimpleLogger;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod state;
use state::ConnectionState;

mod errors;
use errors::ApiError;

mod routes;
use routes::get_tasks;

mod connection;
use connection::connect_db;

use crate::routes::{add_task, remove_task};

mod schema;

fn app(state: ConnectionState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::DELETE, Method::POST])
        .allow_credentials(false);

    // Move to separate mods later on...
    let router = Router::new()
        .route("/tasks", get(get_tasks))
        .route("/add_task", post(add_task))
        .route("/remove_task/{:uuid}", delete(remove_task))
        .layer(cors)
        .with_state(state);

    return router;
}

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    SimpleLogger::new()
        .init()
        .expect("Failed to initialize simple logger.");

    let db = connect_db(3).await?;
    let state = ConnectionState { surrealdb: db };
    let app = app(state);
    let listener = TcpListener::bind("0.0.0.0:8001").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

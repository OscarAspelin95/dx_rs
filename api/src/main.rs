use axum::Router;
use axum::http::Method;
use simple_logger::SimpleLogger;
use tokio::net::TcpListener;
mod state;
use state::ConnectionState;

mod errors;
use errors::ApiError;

mod connection;
use connection::connect_db;
use tower_http::cors::{Any, CorsLayer};

mod routes;
mod schema;

fn app(state: ConnectionState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
        .allow_credentials(false);

    routes::create_routes(state).layer(cors)
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

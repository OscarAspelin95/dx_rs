use axum::Router;
use axum::http::Method;
use simple_logger::SimpleLogger;
use tokio::net::TcpListener;

mod auth;
mod minio_upload;
mod nats;
mod routes;
mod schema;

mod state;
use state::ConnectionState;

mod errors;
use errors::ApiError;

use shared::database::connect_db;
use shared::minio::connect_minio;
use shared::nats::connect_nats;

use tower_http::cors::{Any, CorsLayer};

use crate::state::{Environment, MinIO, Nats, SurrealDB};

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
    let minio = connect_minio().await?;
    let nats = connect_nats().await?;

    let state = ConnectionState {
        surrealdb: SurrealDB { client: db },
        minio: MinIO { client: minio },
        nats: Nats { client: nats },
        environment: Environment {
            google_client_id: std::env::var("GOOGLE_CLIENT_ID")?,
            google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET")?,
            google_redirect_url: std::env::var("GOOGLE_REDIRECT_URL")?,
            jwt_secret: std::env::var("JWT_SECRET")?,
        },
    };

    let app = app(state);
    let listener = TcpListener::bind("0.0.0.0:8001").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

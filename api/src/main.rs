use std::time::Duration;

use axum::{Router, http::Method, routing::get};
use log::info;
use simple_logger::SimpleLogger;
use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};
use tokio::{self, net::TcpListener};
use tower_http::cors::{Any, CorsLayer};

mod state;
use state::ConnectionState;

mod errors;
use errors::ApiError;

mod routes;
use routes::create_person;

mod schema;

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

async fn connect_db(max_retries: usize) -> Result<Surreal<Client>, ApiError> {
    let mut retries: usize = 0;

    let db: Surreal<Client> = loop {
        info!("Attempting to connect to ws://db:8000");

        if retries > max_retries {
            return Err(ApiError::DatabaseConnectionTimeoutError());
        }

        match Surreal::new::<Ws>("db:8000").await {
            Ok(db) => {
                info!("Connected successfully after {} retries", retries);
                break db;
            }
            Err(e) => {
                info!("Connection failed: {:?}", e);
                retries += 1;

                // Exponential (linear) backoff.
                let wait_time: u64 = 5 * retries as u64;
                info!("Retrying in {} seconds...", wait_time);
                tokio::time::sleep(Duration::from_secs(wait_time)).await;
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

    Ok(db)
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

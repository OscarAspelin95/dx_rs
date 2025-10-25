use log::info;
use minio::s3::{Client as MinioClient, ClientBuilder, creds::StaticProvider};
use std::time::Duration;
use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};

use crate::errors::ApiError;
use crate::nats::create_streams;
use async_nats::Client as NatsClient;
use async_nats::jetstream::Context as NatsContext;

pub async fn connect_db(max_retries: usize) -> Result<Surreal<Client>, ApiError> {
    let mut retries: usize = 0;

    let db: Surreal<Client> = loop {
        // Move to .env later on.
        info!("Attempting to connect to SurrealDB endpoint...");

        if retries > max_retries {
            return Err(ApiError::DatabaseConnectionTimeoutError());
        }

        match Surreal::new::<Ws>(std::env::var("SURREALDB_ENDPOINT")?).await {
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
        username: &std::env::var("ROOT_USERNAME")?,
        password: &std::env::var("ROOT_PASSWORD")?,
    })
    .await
    .expect("Failed to sign in");

    info!("Searching for namespace and main database...");
    db.use_ns(&std::env::var("SURREALDB_NAMESPACE")?)
        .use_db(&std::env::var("SURREALDB_DBNAME")?)
        .await
        .expect("Failed to find namespace and database");

    info!("Checking db health...");
    match db.health().await {
        Ok(_) => info!("Db connection healthy"),
        Err(e) => return Err(ApiError::DatabaseUnhealthyError(e.to_string())),
    };

    Ok(db)
}

pub async fn connect_minio() -> Result<MinioClient, ApiError> {
    let static_provider = StaticProvider::new(
        &std::env::var("MINIO_ROOT_USER").unwrap(),
        &std::env::var("MINIO_ROOT_PASSWORD").unwrap(),
        None,
    );

    let client = ClientBuilder::new(
        std::env::var("MINIO_HTTP_ENDPOINT")
            .expect("Missing Minio HTTP Endpoint in environment.")
            .parse()
            .expect("Failed to parse minio base url"),
    )
    .provider(Some(Box::new(static_provider)))
    .build();

    match client {
        Ok(client) => Ok(client),
        Err(e) => Err(ApiError::SomeError(e.to_string())),
    }
}

pub async fn connect_nats() -> Result<NatsContext, ApiError> {
    let nats_client: NatsClient = async_nats::connect(&std::env::var("NATS_URL")?).await?;

    // Enable jetstream.
    let jetstream = async_nats::jetstream::new(nats_client);

    // Create the different streams we need.
    let _ = create_streams(&jetstream).await?;

    Ok(jetstream)
}

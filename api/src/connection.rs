use log::info;
use std::time::Duration;
use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};

use crate::errors::ApiError;

pub async fn connect_db(max_retries: usize) -> Result<Surreal<Client>, ApiError> {
    let mut retries: usize = 0;

    let db: Surreal<Client> = loop {
        // Move to .env later on.
        info!("Attempting to connect to ws://db:8000");

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

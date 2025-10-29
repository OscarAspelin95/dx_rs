use minio::s3;
use surrealdb::{Surreal, engine::remote::ws::Client};

#[derive(Debug, Clone)]
pub struct Environment {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_url: String,
    pub jwt_secret: String,
}

#[derive(Debug, Clone)]
pub struct SurrealDB {
    pub client: Surreal<Client>,
}

#[derive(Debug, Clone)]
pub struct MinIO {
    pub client: s3::Client,
}

#[derive(Debug, Clone)]
pub struct Nats {
    pub client: async_nats::jetstream::Context,
}

#[derive(Debug, Clone)]
pub struct ConnectionState {
    pub surrealdb: SurrealDB,
    pub minio: MinIO,
    pub nats: Nats,
    pub environment: Environment,
}

use minio::s3;
use surrealdb::{Surreal, engine::remote::ws::Client};

#[derive(Debug, Clone)]
pub struct ConnectionState {
    pub surrealdb: Surreal<Client>,
    pub minio: s3::Client,
    pub nats: async_nats::jetstream::Context,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_url: String,
    pub jwt_secret: String,
}

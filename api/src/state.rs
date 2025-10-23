use minio::s3;
use surrealdb::{Surreal, engine::remote::ws::Client};

#[derive(Debug, Clone)]
pub struct ConnectionState {
    pub surrealdb: Surreal<Client>,
    pub minio: s3::Client,
}

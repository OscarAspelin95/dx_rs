use surrealdb::{Surreal, engine::remote::ws::Client};

#[derive(Debug, Clone)]
pub struct ConnectionState {
    pub surrealdb: Surreal<Client>,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NatsMessage {
    pub url: String,
}

#[derive(Debug, PartialEq)]
pub struct MinIOStructuredUrl {
    pub bucket: String,
    pub key: String,
}

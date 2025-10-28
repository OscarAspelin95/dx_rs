use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadField {
    pub file_name: String,
    pub url: String,
    pub created_at: String,
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NatsMessage {
    pub url: String,
}

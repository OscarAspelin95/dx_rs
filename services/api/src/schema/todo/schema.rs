use std::fmt::Debug;

use serde::{Deserialize, Serialize};

// Move to shared/database/schema?
// However, this is only used in the API.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "created")]
    Created,
}

// Move to shared/database/schema?
// However, this is only used in the API.
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ToDoItem {
    pub name: String,
    pub label: String, // NOTE - this should match frontend (currently, does not).
    pub status: Status,
    pub uuid: String,
}

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "created")]
    Created,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ToDoItem {
    pub name: String,
    pub status: Status,
    pub uuid: String,
}

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "created")]
    Created,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ToDoItem {
    pub name: String,
    pub status: Status,
    pub task_id: u64,
}

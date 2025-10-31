use serde::{Deserialize, Serialize};
use strum;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "created")]
    Created,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Serialize,
    Deserialize,
    strum::EnumCount,
    strum::EnumIter,
    strum::Display,
)]
pub enum Label {
    #[serde(rename = "database")]
    #[strum(serialize = "Database")]
    Database,

    #[serde(rename = "dioxus")]
    #[strum(serialize = "Dioxus")]
    Dioxus,

    #[serde(rename = "api")]
    #[strum(serialize = "Api")]
    Api,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToDoItem {
    pub name: String,
    pub label: Option<Label>,
    pub status: Status,
    pub uuid: String,
}

impl ToDoItem {
    /// Can we run the db api query here?
    pub fn toggle(&mut self) {
        match self.status {
            Status::Completed => self.status = Status::Created,
            Status::Created => self.status = Status::Completed,
        }
    }
}

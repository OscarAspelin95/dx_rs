use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

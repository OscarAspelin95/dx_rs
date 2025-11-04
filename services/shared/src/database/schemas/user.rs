use crate::database::schemas::common::SimpleRecordId;
use crate::utils::time::time_now;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<SimpleRecordId>,
    #[serde(flatten)]
    pub data: UserData,
}

impl User {
    pub fn mock() -> Self {
        Self {
            id: None,
            data: UserData {
                first_name: "Oscar".into(),
                last_name: "Aspelin".into(),
                email: "some_email@gmail.com".into(),
                created_at: time_now(),
            },
        }
    }
}

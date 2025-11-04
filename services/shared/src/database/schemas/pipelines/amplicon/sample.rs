use crate::database::schemas::common::SimpleRecordId;
use crate::schema::schema::Status;
use crate::utils::time::time_now;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AmpliconData {
    pub status: Status,
    pub created_at: String,
    pub updated_at: String,
}

impl AmpliconData {
    fn mock() -> Self {
        Self {
            status: Status::Created,
            created_at: time_now(),
            updated_at: time_now(),
        }
    }
}

/// This is essentially only a placeholder.
/// The actual results are stored in AmpliconJob.
#[derive(Serialize, Deserialize, Debug)]
pub struct AmpliconSample {
    pub id: Option<SimpleRecordId>,
    #[serde(flatten)]
    pub data: AmpliconData,
}

impl AmpliconSample {
    pub fn mock() -> Self {
        Self {
            id: None,
            data: AmpliconData::mock(),
        }
    }
}

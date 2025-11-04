use serde::{Deserialize, Serialize};

use crate::database::schemas::common::SimpleRecordId;
use crate::utils::time::time_now;

use crate::schema::schema::{Pipeline, Status};

#[derive(Serialize, Deserialize, Debug)]
pub struct FastqSampleConfig {
    pub min_len: usize,
    pub max_len: Option<usize>,
    pub min_phred: usize,
}

impl FastqSampleConfig {
    fn mock() -> Self {
        Self {
            min_len: 200,
            max_len: None,
            min_phred: 15,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FastqSampleData {
    pub name: String,
    pub status: Status,
    pub url: String,
    pub pipeline: Pipeline,
    pub config: FastqSampleConfig,
    pub created_at: String,
    pub updated_at: String,
}

impl FastqSampleData {
    fn mock() -> Self {
        Self {
            name: "sample_name".into(),
            status: Status::Created,
            url: "http://minio:9000/bucket/key".into(),
            pipeline: Pipeline::AmpliconMetgenome,
            config: FastqSampleConfig::mock(),
            created_at: time_now(),
            updated_at: time_now(),
        }
    }
}

/// Practically, we'd relate a user to this database record.
/// something like person->uploaded->sample.
#[derive(Serialize, Deserialize, Debug)]
pub struct FastqSample {
    pub id: Option<SimpleRecordId>,
    #[serde(flatten)]
    pub data: FastqSampleData,
}

impl FastqSample {
    pub fn mock() -> Self {
        Self {
            id: None,
            data: FastqSampleData::mock(),
        }
    }
}

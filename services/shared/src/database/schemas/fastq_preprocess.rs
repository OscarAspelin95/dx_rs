use serde::{Deserialize, Serialize};

use crate::database::schemas::common::SimpleRecordId;
use crate::utils::time::time_now;

use crate::schema::schema::Status;

#[derive(Debug, Serialize, Deserialize)]
pub struct FastqMetrics {
    pub num_reads: usize,
    pub num_bases: usize,
    pub mean_error: f64,
    pub mean_phred: u8,
    pub mean_len: usize,
    pub shortest: Option<Vec<usize>>,
    pub longest: Option<Vec<usize>>,
}

impl FastqMetrics {
    fn mock() -> Self {
        Self {
            num_reads: 0,
            num_bases: 0,
            mean_error: 0.0f64,
            mean_phred: 0,
            mean_len: 0,
            shortest: None,
            longest: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FastqPreprocessResult {
    pub metrics_raw: FastqMetrics,
    pub metrics_filtered: FastqMetrics,
}

impl FastqPreprocessResult {
    fn mock() -> Self {
        Self {
            metrics_raw: FastqMetrics::mock(),
            metrics_filtered: FastqMetrics::mock(),
        }
    }
}

// -------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub struct FastqPreprocessData {
    pub status: Status,
    pub url: String,
    pub runtime: usize,
    pub result: FastqPreprocessResult,
    pub created_at: String,
    pub updated_at: String,
}

impl FastqPreprocessData {
    fn mock() -> Self {
        Self {
            status: Status::Created,
            url: "http://minio:9000/bucket/preprocessed_key".into(),
            runtime: 0,
            result: FastqPreprocessResult::mock(),
            created_at: time_now(),
            updated_at: time_now(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FastqPreprocess {
    pub id: Option<SimpleRecordId>,
    #[serde(flatten)]
    pub data: FastqPreprocessData,
}

impl FastqPreprocess {
    pub fn mock() -> Self {
        Self {
            id: None,
            data: FastqPreprocessData::mock(),
        }
    }
}

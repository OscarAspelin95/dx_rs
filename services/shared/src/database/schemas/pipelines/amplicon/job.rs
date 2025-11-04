use crate::database::schemas::common::SimpleRecordId;
use crate::schema::schema::Status;
use crate::utils::time::time_now;
use serde::{Deserialize, Serialize};

// Not sure how this serializes/deserializes...
#[derive(Serialize, Deserialize, Debug)]
pub enum Taxonomy {
    Species(String),
    Genus(String),
    Family(String),
    Order(String),
    Class(String),
    Phylum(String),
    Domain(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Result {
    taxonomy: Taxonomy,
    abundance: f64,
    reads: usize,
}

impl Result {
    fn mock() -> Self {
        Self {
            taxonomy: Taxonomy::Species("Escherichia coli".into()),
            abundance: 0.0,
            reads: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmpliconJobData {
    pub status: Status,
    pub runtime: usize,
    pub result: Vec<Result>,
    pub created_at: String,
    pub updated_at: String,
}

impl AmpliconJobData {
    fn mock() -> Self {
        Self {
            status: Status::Created,
            runtime: 0,
            result: vec![Result::mock()],
            created_at: time_now(),
            updated_at: time_now(),
        }
    }
}

/// This is essentially only a placeholder.
/// The actual results are stored in AmpliconJob.
#[derive(Serialize, Deserialize, Debug)]
pub struct AmpliconJob {
    pub id: Option<SimpleRecordId>,
    #[serde(flatten)]
    pub data: AmpliconJobData,
}

impl AmpliconJob {
    pub fn mock() -> Self {
        Self {
            id: None,
            data: AmpliconJobData::mock(),
        }
    }
}

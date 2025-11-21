use serde::{Deserialize, Serialize};

use crate::database::schemas::common::SimpleRecordId;

// What we publish/consume from NATS.
#[derive(Debug, Serialize, Deserialize)]
pub struct FastqMessage {
    pub url: String,
    pub fastq_sample_id: SimpleRecordId,
}

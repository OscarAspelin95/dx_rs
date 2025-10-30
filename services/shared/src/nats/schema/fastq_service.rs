use serde::{Deserialize, Serialize};
use serde_json;
use std::io::BufReader;
use std::path::PathBuf;

use crate::nats::NatsError;

// What we publish/consume from NATS.
#[derive(Debug, Serialize, Deserialize)]
pub struct FastqMessage {
    pub url: String,
}

/// For now, copied from fastq_rs source code.
/// Results from a single fastq_rs stats run.
#[derive(Debug, Serialize, Deserialize)]
pub struct FastqStats {
    pub num_reads: usize,
    pub num_bases: usize,
    pub mean_error: f64,
    pub mean_phred: u8,
    pub mean_len: usize,
    pub shortest: Option<Vec<usize>>,
    pub longest: Option<Vec<usize>>,
}

impl FastqStats {
    pub fn from_json(json: PathBuf) -> Result<Self, NatsError> {
        let f = std::fs::File::open(json)?;
        let bufread = BufReader::new(f);
        let s: FastqStats = serde_json::from_reader(bufread)?;

        Ok(s)
    }
}

// Result after running fastq_rs stats->filter->stats.
#[derive(Debug, Serialize, Deserialize)]
pub struct FastqMetrics {
    pub metrics_raw: FastqStats,
    pub metrics_filtered: FastqStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FastqResponse {
    pub metrics: FastqMetrics,
    pub runtime: u64,
    pub url: String,
}

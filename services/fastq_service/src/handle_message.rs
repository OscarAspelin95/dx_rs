use crate::errors::ProcessorError;
use std::path::{Path, PathBuf};

use crate::config::FilterConfig;
use fastq_rs::filter::fastq_filter;

pub fn handle_message(file: &Path) -> Result<(), ProcessorError> {
    let outfile = PathBuf::from("filtered.fastq.gz");

    let fastq = file.to_path_buf();

    let cfg = FilterConfig::default();

    let filter_result = fastq_filter(
        Some(fastq),
        cfg.min_len,
        cfg.max_len,
        cfg.min_error,
        cfg.max_error,
        cfg.min_softmasked,
        cfg.max_softmasked,
        cfg.min_ambiguous,
        cfg.max_ambiguous,
        Some(outfile),
    );

    match filter_result {
        Ok(()) => Ok(()),
        Err(e) => Err(ProcessorError::FastqRsFilterError(e.to_string())),
    }
}

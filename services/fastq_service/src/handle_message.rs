use crate::errors::FastqError;
use std::path::{Path, PathBuf};
use std::time;

use crate::config::FilterConfig;
use fastq_rs::{filter::fastq_filter, stats::fastq_stats};
use log::info;
use minio::s3::Client;
use shared::file_path;
use shared::minio::minio_upload_file;
use shared::nats::schema::fastq_service::{FastqMetrics, FastqStats};
use shared::utils::file::file_name;

fn fastq_rs_filter(fastq: &Path, outfile: &PathBuf) -> Result<(), FastqError> {
    let cfg = FilterConfig::default();

    let filter_result = fastq_filter(
        Some(fastq.to_path_buf()),
        cfg.min_len,
        cfg.max_len,
        cfg.min_error,
        cfg.max_error,
        cfg.min_softmasked,
        cfg.max_softmasked,
        cfg.min_ambiguous,
        cfg.max_ambiguous,
        Some(outfile.to_path_buf()),
    );

    match filter_result {
        Ok(()) => Ok(()),
        Err(e) => Err(FastqError::FastqRsFilterError(e.to_string())),
    }
}

fn fastq_rs_stats(fastq: &Path, outfile: PathBuf) -> Result<(), FastqError> {
    let stats_result = fastq_stats(Some(fastq.to_path_buf()), Some(outfile));

    match stats_result {
        Ok(()) => Ok(()),
        Err(err) => Err(FastqError::FastqRsError(err.to_string())),
    }
}

/// We can streamline this later on.
/// * Break into separate functions?
/// * Consider using a temp dir that is removed once going out of scope.
pub async fn handle_message(
    fastq: &Path,
    minio_client: &Client,
) -> Result<(FastqMetrics, u64, String), FastqError> {
    let start = time::Instant::now();

    // Stats for raw fastq.
    info!("Running stats on raw fastq...");
    let json_raw = file_path!("/tmp", "raw", "stats.json");
    fastq_rs_stats(fastq, json_raw.clone())?;

    // Filter fastq.
    info!("Running fastq filter...");
    let filtered_fastq = file_path!("/tmp", "trimmed", "trimmed.fastq.gz");
    fastq_rs_filter(fastq, &filtered_fastq)?;

    // Stats for filtered fastq.
    info!("Running stats on filtered fastq...");
    let json_trimmed = file_path!("/tmp", "trimmed", "stats.json");
    fastq_rs_stats(&filtered_fastq, json_trimmed.clone())?;

    let elapsed = start.elapsed().as_secs();

    // Upload file to MinIO
    let key = file_name(filtered_fastq.clone());

    let minio_url =
        minio_upload_file(minio_client, "file-upload-processed", &key, filtered_fastq).await?;

    // Construct FastqResponse
    let fastq_metrics = FastqMetrics {
        metrics_raw: FastqStats::from_json(json_raw)?,
        metrics_filtered: FastqStats::from_json(json_trimmed)?,
    };

    Ok((fastq_metrics, elapsed, minio_url))
}

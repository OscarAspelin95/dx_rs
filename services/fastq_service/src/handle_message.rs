use crate::errors::FastqError;
use std::io::Read;
use std::time;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use crate::config::FilterConfig;
use bytes::Bytes;
use fastq_rs::{filter::fastq_filter, stats::fastq_stats};
use log::info;
use minio::s3::Client;
use minio::s3::segmented_bytes::SegmentedBytes;
use shared::minio::minio_upload;
use shared::nats::schema::fastq_service::{FastqMetrics, FastqStats};

fn fastq_rs_filter(fastq: &Path, outdir: &PathBuf) -> Result<PathBuf, FastqError> {
    let cfg = FilterConfig::default();

    let filtered_fastq = outdir.join("filtered.fastq.gz");

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
        Some(filtered_fastq.clone()),
    );

    match filter_result {
        Ok(()) => Ok(filtered_fastq),
        Err(e) => Err(FastqError::FastqRsFilterError(e.to_string())),
    }
}

fn fastq_rs_stats(fastq: &Path, outdir: &PathBuf) -> Result<PathBuf, FastqError> {
    let stats_json = outdir.join("stats.json");

    let stats_result = fastq_stats(Some(fastq.to_path_buf()), Some(stats_json.clone()));

    match stats_result {
        Ok(()) => Ok(stats_json),
        Err(err) => Err(FastqError::FastqRsError(err.to_string())),
    }
}

/// We can streamline this later on.
/// * Add better ergonomics for creating outdir (separate function?).
/// * Break into separate functions.
pub async fn handle_message(
    fastq: &Path,
    minio_client: &Client,
) -> Result<(FastqMetrics, u64, String), FastqError> {
    let start = time::Instant::now();

    // Stats for raw fastq.
    info!("Running stats on raw fastq...");
    let stats_raw_dir = PathBuf::from("raw");
    create_dir_all(&stats_raw_dir)?;
    let stats_raw = fastq_rs_stats(fastq, &stats_raw_dir)?;
    assert!(stats_raw.exists());

    // Filter fastq.
    info!("Running fastq filter...");
    let stats_filtered_dir = PathBuf::from("filtered");
    create_dir_all(&stats_filtered_dir)?;
    let filtered_fastq = fastq_rs_filter(fastq, &stats_filtered_dir)?;
    assert!(&filtered_fastq.exists());

    // Stats for filtered fastq.
    info!("Running stats on filtered fastq...");
    let stats_filtered = fastq_rs_stats(fastq, &stats_filtered_dir)?;
    assert!(stats_filtered.exists());

    let elapsed = start.elapsed().as_secs();

    // Upload file to MinIO
    let key = filtered_fastq
        .file_name()
        .expect("Invalid file name")
        .to_str()
        .expect("Invalid file name");

    // Later, we'll create two shared functions:
    // * minio_upload_bytes
    // * minio_upload_file (which wraps minio_upload_bytes).

    // Because, it turns out it is rather difficult to convert
    // Bytes<File> to Segmented bytes.

    // Open file.
    let mut file_handle = std::fs::File::open(&filtered_fastq)?;

    // Read into buffer.
    // Later, we probably don't want to read entire file at once.
    let mut buf: Vec<u8> = Vec::new();
    file_handle.read_to_end(&mut buf)?;

    // Convert Vec<u8> -> Vec<Vec<Bytes>>
    // Note sure about this because MinIO expects 5Mb chunks
    // for all chunks except for the last one.
    let b = Bytes::from_owner(buf);
    let segbuf: SegmentedBytes = b.into();

    let minio_url = minio_upload(minio_client, "file-upload-processed", key, segbuf).await?;

    // Construct FastqResponse
    let fastq_metrics = FastqMetrics {
        metrics_raw: FastqStats::from_json(stats_raw)?,
        metrics_filtered: FastqStats::from_json(stats_filtered)?,
    };

    Ok((fastq_metrics, elapsed, minio_url))
}

use std::path::Path;

use ::minio::s3::types::S3Api;
use async_nats::jetstream::AckKind;
use futures::StreamExt;
use log::{error, info};
use simple_logger::SimpleLogger;
use tokio;

mod nats;
use nats::connect_nats;

mod minio;
use minio::connect_minio;

mod errors;

mod utils;
use utils::parse_url;

mod schema;
use schema::NatsMessage;

mod handle_message;
use handle_message::handle_message;

mod config;

/// Entrypoint - check for messages that are put on the NATS consumer queue.
#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .init()
        .expect("Failed to initialize logger");
    info!("Inside file processor.");

    let consumer = connect_nats().await;
    let minio_client = connect_minio().await.expect("Failed to get MinIO.");

    // Get file processor consumer.
    info!("Getting messages...");

    let mut messages = consumer
        .messages()
        .await
        .expect("Failed to get consumer messages");

    info!("Ready to accept messages...");
    while let Some(message) = messages.next().await {
        let message = match message {
            Ok(message) => {
                info!("Got message!");
                message
            }
            Err(e) => {
                error!("Got invalid message: {:?}", e);
                continue;
            }
        };

        //
        let nats_message = serde_json::from_slice::<NatsMessage>(&message.payload)
            .expect("Failed to parse payload");
        info!("{:?}", nats_message);

        // Parse url
        let parsed_url = match parse_url(&nats_message.url) {
            Some(parsed_url) => parsed_url,
            None => continue,
        };

        info!("Parsed url: {:?}", parsed_url);

        // Download file with MinIO...
        let minio_response = minio_client
            .get_object(parsed_url.bucket, parsed_url.key)
            .send()
            .await
            .expect("");

        let file_path = Path::new("temp_file.fastq.gz");
        let _ = minio_response
            .content
            .to_file(file_path)
            .await
            .expect("Failed to write contents to file");

        assert!(file_path.exists());
        info!("Successfully downloaded file to {:?}", file_path);

        // Do actual work...
        // Later on, return filtered file so we can upload to MinIO.
        let handle_result = handle_message(file_path);

        // Acknowledge message...
        match handle_result {
            Ok(()) => message.ack().await.expect("Failed to ack message."),
            Err(e) => {
                error!("{:?}", e);
                message
                    .ack_with(AckKind::Nak(None))
                    .await
                    .expect("Failed to nack message.");
            }
        }
    }
}

use std::path::PathBuf;

use async_nats::jetstream::AckKind;
use futures::StreamExt;
use log::{error, info};
use simple_logger::SimpleLogger;
use tokio;

use shared::minio::{connect_minio, minio_download};
use shared::nats::connect_nats;
use shared::nats::consumer::get_consumer;
use shared::nats::schema::fastq_service::FastqMessage;

mod handle_message;
use handle_message::handle_message;

mod config;
mod errors;

/// Entrypoint - check for messages that are put on the NATS consumer queue.
#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .init()
        .expect("Failed to initialize logger");
    info!("Inside file processor.");

    let jetstream = connect_nats().await.expect("Failed to connect to NATS");

    let consumer = get_consumer(&jetstream, "file-uploaded", "file-uploaded-process")
        .await
        .expect("Failed to get consumer");

    //
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

        // Parse payload bytes.
        // Consider error handling with continue statement.
        let nats_message = serde_json::from_slice::<FastqMessage>(&message.payload)
            .expect("Failed to parse payload");
        info!("{:?}", nats_message);

        // Download file.
        let file_path = PathBuf::from("test.fastq.gz");
        let download_response = minio_download(&minio_client, &nats_message.url, &file_path).await;

        match download_response {
            Ok(local_file) => {}
            Err(e) => {
                error!("{}", e);
                continue;
            }
        }

        // Do actual work...
        // Later on, return filtered file so we can upload to MinIO.
        let handle_result = handle_message(&file_path);

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

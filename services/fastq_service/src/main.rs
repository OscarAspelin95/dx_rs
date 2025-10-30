use std::path::PathBuf;

use async_nats::jetstream::AckKind;
use futures::StreamExt;
use log::{error, info};
use shared::database::connect_db;
use simple_logger::SimpleLogger;
use tokio;

use shared::minio::{connect_minio, minio_download};
use shared::nats::connect_nats;
use shared::nats::schema::fastq_service::{FastqMessage, FastqResponse};
use shared::nats::streams::{config::StreamType, stream::get_consumer_from_stream_type};

mod handle_message;
use handle_message::handle_message;

use crate::database::write_to_db;
use crate::errors::FastqError;

mod config;
mod database;
mod errors;

/// Entrypoint - check for messages that are put on the NATS consumer queue.
#[tokio::main]
async fn main() -> Result<(), FastqError> {
    info!("Inside fastq service.");
    SimpleLogger::new().init()?;

    // Get connections and clients.
    info!("Setting up connections...");
    let jetstream = connect_nats().await?;
    let db = connect_db(3).await?;
    let consumer = get_consumer_from_stream_type(&jetstream, StreamType::FileUpload).await?;
    let minio_client = connect_minio().await?;

    info!("Getting messages...");
    let mut messages = consumer
        .messages()
        .await
        .map_err(|err| FastqError::GetConsumerMessagesError(err.to_string()))?;

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
        let nats_message = serde_json::from_slice::<FastqMessage>(&message.payload)?;
        info!("{:?}", nats_message);

        // Download file.
        // We can change this later on to provide a directory and the function
        // returns the file as outdir/<file_base_name>.
        let file_path = PathBuf::from("test.fastq.gz");
        minio_download(&minio_client, &nats_message.url, &file_path).await?;

        // Do actual work...
        // Later on, return filtered file so we can upload to MinIO.
        info!("Running fastq_rs filter...");
        let handle_result = handle_message(&file_path, &minio_client).await;

        // Acknowledge message...
        match handle_result {
            Ok((fastq_metrics, runtime, url)) => {
                // Write to database.

                let fastq_response = FastqResponse {
                    metrics: fastq_metrics,
                    runtime: runtime,
                    url: url,
                };

                info!("{:?}", fastq_response);
                write_to_db(fastq_response, &db).await?;
                message.ack().await.expect("Failed to ack message.")
            }
            // Something
            Err(e) => {
                error!("{:?}", e);
                message
                    .ack_with(AckKind::Nak(None))
                    .await
                    .map_err(|err| FastqError::MessageAckError(err.to_string()))?;
            }
        }
    }

    Ok(())
}

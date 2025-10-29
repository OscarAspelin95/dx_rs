use async_nats::jetstream::Context;
use shared::nats::errors::NatsError;
use shared::nats::schema::fastq_service::FastqMessage;

pub async fn nats_publish_upload(nats: Context, url: String) -> Result<(), NatsError> {
    let ack = nats
        .publish(
            "file-uploaded.process",
            serde_json::to_string(&FastqMessage { url: url })
                .expect("Failed to serialize NATS message.")
                // Fix.
                .into(),
        )
        .await?;

    ack.await?;

    Ok(())
}

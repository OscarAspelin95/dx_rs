use async_nats::jetstream::Context;
use shared::nats::errors::NatsError;
use shared::nats::schema::fastq_service::FastqMessage;
use shared::nats::streams::config::{StreamConsumerConfig, StreamType};

pub async fn nats_publish_upload(nats: Context, url: String) -> Result<(), NatsError> {
    let cfg = StreamConsumerConfig::from(StreamType::FileUpload);

    // Here, we could publish to any allowed stream subject, but for
    // convenience since we only have one consumer handling one subject,
    // we set the publishing subject be equal to the consumer filter subject.
    let ack = nats
        .publish(
            cfg.consumer.deliver_subject,
            serde_json::to_string(&FastqMessage { url: url })
                .expect("Failed to serialize NATS message.")
                // Fix.
                .into(),
        )
        .await?;

    ack.await?;

    Ok(())
}

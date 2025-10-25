use std::vec;

use async_nats::jetstream::Context as NatsContext;
use async_nats::jetstream::consumer::Consumer as NatsConsumer;
use async_nats::jetstream::consumer::push::Config as PushConsumerConfig;
use async_nats::jetstream::consumer::{AckPolicy, DeliverPolicy};
use async_nats::jetstream::stream::DiscardPolicy;
use async_nats::{self, jetstream::stream::Config};
use log::info;

use crate::errors::ApiError;

/// Basics of NATS:
/// * Stream - stores messages. We can define subjects
///     such as file-uploaded.*, which means that every published message
///     that starts with file-uploaded will end up there. E.g.,
///
///     file-uploaded.started
///     file-uploaded.done
///     file-uploaded.processing
///
/// * Consumer - Reads messages from the stream. With the filter_subject, we
///     can create a consumer that filters and only delivers a particular subject,
///     such as file-uploaded.processing. Here, we need to define a deliver_subject,
///     which is the name that a subscriber will subscribe to.
///
/// * Subscriber - The client can fetch messages from a consumer by subscribing
///     to the consumer subject. This means the consumer deliver subject and the
///     client subscription subject should match.
pub async fn create_file_upload_stream(jetstream: &NatsContext) -> Result<(), ApiError> {
    info!("Creating file upload stream...");
    let jetstream_config = Config {
        name: "file-uploaded".into(),
        max_messages: 1_000,
        subjects: vec!["file-uploaded.*".into()],
        discard: DiscardPolicy::Old,
        ..Default::default()
    };
    jetstream.get_or_create_stream(&jetstream_config).await?;

    info!("Attaching durable push consumer...");
    // We have learned some important stuff:
    // * name and durable name should both be set to the same value.
    // * No dots in consumer name, this can confuse NATS.
    let push_consumer_config = PushConsumerConfig {
        name: Some("file-uploaded-process".into()),
        durable_name: Some("file-uploaded-process".into()),
        filter_subject: "file-uploaded.process".into(),
        deliver_subject: "file-uploaded.process.deliver".into(),
        ack_policy: AckPolicy::Explicit,
        deliver_policy: DeliverPolicy::All,
        ..Default::default()
    };
    let mut push_consumer: NatsConsumer<PushConsumerConfig> = jetstream
        .create_consumer_on_stream(push_consumer_config, "file-uploaded")
        .await
        .expect("Failed to create consumer for stream.");

    info!("Consumer info:");
    let consumer_info = push_consumer
        .info()
        .await
        .expect("Failed to get consumer info.");
    info!("{:?}", consumer_info);

    Ok(())
}

pub async fn create_streams(jetstream: &NatsContext) -> Result<(), ApiError> {
    create_file_upload_stream(jetstream).await?;
    Ok(())
}

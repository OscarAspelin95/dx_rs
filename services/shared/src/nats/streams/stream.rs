use async_nats::jetstream::Context as NatsContext;
use async_nats::jetstream::consumer::Consumer as NatsConsumer;
use async_nats::jetstream::consumer::push::Config as PushConsumerConfig;
use log::info;

use crate::nats::NatsError;
use crate::nats::streams::config::{StreamConsumerConfig, StreamType};

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
///
/// Basically, for this function we:
/// * Extract out pre-defined stream and consumer setup based on the stream type.
/// * Create the stream itself.
/// * Attach a push consumer.
pub async fn create_stream_with_consumer(
    jetstream: &NatsContext,
    stream_type: StreamType,
) -> Result<(), NatsError> {
    let cfg = StreamConsumerConfig::from(stream_type);

    let stream_name = cfg.stream.name.as_str();

    info!("Creating {} stream...", stream_name);
    jetstream.get_or_create_stream(&cfg.stream).await?;

    info!("Attaching durable push consumer...");
    let mut push_consumer: NatsConsumer<PushConsumerConfig> = jetstream
        .create_consumer_on_stream(cfg.consumer, stream_name)
        .await
        .expect("Failed to create consumer for stream.");

    info!("Getting consumer info...");
    let consumer_info = push_consumer
        .info()
        .await
        .expect("Failed to get consumer info.");
    info!("Succeeded:\n{:?}", consumer_info);

    Ok(())
}

// Get an already existing stream and consumer.
pub async fn get_consumer_from_stream_type(
    jetstream: &NatsContext,
    stream_type: StreamType,
) -> Result<NatsConsumer<PushConsumerConfig>, NatsError> {
    let cfg = StreamConsumerConfig::from(stream_type);

    let stream_name = cfg.stream.name.as_str();

    // Get stream by name.
    info!("Getting stream {}", &stream_name);
    let stream = jetstream.get_stream(&stream_name).await?;

    info!("Getting consumer {:?}...", &cfg.consumer.name);
    let consumer: NatsConsumer<PushConsumerConfig> = stream
        .get_consumer(&cfg.consumer.name.expect("Consumer name does not exist."))
        .await?;

    Ok(consumer)
}

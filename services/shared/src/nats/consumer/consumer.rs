use async_nats::jetstream::Context;
use log::info;

use async_nats::jetstream::consumer::Consumer as PushConsumer;
use async_nats::jetstream::consumer::push::Config as PushConsumerConfig;

use crate::nats::NatsError;

pub async fn get_consumer(
    jetstream: &Context,
    stream_name: &str,
    consumer_name: &str,
) -> Result<PushConsumer<PushConsumerConfig>, NatsError> {
    info!("Getting stream...");
    let stream = jetstream.get_stream(stream_name).await?;

    info!("Getting consumer...");
    let consumer: PushConsumer<PushConsumerConfig> = stream.get_consumer(consumer_name).await?;

    Ok(consumer)
}

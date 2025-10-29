use crate::nats::NatsError;
use crate::nats::streams::{config::StreamType, stream::create_stream_with_consumer};
use async_nats::jetstream::Context as NatsContext;

pub async fn create_streams(jetstream: &NatsContext) -> Result<(), NatsError> {
    create_stream_with_consumer(jetstream, StreamType::FileUpload).await?;
    Ok(())
}

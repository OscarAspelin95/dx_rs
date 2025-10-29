use crate::nats::NatsError;
use crate::nats::streams::file_upload::create_file_upload_stream_with_consumer;
use async_nats::jetstream::Context as NatsContext;

pub async fn create_streams(jetstream: &NatsContext) -> Result<(), NatsError> {
    create_file_upload_stream_with_consumer(jetstream).await?;
    Ok(())
}

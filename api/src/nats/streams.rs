use crate::errors::ApiError;
use async_nats::jetstream::Context as NatsContext;

use crate::nats::file_upload::create_file_upload_stream_with_consumer;

pub async fn create_streams(jetstream: &NatsContext) -> Result<(), ApiError> {
    create_file_upload_stream_with_consumer(jetstream).await?;
    Ok(())
}

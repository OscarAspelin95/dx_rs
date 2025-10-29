use async_nats::Client as NatsClient;
use async_nats::jetstream::Context as NatsContext;

use crate::nats::errors::NatsError;
use crate::nats::streams::create_streams;

/// Get a NATS client and enable jetstream.
pub async fn connect_nats() -> Result<NatsContext, NatsError> {
    let nats_client: NatsClient = async_nats::connect(&std::env::var("NATS_URL")?).await?;

    // Enable jetstream.
    let jetstream = async_nats::jetstream::new(nats_client);

    // Create streams.
    create_streams(&jetstream).await?;

    Ok(jetstream)
}

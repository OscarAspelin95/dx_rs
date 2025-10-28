use crate::{errors::ApiError, schema::file_upload::schema::NatsMessage};
use async_nats::jetstream::Context;

pub async fn nats_publish_upload(nats: Context, url: String) -> Result<(), ApiError> {
    let ack = nats
        .publish(
            "file-uploaded.process",
            serde_json::to_string(&NatsMessage { url: url })
                .expect("Failed to serialize NATS message.")
                // Fix.
                .into(),
        )
        .await?;

    ack.await?;

    Ok(())
}

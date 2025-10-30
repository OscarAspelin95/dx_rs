use crate::errors::FastqError;
use log::info;
use shared::nats::schema::fastq_service::FastqResponse;
use surrealdb::{Surreal, engine::remote::ws::Client};

pub async fn write_to_db(
    fastq_reponse: FastqResponse,
    db: &Surreal<Client>,
) -> Result<(), FastqError> {
    let response: Option<FastqResponse> = db
        .create("file_process_result")
        .content(fastq_reponse)
        .await?;

    info!("Database response: {:?}", response);

    Ok(())
}

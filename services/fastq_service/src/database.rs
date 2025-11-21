use crate::errors::FastqError;
use log::info;
use shared::{
    database::schemas::{
        common::SimpleRecordId,
        fastq_preprocess::{FastqPreprocess, FastqPreprocessData, FastqPreprocessResult},
    },
    schema::schema::Status,
    utils::time::time_now,
};
use surrealdb::{Surreal, engine::remote::ws::Client};

pub async fn write_to_db(
    fastq_preprocess_result: FastqPreprocessResult,
    url: String,
    runtime: usize,
    fastq_sample_id: SimpleRecordId,
    db: &Surreal<Client>,
) -> Result<(), FastqError> {
    // Define our preprocess struct to write to database.
    let fastq_preprocess = FastqPreprocess {
        id: None,
        data: FastqPreprocessData {
            status: Status::Done,
            url: url,
            runtime: runtime,
            result: fastq_preprocess_result,
            created_at: time_now(),
            updated_at: time_now(),
        },
    };

    // Write actual data to database, capture response for relation.
    let preprocess_response: Option<FastqPreprocess> = db
        .create("fastq_preprocess")
        .content(fastq_preprocess)
        .await?;
    info!("Preprocess response: {:?}", preprocess_response);

    // Add relation between fastq sample and fastq preprocess.
    let relation_response = db
        .query("RELATE $fastq_sample->processed->$fastq_process")
        .bind(("fastq_sample", fastq_sample_id.surrealdb_id()?))
        .bind((
            "fastq_preprocess",
            preprocess_response
                .unwrap()
                .id
                .as_ref()
                .unwrap()
                .surrealdb_id()?,
        ))
        .await?;

    info!("Relation response: {:?}", relation_response);

    Ok(())
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "error")]
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Pipeline {
    #[serde(rename = "wgs_single_isolate")]
    WgsSingleIsolate,
    #[serde(rename = "wgs_metagenome")]
    WgsMetagenome,
    #[serde(rename = "amplicon_metagenome")]
    AmpliconMetgenome,
}

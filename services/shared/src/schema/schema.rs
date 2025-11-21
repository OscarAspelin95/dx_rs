use serde::{Deserialize, Serialize};
use strum;

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

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    strum::EnumCount,
    strum::EnumIter,
    strum::Display,
)]
pub enum Pipeline {
    #[serde(rename = "wgs_single_isolate")]
    #[strum(serialize = "WGS Single Isolate")]
    WgsSingleIsolate,
    #[serde(rename = "wgs_metagenome")]
    #[strum(serialize = "WGS Metagenome")]
    WgsMetagenome,
    #[serde(rename = "amplicon_metagenome")]
    #[strum(serialize = "Amplicon Metagenome")]
    AmpliconMetgenome,
}

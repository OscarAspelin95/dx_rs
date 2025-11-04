pub mod user;
pub use user::User;

pub mod common;
pub mod fastq_preprocess;
pub mod fastq_sample;
pub mod pipelines;

// We have a general pattern for structs that are written to our surrealdb database tables.
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Data {
//     pub id: Option<RecordId>,
//     pub data: DataStruct,
// }
//
// Where DataStruct contains the fields we want to write.
// By having id as Option<RecordId>, we can skip providing an id
// when writing to the db and get the automatically generated id
// when reading from the db.

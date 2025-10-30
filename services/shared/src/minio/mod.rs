pub mod connection;
pub use connection::connect_minio;

pub mod upload;
pub use upload::{minio_upload_bytes, minio_upload_file};

pub mod download;
pub use download::minio_download;

pub mod errors;
pub use errors::MinIoError;

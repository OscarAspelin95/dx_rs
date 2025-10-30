mod upload;
use std::time::Duration;

use axum::{Router, extract::DefaultBodyLimit, routing::post};
use tower_http::timeout::RequestBodyTimeoutLayer;
pub use upload::upload_file;

use crate::state::ConnectionState;

pub fn routes() -> Router<ConnectionState> {
    let router = Router::new()
        .route("/upload", post(upload_file))
        // For the moment, restrict .fastq.gz uploads to 250Mb
        // and the request timeout to 5 minutes per file.
        .layer(DefaultBodyLimit::max(250 * 1024 * 1024))
        .layer(RequestBodyTimeoutLayer::new(Duration::from_secs(60 * 5)));

    router
}

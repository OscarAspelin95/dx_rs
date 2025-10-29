mod upload;
use axum::{Router, routing::post};
pub use upload::upload_file;

use crate::state::ConnectionState;

pub fn routes() -> Router<ConnectionState> {
    let router = Router::new().route("/upload", post(upload_file));

    router
}

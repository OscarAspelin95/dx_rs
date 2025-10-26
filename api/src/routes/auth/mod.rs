mod auth;
pub use auth::test_auth;
use axum::Router;
use axum::routing::post;

use crate::state::ConnectionState;

pub fn router() -> Router<ConnectionState> {
    let router = Router::new().route("/test_auth", post(test_auth));

    router
}

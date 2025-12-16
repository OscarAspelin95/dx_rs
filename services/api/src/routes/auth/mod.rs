mod auth;
pub use auth::test_auth;
use axum::middleware;
use axum::routing::post;
use axum::Router;

use crate::auth::middleware::auth_middleware;
use crate::state::ConnectionState;

pub fn router() -> Router<ConnectionState> {
    Router::new()
        .route("/test_auth", post(test_auth))
        .route_layer(middleware::from_fn(auth_middleware))
}

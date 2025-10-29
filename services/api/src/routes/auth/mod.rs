mod auth;
pub use auth::test_auth;
use axum::routing::{get, post};
use axum::{Router, middleware};

use crate::auth::middleware::auth_middleware;

use crate::routes::auth::auth::{auth_google_callback, auth_google_login};

use crate::state::ConnectionState;
pub fn router() -> Router<ConnectionState> {
    let router = Router::new()
        // Test create a route protected with user authentication.
        .route("/test_auth", post(test_auth))
        .route_layer(middleware::from_fn(auth_middleware))
        // Oauth related stuff.
        .route("/auth/google/login", get(auth_google_login))
        .route("/auth/google/callback", get(auth_google_callback));
    router
}

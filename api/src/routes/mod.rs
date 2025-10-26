use crate::state::ConnectionState;
use axum::Router;

mod auth;
mod todo;
mod upload;

pub fn create_routes(state: ConnectionState) -> Router {
    // Merge all routes.
    let router = Router::new()
        .merge(todo::routes())
        .merge(upload::routes())
        .merge(auth::router())
        .with_state(state);

    router
}

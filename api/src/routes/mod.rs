use crate::state::ConnectionState;
use axum::Router;

mod todo;
mod upload;

pub fn create_routes(state: ConnectionState) -> Router {
    // Merge all routes.
    let router = Router::new()
        .merge(todo::routes())
        .merge(upload::routes())
        .with_state(state);

    router
}

mod todo;
use axum::Router;
use axum::routing::{delete, get, patch, post};
pub use todo::{add_task, get_tasks, remove_all_tasks, remove_task, toggle_task};

use crate::state::ConnectionState;

pub fn routes() -> Router<ConnectionState> {
    let router = Router::new()
        .route("/tasks", get(get_tasks))
        .route("/add_task", post(add_task))
        .route("/remove_task/{:uuid}", delete(remove_task))
        .route("/toggle_task/{:uuid}", patch(toggle_task))
        .route("/remove_all_tasks", delete(remove_all_tasks));
    router
}

use crate::{api::todos::todos_handler, AppState};
use axum::{routing::get, Router};

pub fn todos_routes(state: AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(todos_handler::get_todos)
                .post(todos_handler::create_todo)
                .with_state(state.clone()),
        )
        .route(
            "/:title",
            get(todos_handler::get_todo_by_id)
                .put(todos_handler::update_todo)
                .delete(todos_handler::delete_todo)
                .with_state(state.clone()),
        )
}

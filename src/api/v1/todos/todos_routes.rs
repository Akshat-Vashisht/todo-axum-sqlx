use crate::{api::v1::todos::todos_handler, AppState};
use axum::{routing::get, Router};

pub fn todos_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(todos_handler::hello))
        .with_state(state)
}

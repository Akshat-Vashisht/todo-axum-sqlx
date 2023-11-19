use crate::{api::v1::todos::todos_routes::todos_routes, AppState};
use axum::Router;

pub fn configure(state: AppState) -> Router {
    Router::new().nest("/todos", todos_routes(state))
}

use crate::api::todos::todos_routes::todos_routes;
use crate::AppState;
use axum::Router;

pub fn configure(state: AppState) -> Router {
    Router::new().nest("/todos", todos_routes(state))
}

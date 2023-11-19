use crate::api::v1::todos::todos_routes::todos_routes;
use axum::Router;

pub fn configure() -> Router {
    Router::new().nest("/todos", todos_routes())
}

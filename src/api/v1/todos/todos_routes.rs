use crate::api::v1::todos::todos_handler;
use axum::{routing::get, Router};

pub fn todos_routes() -> Router {
    Router::new().route("/", get(todos_handler::hello))
}

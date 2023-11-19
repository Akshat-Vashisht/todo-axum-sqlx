use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response}, extract::State,
};

use crate::AppState;

pub async fn hello(State(data): State<AppState>) -> impl IntoResponse {
    dbg!(data);
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Hello world"))
        .unwrap()
}

use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn hello() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Hello world"))
        .unwrap()
}

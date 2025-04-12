// src/api/foo/route.rs
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn get() -> impl IntoResponse {
    (StatusCode::OK, "GET test")
}

pub async fn post() -> impl IntoResponse {
    (StatusCode::OK, "POST foo")
}

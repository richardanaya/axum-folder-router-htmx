use askama::Template;
use axum::response::{Html, IntoResponse};

#[derive(Template)]
#[template(path = "ui_kitchen_sink.html")]
struct UiKitchenSink {}

pub async fn get() -> impl IntoResponse {
    Html(UiKitchenSink {}.render().unwrap()).into_response()
}

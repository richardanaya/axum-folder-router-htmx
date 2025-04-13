use askama::Template;
use axum::response::{Html, IntoResponse};

#[derive(Template)]
#[template(path = "app.html")]
struct AppTemplate {}

pub async fn get() -> impl IntoResponse {
    Html(AppTemplate {}.render().unwrap()).into_response()
}

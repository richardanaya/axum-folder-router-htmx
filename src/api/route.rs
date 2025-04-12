use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};

#[derive(Template)]
#[template(path = "landing_page.html")]
struct LandingPage {}

pub async fn get() -> impl IntoResponse {
    Html(LandingPage {}.render().unwrap()).into_response()
}

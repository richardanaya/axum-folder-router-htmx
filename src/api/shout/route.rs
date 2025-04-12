use askama::Template;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum_extra::extract::Form;
use serde::Deserialize;
use serde::Serialize;

#[derive(Template)]
#[template(path = "shout.html")]
struct Shout<'a> {
    word: &'a str,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ShoutParams {
    word: String,
}

pub async fn post(Form(params): Form<ShoutParams>) -> impl IntoResponse {
    Html(Shout { word: &params.word }.render().unwrap()).into_response()
}

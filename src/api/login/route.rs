use crate::AppState;
use axum::response::Redirect;
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar};
use axum_extra::extract::Form;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct LoginParams {
    username: String,
}

pub async fn post(
    jar: PrivateCookieJar,
    Form(params): Form<LoginParams>,
) -> (PrivateCookieJar, Redirect) {
    // In a real app, you'd validate the username/password here.
    // For this example, we just store the username.
    let updated_jar = jar.add(Cookie::new("username", params.username));
    (updated_jar, Redirect::to("/"))
}

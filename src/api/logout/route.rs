use axum::response::Redirect;
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar};

pub async fn post(jar: PrivateCookieJar) -> (PrivateCookieJar, Redirect) {
    // Remove the username cookie
    (jar.remove(Cookie::named("username")), Redirect::to("/"))
}

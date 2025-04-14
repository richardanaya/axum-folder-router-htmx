use axum::response::Redirect;
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar}; // Keep PrivateCookieJar

pub async fn post(jar: PrivateCookieJar) -> (PrivateCookieJar, Redirect) {
    // Remove the email cookie
    (jar.remove(Cookie::named("email")), Redirect::to("/")) // Change "username" to "email"
}

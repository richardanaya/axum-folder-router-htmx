use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum_extra::extract::cookie::PrivateCookieJar; // Import PrivateCookieJar

#[derive(Template)]
#[template(path = "landing_page.html")]
struct LandingPage<'a> {
    email: &'a str, // Changed from username to email
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

// Modify the get handler
pub async fn get(jar: PrivateCookieJar) -> impl IntoResponse {
    if let Some(cookie) = jar.get("email") { // Check for "email" cookie
        // User is logged in, render landing page
        let email = cookie.value().to_string(); // Get email value
        Html(
            LandingPage { email: &email } // Pass email to template
                .render()
                .unwrap(),
        )
        .into_response()
    } else {
        // User is not logged in, render login page
        Html(LoginTemplate {}.render().unwrap()).into_response()
    }
}

use crate::AppState;
use axum::{extract::State, response::Redirect};
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar};
use axum_extra::extract::Form;
use serde::Deserialize;
use sqlx::PgPool; // Import PgPool

#[derive(Deserialize, Clone)]
pub struct LoginParams {
    email: String, // Changed from username to email
}

pub async fn post(
    State(pool): State<PgPool>, // Inject the database pool
    jar: PrivateCookieJar,
    Form(params): Form<LoginParams>,
) -> (PrivateCookieJar, Redirect) {
    // Basic email validation (presence of '@') - more robust validation recommended
    if !params.email.contains('@') {
        // Redirect back to login or show an error - simple redirect for now
        // Ideally, include an error message
        return (jar, Redirect::to("/")); // Or maybe a dedicated login page route if not root
    }

    // Check if user exists, insert if not
    // Using ON CONFLICT DO NOTHING to handle potential race conditions and simplify logic
    // RETURNING id is useful if we needed the user ID later
    let insert_result = sqlx::query(
        "INSERT INTO users (email) VALUES ($1) ON CONFLICT (email) DO NOTHING"
    )
    .bind(&params.email) // Bind the parameter
    .execute(&pool)
    .await;

    // Handle potential database errors during insert/check
    if insert_result.is_err() {
        // Log the error, maybe redirect with an error message
        eprintln!("Database error during login: {:?}", insert_result.err());
        // For simplicity, redirecting home, but an error page/message is better UX
        return (jar, Redirect::to("/"));
    }

    // Store the email in the cookie
    let updated_jar = jar.add(Cookie::new("email", params.email.clone())); // Use email now
    (updated_jar, Redirect::to("/"))
}

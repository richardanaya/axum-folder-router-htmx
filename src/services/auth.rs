use crate::model::User; // Import models
use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect, Response}, // Import Response
};
use axum_extra::extract::cookie::PrivateCookieJar;
use sqlx::PgPool;

pub struct AuthService {
    value: i32,
}

impl AuthService {
    pub fn new() -> Self {
        AuthService { value: 42 }
    }

    // Helper function to get user ID from email cookie
    pub async fn get_user_from_cookie(
        &self,
        jar: &PrivateCookieJar,
        pool: &PgPool,
    ) -> Result<User, Response> {
        let email = jar
            .get("email")
            .map(|cookie| cookie.value().to_string())
            .ok_or_else(|| Redirect::to("/").into_response())?; // Redirect home if not logged in

        // Clone email here to satisfy the borrow checker for the error case below
        let email_clone_for_query = email.clone();

        sqlx::query_as::<_, User>("SELECT id, email FROM users WHERE email = $1")
            .bind(email_clone_for_query) // Bind the cloned parameter
            .fetch_optional(pool)
            .await
            .map_err(|e| {
                eprintln!("Database error fetching user: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            })?
            .ok_or_else(move || {
                // Use move closure to capture the original email
                // Should not happen if login ensures user exists, but handle defensively
                eprintln!("User not found for email in cookie: {}", email); // Use original email here
                Redirect::to("/").into_response() // Redirect home
            })
    }
}

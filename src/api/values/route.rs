use crate::model::{PersonalValue, User}; // Import models
use crate::AppState;
use askama::Template;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response}, // Import Response
};
use axum_extra::extract::cookie::PrivateCookieJar;
use serde::Deserialize;
use sqlx::PgPool;

// Template for the values page
#[derive(Template)]
#[template(path = "values.html")]
struct ValuesTemplate {
    email: String, // Pass user email to template
    values: Vec<PersonalValue>, // List of user's values
}

// Struct for the form data when adding a value
#[derive(Deserialize)]
pub struct AddValueParams {
    name: String,
    description: Option<String>,
}

// Helper function to get user ID from email cookie
async fn get_user_from_cookie(jar: &PrivateCookieJar, pool: &PgPool) -> Result<User, Response> {
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
        .ok_or_else(move || { // Use move closure to capture the original email
            // Should not happen if login ensures user exists, but handle defensively
            eprintln!("User not found for email in cookie: {}", email); // Use original email here
            Redirect::to("/").into_response() // Redirect home
        })
}

// GET handler for /values
pub async fn get(jar: PrivateCookieJar, State(pool): State<PgPool>) -> impl IntoResponse {
    let user = match get_user_from_cookie(&jar, &pool).await {
        Ok(user) => user,
        Err(response) => return response, // Return redirect or error response
    };

    // Fetch personal values for the user
    let values_result = sqlx::query_as::<_, PersonalValue>(
        "SELECT id, user_id, name, description FROM personal_values WHERE user_id = $1 ORDER BY name"
    )
    .bind(user.id) // Bind the parameter separately
    .fetch_all(&pool)
    .await;

    let values = match values_result {
        Ok(vals) => vals,
        Err(e) => {
            eprintln!("Database error fetching values: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    let template = ValuesTemplate {
        email: user.email,
        values,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Template rendering error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response()
        }
    }
}

// POST handler for /values
pub async fn post(
    jar: PrivateCookieJar,
    State(pool): State<PgPool>,
    Form(params): Form<AddValueParams>,
) -> impl IntoResponse {
    let user = match get_user_from_cookie(&jar, &pool).await {
        Ok(user) => user,
        Err(response) => return response, // Return redirect or error response
    };

    // Validate input (basic non-empty check for name)
    if params.name.trim().is_empty() {
        // Ideally, return to form with an error message
        return Redirect::to("/values").into_response();
    }

    // Insert the new value, handling potential unique constraint violation
    let insert_result = sqlx::query(
        "INSERT INTO personal_values (user_id, name, description) VALUES ($1, $2, $3) ON CONFLICT (user_id, name) DO NOTHING"
    )
    .bind(user.id)
    .bind(params.name.trim()) // Trim whitespace
    .bind(params.description.as_deref().filter(|s| !s.trim().is_empty())) // Store None if description is empty/whitespace
    .execute(&pool)
    .await;

    if let Err(e) = insert_result {
        eprintln!("Database error inserting value: {}", e);
        // Could redirect with error or render page with error
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to add value").into_response();
    }

    // Redirect back to the values page to show the updated list
    Redirect::to("/values").into_response()
}

use crate::model::{PersonalValue, User}; // Import models
use crate::services::auth_service;
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
use std::collections::HashMap; // Import HashMap

// View model for displaying values in the template
#[derive(Clone)] // Add Clone derive
struct ValueView {
    id: i32,
    name: String,
    description: Option<String>,
    parent_name: Option<String>, // Add parent name
}

// Template for the values page
#[derive(Template)]
#[template(path = "values.html")]
struct ValuesTemplate {
    email: String,
    values: Vec<ValueView>,                // Use ValueView for display
    potential_parents: Vec<PersonalValue>, // Keep PersonalValue for the dropdown
}

// Struct for the form data when adding a value
#[derive(Deserialize)]
pub struct AddValueParams {
    name: String,
    description: Option<String>,
    parent_id: Option<i32>, // Add parent_id, make it optional
}

// GET handler for /values
pub async fn get(jar: PrivateCookieJar, State(pool): State<PgPool>) -> impl IntoResponse {
    let user = match auth_service().get_user_from_cookie(&jar, &pool).await {
        Ok(user) => user,
        Err(response) => return response, // Return redirect or error response
    };

    // Fetch personal values for the user, including parent_id
    let values_result = sqlx::query_as::<_, PersonalValue>(
        "SELECT id, user_id, name, description, parent_id FROM personal_values WHERE user_id = $1 ORDER BY name"
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

    // Create a lookup map for parent names
    let id_to_name: HashMap<i32, String> = values.iter().map(|v| (v.id, v.name.clone())).collect();

    // Map PersonalValue to ValueView for display
    let value_views: Vec<ValueView> = values
        .iter()
        .map(|v| ValueView {
            id: v.id,
            name: v.name.clone(),
            description: v.description.clone(),
            parent_name: v.parent_id.and_then(|pid| id_to_name.get(&pid).cloned()), // Look up parent name
        })
        .collect();

    // Keep the original values for the parent dropdown
    let potential_parents = values;

    let template = ValuesTemplate {
        email: user.email,
        values: value_views, // Pass the view models
        potential_parents,   // Pass original values for dropdown
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
    let user = match auth_service().get_user_from_cookie(&jar, &pool).await {
        Ok(user) => user,
        Err(response) => return response, // Return redirect or error response
    };

    // Validate input (basic non-empty check for name)
    if params.name.trim().is_empty() {
        // Ideally, return to form with an error message
        return Redirect::to("/values").into_response();
    }

    // Insert the new value, including parent_id, handling potential unique constraint violation
    let insert_result = sqlx::query(
        "INSERT INTO personal_values (user_id, name, description, parent_id) VALUES ($1, $2, $3, $4) ON CONFLICT (user_id, name) DO NOTHING"
    )
    .bind(user.id)
    .bind(params.name.trim()) // Trim whitespace
    .bind(params.description.as_deref().filter(|s| !s.trim().is_empty())) // Store None if description is empty/whitespace
    .bind(params.parent_id) // Bind the optional parent_id
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

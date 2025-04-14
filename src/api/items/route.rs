use crate::model::Item; // Import the Item struct
use crate::AppState; // Import AppState and Item from main.rs (or models module if refactored)
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response}, // Response is already imported
};
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "items.html")] // Points to the new template
struct ItemsTemplate {
    items: Vec<Item>, // Field to hold the list of items
}

// Handler function for GET /items
// Change return type from impl IntoResponse to Response
pub async fn get(State(pool): State<PgPool>) -> Response {
    let items_result = sqlx::query_as::<_, Item>("SELECT id, name FROM first_table ORDER BY id")
        .fetch_all(&pool)
        .await;

    match items_result {
        Ok(items) => {
            // Directly return the result of the inner match
            ItemsTemplate { items }.render().unwrap().into_response()
        }
        Err(e) => {
            eprintln!("Database error fetching items: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

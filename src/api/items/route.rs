use crate::model::Item; // Import the Item struct
use crate::AppState; // Import AppState and Item from main.rs (or models module if refactored)
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "items.html")] // Points to the new template
struct ItemsTemplate {
    items: Vec<Item>, // Field to hold the list of items
}

// Handler function for GET /items
pub async fn get(State(pool): State<PgPool>) -> impl IntoResponse {
    let items_result = sqlx::query_as::<_, Item>("SELECT id, name FROM first_table ORDER BY id")
        .fetch_all(&pool)
        .await;

    match items_result {
        Ok(items) => {
            // Assign the result of the inner match to a variable
            let response = match ItemsTemplate { items }.render() {
                Ok(html) => Html(html).into_response(),
                Err(e) => {
                    eprintln!("Template rendering error: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response()
                }
            };
            response // Return the response variable
        }
        Err(e) => {
            eprintln!("Database error fetching items: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

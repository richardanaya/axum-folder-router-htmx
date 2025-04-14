use crate::{AppState, Item}; // Import AppState and Item from main.rs (or models module if refactored)
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
pub async fn get(State(pool): State<PgPool>) -> Result<impl IntoResponse, Response> {
    // Fetch items from the database
    let items = sqlx::query_as::<_, Item>("SELECT id, name FROM first_table ORDER BY id")
        .fetch_all(&pool)
        .await;

    match items {
        Ok(items) => {
            // Render the template with the fetched items
            let template = ItemsTemplate { items };
            match template.render() {
                Ok(html) => Ok(Html(html)),
                Err(e) => {
                    // Handle template rendering errors
                    eprintln!("Template rendering error: {}", e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to render page",
                    )
                        .into_response())
                }
            }
        }
        Err(e) => {
            // Handle database query errors
            eprintln!("Database query error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch items from database",
            )
                .into_response())
        }
    }
}

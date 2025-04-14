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
    ItemsTemplate {
        items: sqlx::query_as::<_, Item>("SELECT id, name FROM first_table ORDER BY id")
            .fetch_all(&pool)
            .await
            .unwrap(),
    }
    .render()
    .unwrap()
    .into_response()
}

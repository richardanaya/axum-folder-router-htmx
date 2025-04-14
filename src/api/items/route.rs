use crate::model::Item; // Import the Item struct
use crate::AppState; // Import AppState and Item from main.rs (or models module if refactored)
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response}, // Response is already imported
};
use sqlx::PgPool;

use sqlx::FromRow;

// Define a struct matching the 'first_table' columns for fetching
#[derive(FromRow, Debug, Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
}

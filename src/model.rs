use sqlx::FromRow;

// Define a struct for personal values
#[derive(FromRow, Debug, Clone)]
pub struct PersonalValue {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>, // Description is optional
    pub parent_id: Option<i32>,      // Optional parent value ID
}

// Define a struct for user lookup
#[derive(FromRow, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
}

use chrono;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct ToDo {
    pub todo_id: i32,
    pub description: String,
    pub created_on: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct NewToDo {
    pub description: String,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct UpdateToDo {
    pub description: String,
}

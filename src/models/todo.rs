use chrono;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct ToDo {
    pub id: String,
    pub status: String,
    pub description: String,
    pub created_on: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct NewToDo {
    pub id: String,
    pub description: String,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct UpdateToDo {
    pub description: String,
}

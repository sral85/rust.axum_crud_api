use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::{json, Value};

use sqlx::PgPool;

use crate::errors::app_errors::AppError;
use crate::models::todo::{NewToDo, ToDo, UpdateToDo};

pub async fn get_todo(
    Path(id): Path<u32>,
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    let sql = "SELECT * FROM todo where todo_id = $1::int4".to_string();
    let todo = sqlx::query_as::<_, ToDo>(&sql)
        .bind(id.to_string())
        .fetch_one(&pool)
        .await
        .unwrap();
    (StatusCode::OK, Json(todo))
}

pub async fn get_todos(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM todo".to_string();

    let todos = sqlx::query_as::<_, ToDo>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(todos))
}

pub async fn create_todo(
    Extension(pool): Extension<PgPool>,
    Json(new_todo): Json<NewToDo>,
) -> Result<(StatusCode, Json<NewToDo>), AppError> {
    /*
    if task.task.is_empty() {
        return Err(CustomError::BadRequest)
    }
    */

    let sql = "INSERT INTO todo (description) values ($1)";

    let _ = sqlx::query(sql)
        .bind(&new_todo.description)
        .execute(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(new_todo)))
}

pub async fn delete_todo(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let _find = sqlx::query("SELECT * FROM todo where id=$1::int4")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| AppError::ObjectNotFoundError)?;

    sqlx::query("DELETE FROM todo WHERE todo_id=$1::int4")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::OK, Json(json!({"msg": "ToDo deleted"}))))
}

pub async fn update_todo(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
    Json(todo): Json<UpdateToDo>,
) -> Result<(StatusCode, Json<UpdateToDo>), AppError> {
    let sql = "SELECT * FROM todo where todo_id=$1::int4".to_string();

    let _find: ToDo = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();

    sqlx::query("UPDATE todo SET description=$1 WHERE todo_id=$2::int4")
        .bind(&todo.description)
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();

    Ok((StatusCode::OK, Json(todo)))
}

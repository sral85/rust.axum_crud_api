use axum::{
    extract::{Extension, Path},
    http::header::{HeaderMap, CONTENT_DISPOSITION, CONTENT_TYPE},
    http::{StatusCode, Uri},
    response::{IntoResponse, Json, Response},
};
use serde_json::{json, Value};

use sqlx::PgPool;

use crate::errors::app_errors::AppError;
use crate::models::todo::{NewToDo, ToDo, UpdateToDo};

pub async fn get_todo(
    Path(id): Path<u32>,
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<ToDo>), AppError> {
    let sql = "SELECT * FROM todo where todo_id = $1::int4".to_string();
    let todo = sqlx::query_as::<_, ToDo>(&sql)
        .bind(id.to_string())
        .fetch_one(&pool)
        .await
        .map_err(|_| AppError::ObjectNotFound)?;

    Ok((StatusCode::OK, Json(todo)))
}

pub async fn get_todos(
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<Vec<ToDo>>), AppError> {
    let sql = "SELECT * FROM todo".to_string();

    let todos = sqlx::query_as::<_, ToDo>(&sql)
        .fetch_all(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::OK, Json(todos)))
}

pub async fn create_todo(
    Extension(pool): Extension<PgPool>,
    Json(new_todo): Json<NewToDo>,
) -> Result<(StatusCode, Json<NewToDo>), AppError> {
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
    let _find = sqlx::query("SELECT * FROM todo where todo_id=$1::int4")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| AppError::ObjectNotFound)?;

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
        .map_err(|_| AppError::ObjectNotFound)?;

    sqlx::query("UPDATE todo SET description=$1 WHERE todo_id=$2::int4")
        .bind(&todo.description)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok((StatusCode::OK, Json(todo)))
}

pub async fn no_uri(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route for uri {}", uri))
}

pub async fn file_download() -> Response {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "text/csv; charset=utf-8".parse().unwrap());
    headers.insert(
        CONTENT_DISPOSITION,
        "attachment; filename=\"todo.csv\"".parse().unwrap(),
    );
    (StatusCode::ACCEPTED, headers, "id,value\n1,2").into_response()
}

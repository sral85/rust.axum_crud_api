use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    InternalServerError,
    ObjectNotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            AppError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::ObjectNotFound => (StatusCode::NOT_FOUND, "Object not found"),
        };
        (status, Json(json!(err_msg))).into_response()
    }
}

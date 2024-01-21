use reqwest;

extern crate axum_crud_api;
use axum_crud_api::models::todo;
use serde_json::json;

const BASE_URL: &str = "http://127.0.0.1:5000/";


#[tokio::test]
async fn test_get_todos() {
    let request_url = format!("{url}todo/", url = BASE_URL);
    println!("{}", &request_url);
    let response = reqwest::get(&request_url).await.unwrap();

    let todos: Vec<todo::ToDo> = response.json().await.unwrap();
    assert!(todos.len() > 0)
}

#[tokio::test]
async fn test_get_todo() {
    let request_url = format!("{url}todo/{id}/", url = BASE_URL, id = "homework");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await.unwrap();

    let todo: todo::ToDo = response.json().await.unwrap();
    assert!(todo.id == "homework")
}

#[tokio::test]
async fn test_create_and_delete_todo() {
    let request_url = format!("{url}todo/", url = BASE_URL);
    let client = reqwest::Client::new();

    // Create todo with id clean car
    let body = json!({"id": "clean car", "description": "clean car"});
    let response = client.post(&request_url).json(&body).send().await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    // Delete todo with id clean car
    let request_url = format!("{url}todo/{id}/", url = BASE_URL, id = "clean car");
    let response = client.delete(&request_url).send().await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_update_todo() {
    let request_url = format!("{url}todo/{id}/", url = BASE_URL, id = "homework");
    let client = reqwest::Client::new();

    // Update todo with id homework
    let body = json!({"description": "math homework"});
    let response = client.put(&request_url).json(&body).send().await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_fallback_url() {
    let request_url = format!("{url}unknown/", url = BASE_URL);
    let client = reqwest::Client::new();
    let response = client.get(&request_url).send().await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
}

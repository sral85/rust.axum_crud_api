use reqwest;

extern crate axum_crud_api;
use axum_crud_api::models::todo;
use serde_json::json;

fn get_base_url() -> &'static str {
    "http://127.0.0.1:5000/"
}

#[tokio::test]
async fn test_get_todos() {
    let request_url = format!("{url}todo/", url = get_base_url());
    println!("{}", &request_url);
    let response = reqwest::get(&request_url).await.unwrap();

    let todos: Vec<todo::ToDo> = response.json().await.unwrap();
    assert!(todos.len() > 0)
}

#[tokio::test]
async fn test_get_todo() {
    let request_url = format!("{url}todo/{id}/", url = get_base_url(), id = "homework");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await.unwrap();

    let todo: todo::ToDo = response.json().await.unwrap();
    assert!(todo.id == "homework")
}

#[tokio::test]
async fn test_create_and_delete_todo() {
    let request_url = format!("{url}todo/", url = get_base_url());
    let client = reqwest::Client::new();

    // Create todo with id clean car
    let body = json!({"id": "clean car", "description": "clean car"});
    let response = client.post(&request_url).json(&body).send().await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    // Delete todo with id clean car
    let request_url = format!("{url}todo/{id}/", url = get_base_url(), id = "clean car");
    let response = client.delete(&request_url).send().await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn test_fallback_url() {
    let request_url = format!("{url}unknown/", url = get_base_url());
    let client = reqwest::Client::new();
    let response = client.get(&request_url).send().await.unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
}
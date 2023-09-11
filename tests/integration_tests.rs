use reqwest;

extern crate axum_crud_api;
use axum_crud_api::models::todo;

#[tokio::test]
async fn test_get_todo() {
    let request_url = format!("http://127.0.0.1:5000/todo/{id}/", id = "1");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await.unwrap();

    let todo: todo::ToDo = response.json().await.unwrap();
    assert!(todo.todo_id == 1)
}

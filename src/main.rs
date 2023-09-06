use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tokio;

use std::{env, net::SocketAddr};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod errors;
mod models;
use controllers::handlers::{create_todo, delete_todo, get_todo, get_todos, update_todo};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "axum_crud_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    //let database_url = "postgres://admin:Passw0rd@localhost:5555/trial";
    let database_url =
        std::env::var("DATABASE_URL").expect("Environmental variable for database not present.");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/todo/:id", get(get_todo))
        .route("/todo/", get(get_todos))
        .route("/todo/", post(create_todo))
        .route("/todo/:id", put(update_todo))
        .route("/todo/:id", delete(delete_todo))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start axum server.");
}

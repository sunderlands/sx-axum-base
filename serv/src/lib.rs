use axum::{Router, serve};
use tokio::net::TcpListener;

#[tokio::main]
pub async fn run() {
    let listener = TcpListener::bind("localhost:3000").await.unwrap();
    serve(listener, Router::new()).await.unwrap();
}

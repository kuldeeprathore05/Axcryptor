mod handlers;
mod encryption;
mod models;
mod streaming;
use handlers::*;
use axum::{
    routing::post,
    Router,
};
use tower_http::services::ServeDir;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/encrypt", post(encrypt_file))
        .nest_service("/static", ServeDir::new("static"));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();  
}

mod handlers;
mod encryption;
mod models;
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
        .route("/api/decrypt", post(decrypt_file))
        .route("/api/batch_encrypt", post(encrypt_batch))
        .route("/api/batch_decrypt", post(decrypt_batch))
        .nest_service("/", ServeDir::new("static"));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();  
}
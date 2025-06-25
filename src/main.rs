mod handlers;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Set up app
    let app = Router::new()
        .route("/", get(handlers::serve_index))
        .route("/encrypt", post(handlers::handle_encrypt))
        .nest_service("/static", ServeDir::new("static"));

    // Create a listener on localhost:3000
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running at http://127.0.0.1:3000");

    // Serve the app
    axum::serve(listener, app).await.unwrap();  // ✅ Axum's serve, no hyper::Server needed
}

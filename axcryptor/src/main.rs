mod handlers;
mod encryption;
mod models;
use handlers::*;

use tower_http::services::{ServeDir, ServeFile};
use shuttle_axum::ShuttleAxum;
use shuttle_axum::axum::{routing::post, Router};
use tower_http::trace::TraceLayer;

#[shuttle_runtime::main]
async fn axum() -> ShuttleAxum {


    let static_dir = ServeDir::new("static")
        .not_found_service(ServeFile::new("static/index.html"));

    let app = Router::new()
        .route("/api/encrypt", post(encrypt_file))
        .route("/api/decrypt", post(decrypt_file))
        .route("/api/batch_encrypt", post(encrypt_batch))
        .route("/api/batch_decrypt", post(decrypt_batch))
        .fallback_service(static_dir)
        .layer(TraceLayer::new_for_http()); // Add this for request logging

    Ok(app.into())
}
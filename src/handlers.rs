use axum::{
    response::{Html, IntoResponse},
};
use axum_extra::extract::multipart::Multipart;
use tokio::fs;
use uuid::Uuid;
use crate::utils;

pub async fn serve_index() -> impl IntoResponse {
    Html(fs::read_to_string("static/index.html").await.unwrap())
}

pub async fn handle_encrypt(mut multipart: Multipart) -> impl IntoResponse {
    let mut file_bytes = Vec::new();
    let mut password = String::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        match name.as_str() {
            "file" => file_bytes = data.to_vec(),
            "password" => password = String::from_utf8(data.to_vec()).unwrap(),
            _ => {}
        }
    }

    let encrypted = utils::xor_encrypt(&file_bytes, &password);
    let filename = format!("{}.enc", Uuid::new_v4());
    fs::write(format!("static/{}", filename), encrypted).await.unwrap();

    format!("File encrypted successfully: /static/{}", filename)
}

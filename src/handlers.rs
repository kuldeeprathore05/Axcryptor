#![allow(unused)]

use axum::{
    extract::Multipart,
    http::StatusCode,
    response::Json,
};
use base64::{Engine as _, engine::general_purpose};
use std::sync::OnceLock;
use uuid::Uuid;
use crate::{
    encryption::{encrypt_data, decrypt_data, DecryptionInput},
    models::*,
    //streaming::{StreamProcessor, split_into_chunks},
};

// static STREAM_PROCESSOR: OnceLock<StreamProcessor> = OnceLock::new();

// fn get_stream_processor() -> &'static StreamProcessor {
//     STREAM_PROCESSOR.get_or_init(|| StreamProcessor::new())
// }

pub async fn encrypt_file(mut multipart: Multipart) -> Result<Json<EncryptResponse>, StatusCode> {
    println!("{}",123);
    let mut algorithm = None;
    let mut password = None;
    let mut file_data = None;
    let mut filename = None;
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let field_name = field.name().unwrap_or("").to_string();
        match field_name.as_str() {
            "algorithm" => {
                let value = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                algorithm = Some(match value.as_str() {
                    "AES256" => Algorithm::AES256,
                    "ChaCha20" => Algorithm::ChaCha20,
                    _ => return Err(StatusCode::BAD_REQUEST),
                });
            }
            "password" => {
                password = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            "file" => {
                filename = field.file_name().map(|s| s.to_string());
                file_data = Some(field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            _ => {}
        }
    }

    let algorithm = algorithm.ok_or(StatusCode::BAD_REQUEST)?;
    let password = password.ok_or(StatusCode::BAD_REQUEST)?;
    let file_data = file_data.ok_or(StatusCode::BAD_REQUEST)?;
    println!("{:?}",algorithm);
    println!("{:?}",password);
    //println!("{:?}",file_data);
    let aes = vec![0x01];
    let chacha :Vec<u8> = vec![0x02];
    match encrypt_data(&file_data, &password, &algorithm) {
        Ok(result) => {
            let file_id = Uuid::new_v4().to_string();
            
            // Combine salt + nonce + encrypted_data for storage
            let mut combined = Vec::new();
            match algorithm {
                Algorithm::AES256 => combined.extend_from_slice(&aes),
                Algorithm::ChaCha20 => combined.extend_from_slice(&chacha),
            }
            combined.extend_from_slice(&result.salt);
            combined.extend_from_slice(&result.nonce);
            combined.extend_from_slice(&result.encrypted_data);
            
            let encrypted_b64 = general_purpose::STANDARD.encode(&combined);
            
            Ok(Json(EncryptResponse {
                success: true,
                message: format!("File '{}' encrypted successfully", filename.unwrap_or("unknown".to_string())),
                file_id: Some(file_id),
                encrypted_data: Some(encrypted_b64),
            }))
        }
        Err(e) => Ok(Json(EncryptResponse {
            success: false,
            message: format!("Encryption failed: {}", e),
            file_id: None,
            encrypted_data: None,
        }))
    }
}


pub async fn decrypt_file(mut multipart: Multipart) -> Result<Json<DecryptResponse>, StatusCode> {
    let mut password = None;
    let mut encrypted_data = None;

    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let field_name = field.name().unwrap_or("").to_string();
        
        match field_name.as_str() {
            "password" => {
                password = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            "file" => {
                let b64_data = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                encrypted_data = Some(general_purpose::STANDARD.decode(b64_data).map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            _ => {}
        }
    }
    
    let password = password.ok_or(StatusCode::BAD_REQUEST)?;
    let encrypted_data = encrypted_data.ok_or(StatusCode::BAD_REQUEST)?;
    let algo = encrypted_data[0];
    let algorithm=match algo {
        0x01=>Algorithm::AES256,
        0x02=> Algorithm::ChaCha20,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    println!("{:?}",algorithm);
    println!("{:?}",password);
    println!("{:?}",encrypted_data);

    // Extract salt, nonce, and encrypted data
    if encrypted_data.len() < 45 { //1(alogo) + 32 (salt) + 12 (nonce) = 44 minimum
        return Err(StatusCode::BAD_REQUEST);
    }

    let salt = encrypted_data[1..33].to_vec();
    let nonce = encrypted_data[33..45].to_vec();
    let data = encrypted_data[45..].to_vec();

    let input = DecryptionInput {
        encrypted_data: data,
        nonce,
        salt,
    };
    println!("{:?}",1234);
    
    match decrypt_data(input, &password, &algorithm) {
        Ok(decrypted_data) => {
            let decrypted_b64 = general_purpose::STANDARD.encode(&decrypted_data);
            
            Ok(Json(DecryptResponse {
                success: true,
                message: "File decrypted successfully".to_string(),
                decrypted_data: Some(general_purpose::STANDARD.encode(&decrypted_data)),  
                filename: Some("decrypted_file".to_string()),
            }))
        }
        Err(e) => Ok(Json(DecryptResponse {
            success: false,
            message: format!("Decryption failed: {}", e),
            decrypted_data: None,
            filename: None,
        }))
    }
}

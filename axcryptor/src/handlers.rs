#![allow(unused)]
use axum::{
    extract::Multipart,
    response::{IntoResponse, Json},
    http::StatusCode,
};
use axum_macros::debug_handler;
use base64::{Engine as _, engine::general_purpose};
use std::{f32::consts::E, result, sync::OnceLock};
use uuid::Uuid;
use crate::{
    encryption::{encrypt_data, decrypt_data, DecryptionInput},
    models::*,
};


#[debug_handler]
pub async fn encrypt_file(mut multipart: Multipart) -> Result<impl IntoResponse, impl IntoResponse> {
    println!("{}",123);
    let mut algorithm = None;
    let mut password = None;
    let mut files = Vec::new();
    while let Some(field) = multipart.next_field().await.map_err(|_| (StatusCode::BAD_REQUEST, "Bad request".to_string()))? {
        let field_name = field.name().unwrap_or("").to_string();
        match field_name.as_str() {
            "algorithm" => {
                let value = field.text().await.map_err(|_| (StatusCode::BAD_REQUEST, "Bad request".to_string()))?;
                algorithm = Some(match value.as_str() {
                    "AES256" => Algorithm::AES256,
                    "ChaCha20" => Algorithm::ChaCha20,
                    _ => return Err((StatusCode::BAD_REQUEST, "Bad request".to_string())),
                });
            }
            "password" => {
                password = Some(field.text().await.map_err(|_| (StatusCode::BAD_REQUEST, "Bad request".to_string()))?);
            }
            "files" => {
                let filename = field.file_name().map(|s| s.to_string());
                let file_data = field.bytes().await.map_err(|_|(StatusCode::BAD_REQUEST, "Bad request".to_string()))?;
                files.push((filename,file_data));
            }
            _ => {}
        }
    }

    let algorithm = algorithm.ok_or((StatusCode::BAD_REQUEST, "Bad request".to_string()))?;
    let password = password.ok_or((StatusCode::BAD_REQUEST, "Bad request".to_string()))?;
    //let file_data = file_data.ok_or(StatusCode::BAD_REQUEST)?;
    println!("{:?}",algorithm);
    println!("{:?}",password);
    //println!("{:?}",file_data);
    let aes = vec![0x01];
    let chacha :Vec<u8> = vec![0x02];
     let mut results = Vec::new();
    for (filename, file_data) in files.iter() {
        match encrypt_data(file_data, &password, &algorithm) {
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
                //results.push((encrypted_b64,filename.clone().unwrap_or_else(|| "Encrypted".to_string())));
                results.push((encrypted_b64,file_id));
                
            }
            Err(e)=>{
                println!("eror")
            }
                
        }
    }
    Ok(Json(EncryptResponse {
            success: true,
            message: format!("Files encrypted successfully"),
            result : results,
        }))
}

#[debug_handler]
pub async fn decrypt_file(mut multipart: Multipart) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut password = None;
    //let mut encrypted_data = None;
    let mut files = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|_| (StatusCode::BAD_REQUEST, "Bad request".to_string()))? {
        let field_name = field.name().unwrap_or("").to_string();
        
        match field_name.as_str() {
            "password" => {
                password = Some(field.text().await.map_err(|_| (StatusCode::BAD_REQUEST, "Bad request".to_string()))?);
            }
            "file" => {
                let b64_data = field.text().await.map_err(|_| (StatusCode::BAD_REQUEST, "Bad request".to_string()))?;
                let encrypted_data = (general_purpose::STANDARD.decode(b64_data).map_err(|_| (StatusCode::BAD_REQUEST, "Bad request".to_string()))?);
                files.push(encrypted_data);
            }
            _ => {}
        }
    }
    println!("Files ki size {:?}",files.len()) ;
    
    let password = password.ok_or((StatusCode::BAD_REQUEST, "Bad request".to_string()))?;
   // let encrypted_data = encrypted_data.ok_or(StatusCode::BAD_REQUEST)?;
   let mut results = Vec::new();

    for (encrypted_data) in files.iter() {
        let algo = encrypted_data[0];
        let algorithm=match algo {
            0x01=>Algorithm::AES256,
            0x02=> Algorithm::ChaCha20,
            _ => return Err((StatusCode::BAD_REQUEST, "Bad request".to_string())),
        };
        println!("{:?}",algorithm);
        println!("{:?}",password);
        println!("{:?}",encrypted_data);
        if encrypted_data.len() < 45 { //1(alogo) + 32 (salt) + 12 (nonce) = 44 minimum
            return Err((StatusCode::BAD_REQUEST, "Bad request".to_string()));
        }

        let salt = encrypted_data[1..33].to_vec();
        let nonce = encrypted_data[33..45].to_vec();
        let data = encrypted_data[45..].to_vec();

        let input = DecryptionInput {
            encrypted_data: data,
            nonce,
            salt,
        };
      //  println!("{:?}",1234);
        match decrypt_data(input, &password, &algorithm) {
            Ok(decrypted_data) => {
                let decrypted_b64 = general_purpose::STANDARD.encode(&decrypted_data);
                results.push(decrypted_b64);
            }
            Err(e) => {}
        }
    }
     Ok(Json(DecryptResponse {
        success: true,
        message: "File decrypted successfully".to_string(),
        decrypted_data: results,  
        filename: Some("decrypted_file".to_string()),
    }))
    
}

#[debug_handler]
pub async fn encrypt_batch(mut multipart: Multipart) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut algorithm = None;
    let mut password = None;
    let mut files = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|_| (StatusCode::BAD_REQUEST, "Bad request".to_string()))? {
        let field_name = field.name().unwrap_or("").to_string();
        
        match field_name.as_str() {
            "algorithm" => {
                let value = field.text().await.map_err(|_| (StatusCode::BAD_REQUEST, "Bad request".to_string()))?;
                algorithm = Some(match value.as_str() {
                    "AES256" => Algorithm::AES256,
                    "ChaCha20" => Algorithm::ChaCha20,
                    _ => return Err((StatusCode::BAD_REQUEST, "Bad request".to_string())),
                });
            }
            "password" => {
                password = Some(field.text().await.map_err(|_| (StatusCode::BAD_REQUEST, "Bad request".to_string()))?);
            }
            "files" => {
                let filename = field.file_name().unwrap_or("unknown").to_string();
                let file_data = field.bytes().await.map_err(|_| (StatusCode::BAD_REQUEST, "Bad request".to_string()))?;
                files.push((filename, file_data));
            }
            _ => {}
        }
    }
    

    let algorithm = algorithm.ok_or((StatusCode::BAD_REQUEST, "Bad request".to_string()))?;
    let password = password.ok_or((StatusCode::BAD_REQUEST, "Bad request".to_string()))?;
    println!("{:?}",123);
    let mut results = Vec::new();
    let mut successful = 0;

    let aes = vec![0x01];
    let chacha :Vec<u8> = vec![0x02];

    let mut combined = Vec::new();

    for (filename, file_data) in files.iter() {
        match encrypt_data(file_data, &password, &algorithm) {
            Ok(res) => {
               
                match algorithm {
                    Algorithm::AES256 => combined.extend_from_slice(&aes),
                    Algorithm::ChaCha20 => combined.extend_from_slice(&chacha),
                }
                combined.extend_from_slice(&res.salt);
                combined.extend_from_slice(&res.nonce);
                let length  = res.encrypted_data.len() as u64;
                combined.extend_from_slice(&length.to_le_bytes());
                combined.extend_from_slice(&res.encrypted_data);
                successful+=1;
               
            }
            Err(e) => {
                results.push(FileResult {
                    filename: filename.clone(),
                    success: false,
                    message: format!("Encryption failed: {}", e),
                    file_id: None,
                });
            }
        }
    }
    let encrypted_b64 = general_purpose::STANDARD.encode(&combined);

    let total = files.len();
    let failed = total - successful;
    let file_id = Uuid::new_v4().to_string();
    Ok(Json(BatchResponse {
        success: successful > 0,
        message: format!("Processed {} files: {} successful, {} failed", total, successful, failed),
        processed_files: results,
        total_files: total,
        successful_files: successful,
        failed_files: failed,
        file_id:Some(file_id),
        encrypted_data:Some(encrypted_b64),
    }))
}


#[debug_handler]
pub async fn decrypt_batch(mut multipart: Multipart) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut password = None;
    let mut encrypted_data = None;
    
    while let Some(field) = multipart.next_field().await.map_err(|_| (StatusCode::BAD_REQUEST, "Bad request".to_string()))? {
        match field.name().unwrap_or("") {
            "password" => {
                password = Some(field.text().await.map_err(|_| (StatusCode::BAD_REQUEST, "Bad request".to_string()))?);
            }
            "files" => {
                let b64_data = field.text().await.map_err(|_| (StatusCode::BAD_REQUEST, "Bad request".to_string()))?;
                encrypted_data = Some(general_purpose::STANDARD.decode(b64_data).map_err(|_|(StatusCode::BAD_REQUEST, "Bad request".to_string()))?);
            }
            "algorithm" => continue,
            _ => {}
        }
    }
    let password = password.ok_or((StatusCode::BAD_REQUEST, "Bad request".to_string()))?;
    let encrypted_data = encrypted_data.ok_or((StatusCode::BAD_REQUEST, "Bad request".to_string()))?;


    let mut index = 0;
    let mut files: Vec<_> = Vec::new();
    println!("{:?}",encrypted_data.len());
    while index<encrypted_data.len() {
        let algo = encrypted_data[0+index];
        let algorithm=match algo {
            0x01=>Algorithm::AES256,
            0x02=> Algorithm::ChaCha20,
            _ => return Err((StatusCode::BAD_REQUEST, "Bad request".to_string())),
        };


        let salt = encrypted_data[1+index..33+index].to_vec();
        let nonce = encrypted_data[33+index..45+index].to_vec();
          let file_len = usize::from_le_bytes(encrypted_data[index+45..index+53].try_into().unwrap());

        if index + 53 + file_len > encrypted_data.len() {
            break;
        }
        println!("{:?}",file_len);

        let data = encrypted_data[index+53..index+53+file_len].to_vec();
        index += 53 + file_len;

        let input = DecryptionInput {
            encrypted_data: data,
            nonce,
            salt,
        };
        
        match decrypt_data(input, &password, &algorithm) {
            Ok(decrypted_data) => {
                let decrypted_b64 = general_purpose::STANDARD.encode(&decrypted_data);
                files.push(general_purpose::STANDARD.encode(&decrypted_data));
            }
            Err(e) =>continue,
        }
    }
     Ok(Json(BatchDecryptResponse {
        success: !files.is_empty(),
        message: format!("Decrypted {} files successfully", files.len()),
        files,
    }))
}




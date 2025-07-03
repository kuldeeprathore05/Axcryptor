use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Algorithm {
    AES256,
    ChaCha20,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptRequest {
    pub algorithm: Algorithm,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DecryptRequest {
    pub algorithm: Algorithm,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct EncryptResponse {
    pub success: bool,
    pub message: String,
    pub result : Vec<(String,String)>
}

#[derive(Debug, Serialize)]
pub struct DecryptResponse {
    pub success: bool,
    pub message: String,
    pub decrypted_data: Vec<String>,
    pub filename: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BatchResponse {
    pub success: bool,
    pub message: String,
    pub processed_files: Vec<FileResult>,
    pub total_files: usize,
    pub successful_files: usize,
    pub failed_files: usize,
    pub file_id: Option<String>,
    pub encrypted_data: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FileResult {
    pub filename: String,
    pub success: bool,
    pub message: String,
    pub file_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BatchDecryptResponse {
    pub success: bool,
    pub message: String,
    pub files: Vec<String>,
}

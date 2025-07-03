#![allow(unused)]
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use chacha20poly1305::{ChaCha20Poly1305, Key};
use rand::RngCore;
use anyhow::{Result, anyhow};
use crate::models::Algorithm;

const NONCE_SIZE: usize = 12;
const SALT_SIZE: usize = 32;

pub struct EncryptionResult {
    pub encrypted_data: Vec<u8>,
    pub nonce: Vec<u8>,
    pub salt: Vec<u8>,
}

pub struct DecryptionInput {
    pub encrypted_data: Vec<u8>,
    pub nonce: Vec<u8>,
    pub salt: Vec<u8>,
}



pub fn encrypt_data(data: &[u8], password: &str, algorithm: &Algorithm) -> Result<EncryptionResult> {
    let mut salt = vec![0u8; SALT_SIZE];
    OsRng.fill_bytes(&mut salt);
    
    let mut nonce = vec![0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce);

    let key = derive_key(password, &salt).unwrap();
    
    let encrypted_data = match algorithm {
        Algorithm::AES256 => encrypt_aes(&data, &key, &nonce),
        Algorithm::ChaCha20 => encrypt_chacha20(&data, &key, &nonce),
    };

    Ok(EncryptionResult {
        encrypted_data,
        nonce,
        salt,
    })
}



//decrypr data 
use argon2::{Argon2, password_hash::{PasswordHasher, SaltString}};
fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    let salt_string = SaltString::encode_b64(salt)
        .map_err(|e| format!("Salt encoding error: {}", e))?;
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt_string)
        .map_err(|e| format!("Password hashing error: {}", e))?;
    let hash_bytes = password_hash.hash.ok_or("No hash generated")?;

    let mut key = [0u8; 32];
    let hash_slice = hash_bytes.as_bytes();
    
    
    let copy_len = std::cmp::min(32, hash_slice.len());
    key[..copy_len].copy_from_slice(&hash_slice[..copy_len]);
    
    Ok(key)
}

fn encrypt_aes(data: &[u8], key: &[u8; 32], nonce: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new_from_slice(key).unwrap();
    
    let nonce = Nonce::from_slice(&nonce[..12]);
    
    cipher.encrypt(nonce, data).unwrap()
        
}

fn not_encrypt_aes(data: &[u8], key: &[u8; 32], nonce: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| anyhow!("Failed to create AES cipher: {}", e))?;
    
    let nonce = Nonce::from_slice(&nonce[..12]);
    
    cipher.encrypt(nonce, data)
        .map_err(|e| anyhow!("AES encryption failed: {}", e))
}

fn encrypt_chacha20(data: &[u8], key: &[u8; 32], nonce: &[u8]) -> Vec<u8> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = chacha20poly1305::Nonce::from_slice(&nonce[..12]);
    
    cipher.encrypt(nonce, data).unwrap()
}


fn not_encrypt_chacha20(data: &[u8], key: &[u8; 32], nonce: &[u8]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = chacha20poly1305::Nonce::from_slice(&nonce[..12]);
    
    cipher.encrypt(nonce, data)
        .map_err(|e| anyhow!("ChaCha20 encryption failed: {}", e))
}


pub fn decrypt_data(input: DecryptionInput, password: &str, algorithm: &Algorithm) -> Result<Vec<u8>> {
    let key = derive_key(password, &input.salt).unwrap();
    
    let decrypted_data = match algorithm {
        Algorithm::AES256 => decrypt_aes(&input.encrypted_data, &key, &input.nonce)?,
        Algorithm::ChaCha20 => decrypt_chacha20(&input.encrypted_data, &key, &input.nonce)?,
    };

    Ok(decrypted_data)
}
fn decrypt_aes(encrypted_data: &[u8], key: &[u8; 32], nonce: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| anyhow!("Failed to create AES cipher: {}", e))?;
    
    let nonce = Nonce::from_slice(&nonce[..12]);
    
    cipher.decrypt(nonce, encrypted_data)
        .map_err(|e| anyhow!("AES decryption failed: {}", e))
}
fn decrypt_chacha20(encrypted_data: &[u8], key: &[u8; 32], nonce: &[u8]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = chacha20poly1305::Nonce::from_slice(&nonce[..12]);
    
    cipher.decrypt(nonce, encrypted_data)
        .map_err(|e| anyhow!("ChaCha20 decryption failed: {}", e))
}
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

    let key = derive_key(password, &salt)?;
    
    let encrypted_data = match algorithm {
        Algorithm::AES256 => encrypt_aes(&data, &key, &nonce)?,
        Algorithm::ChaCha20 => encrypt_chacha20(&data, &key, &nonce)?,
    };

    Ok(EncryptionResult {
        encrypted_data,
        nonce,
        salt,
    })
}

//decrypr data 

fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32]> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    // Simple key derivation - in production, use PBKDF2 or Argon2
    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    salt.hash(&mut hasher);
    
    let hash = hasher.finish();
    let mut key = [0u8; 32];
    
    for (i, chunk) in hash.to_be_bytes().iter().cycle().take(32).enumerate() {
        key[i] = *chunk;
    }
    
    Ok(key)
}


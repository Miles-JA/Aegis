use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Nonce};

use crate::models::encrypted::EncryptedData;

pub fn encrypt_secret(plaintext: &str, key: &[u8; 32]) -> Result<EncryptedData, aes_gcm::Error> {

    let cipher = Aes256Gcm::new_from_slice(key).expect("Failed to generate cipher");

    let mut nonce = [0u8; 12];
    getrandom::fill(&mut nonce).expect("failed to generate encryption nonce");

    let nonce_value = Nonce::try_from(nonce.as_slice()).expect("nonce has the required 12-byte length");
    let ciphertext = cipher.encrypt(&nonce_value, plaintext.as_bytes())?;

    Ok(EncryptedData {
        ciphertext,
        nonce,
    })
}

pub fn generate_key() -> [u8; 32] {

    let mut key = [0u8; 32];
    getrandom::fill(&mut key).expect("failed to generate encryption key");
    key
}

use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Nonce};

use crate::models::{encrypted::EncryptedData, secret::Secret};

pub fn encrypt_secret(plaintext: &str, key: &[u8; 32]) -> Result<EncryptedData, aes_gcm::Error> {

    let cipher = Aes256Gcm::new_from_slice(key).expect("Failed to generate cipher");

    let mut nonce = [0u8; 12];
    getrandom::fill(&mut nonce).expect("failed to generate encryption nonce");

    let nonce_value = Nonce::try_from(nonce.as_slice()).expect("nonce has the required 12-byte length");

    let ciphertext = cipher.encrypt(&nonce_value, plaintext.as_bytes())?;


    let kek_hex = std::env::var("KEK").expect("Kek env not found");

    let kek_bytes = hex::decode(kek_hex).expect("Failed to decode KEK");

    let kek_cipher = Aes256Gcm::new_from_slice(&kek_bytes).expect("Failed to generate kek cipher");

     let mut dek_nonce = [0u8; 12];
    getrandom::fill(&mut dek_nonce)
        .expect("Failed to generate DEK nonce");

    let dek_nonce_value = Nonce::from(dek_nonce);

    // Encrypt DEK using KEK
    let encrypted_dek = kek_cipher.encrypt(
        &dek_nonce_value,
        key.as_slice()
    )?;

    Ok(EncryptedData {
        ciphertext,
        nonce,
        encrypted_dek,
        dek_nonce,
    })
}

pub fn decrypt_secret(secret: &Secret) -> anyhow::Result<String> {

    // Load KEK
    let kek_hex = std::env::var("KEK")
        .expect("KEK env not found");

    let kek_bytes = hex::decode(kek_hex)
        .expect("Failed to decode KEK");

    // Create KEK cipher
    let kek_cipher = Aes256Gcm::new_from_slice(&kek_bytes)?;

    // Decrypt DEK
    let dek_nonce = Nonce::from(secret.dek_nonce);

    let dek = kek_cipher.decrypt(
        &dek_nonce,
        secret.encrypted_dek.as_slice()
    )?;

    // Create DEK cipher
    let dek_cipher = Aes256Gcm::new_from_slice(&dek)?;

    // Decrypt actual secret
    let nonce = Nonce::from(secret.nonce);

    let plaintext = dek_cipher.decrypt(
        &nonce,
        secret.ciphertext.as_slice()
    )?;

    let text = String::from_utf8(plaintext)?;
    
    Ok(text)
}

pub fn generate_key() -> [u8; 32] {

    let mut key = [0u8; 32];
    getrandom::fill(&mut key).expect("failed to generate encryption key");
    key
}

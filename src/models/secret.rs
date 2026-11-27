use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateSecretRequest {
    pub name: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct CreateSecretResponse {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize)]
pub struct GetSecretResponse {
    pub name: String,
    pub secret: String,
}

#[derive(FromRow)]
pub struct Secret {
    pub id: Uuid,
    pub name: String,
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
    pub encrypted_dek: Vec<u8>,
    pub dek_nonce: [u8; 12],
}
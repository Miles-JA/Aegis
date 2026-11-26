use serde::{Deserialize, Serialize};
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

pub struct Secret {
    pub id: Uuid,
    pub name: String,
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
}
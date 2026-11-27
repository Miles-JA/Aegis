use axum::Json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{crypto::encryption::{self, decrypt_secret}, database::secrets::{insert_secret, select_secret}, models::secret::{CreateSecretRequest, CreateSecretResponse, GetSecretResponse, Secret}};

pub async fn create_secret(pool: &PgPool, Json(payload): Json<CreateSecretRequest>) -> CreateSecretResponse{
    let secret_id = Uuid::new_v4();

    let key = encryption::generate_key();

    let encrypted_data = encryption::encrypt_secret(&payload.value, &key).expect("Failed to encrypt secret");

    let secret = Secret {
        id: secret_id,
        name: payload.name.clone(),
        ciphertext: encrypted_data.ciphertext,
        nonce: encrypted_data.nonce,
        encrypted_dek: encrypted_data.encrypted_dek,
        dek_nonce: encrypted_data.dek_nonce,
    };

    insert_secret(&pool, &secret).await.expect("Failed to insert secret into database");

    CreateSecretResponse {
        id: secret_id,
        name: payload.name,
    }
}

pub async fn retrieve_secret(pool: &PgPool, name: String,) -> Result<GetSecretResponse, anyhow::Error> {
    let secret_name = name;
    let secret = select_secret(pool, &secret_name).await?;

    let decrypted_secret = decrypt_secret(&secret)?;

    Ok(GetSecretResponse {
        name: secret_name,
        secret: decrypted_secret,
    })
}


use axum::Json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{crypto::encryption, database::secrets::insert_secret, models::secret::{CreateSecretRequest, CreateSecretResponse, Secret}};

pub async fn create_secret(pool: &PgPool,Json(payload): Json<CreateSecretRequest>) -> CreateSecretResponse{
    let secret_id = Uuid::new_v4();

    let key = encryption::generate_key();

    let encrypted_data = encryption::encrypt_secret(&payload.value, &key).expect("Failed to encrypt secret");

    let secret = Secret {
        id: secret_id,
        name: payload.name.clone(),
        ciphertext: encrypted_data.ciphertext,
        nonce: encrypted_data.nonce,
    };

    insert_secret(&pool, &secret).await.expect("Failed to insert secret into database");

    CreateSecretResponse {
        id: secret_id,
        name: payload.name,
    }
}
use axum::Json;
use axum::extract::State;
use sqlx::PgPool;

use crate::models::secret::{CreateSecretRequest, CreateSecretResponse};
use crate::services::secrets::create_secret;

pub async fn secret(State(pool): State<PgPool>, payload: Json<CreateSecretRequest>) -> Json<CreateSecretResponse> {
    let encrypted_data = create_secret(&pool, payload).await;

    Json(CreateSecretResponse {
       id: encrypted_data.id,
       name: encrypted_data.name,
    })
}
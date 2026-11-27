use axum::Json;
use axum::extract::{Path, State};
use sqlx::PgPool;

use crate::models::secret::{CreateSecretRequest, CreateSecretResponse, GetSecretResponse};
use crate::services::secrets::{create_secret, retrieve_secret};

pub async fn post_secret(State(pool): State<PgPool>, payload: Json<CreateSecretRequest>) -> Json<CreateSecretResponse> {
    let encrypted_data = create_secret(&pool, payload).await;

    Json(CreateSecretResponse {
       id: encrypted_data.id,
       name: encrypted_data.name,
    })
}

pub async fn get_secret(State(pool): State<PgPool>, Path(name): Path<String>,) -> Result<Json<GetSecretResponse>, (axum::http::StatusCode, String)> {
    let secret = retrieve_secret(&pool, name)
        .await
        .map_err(|e| {
            eprintln!("Failed to retrieve secret: {e}");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to retrieve secret".to_string(),
            )
        })?;

    Ok(Json(secret))
}
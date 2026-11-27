use axum::Json;
use sqlx::PgPool;

use crate::models::secret::Secret;

pub async fn insert_secret(pool: &PgPool, payload: &Secret) -> Result<(), sqlx::Error> {

    sqlx::query(
        r#"
        INSERT INTO secrets (id, name, ciphertext, nonce, encrypted_dek, dek_nonce)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(payload.id)
    .bind(&payload.name)
    .bind(&payload.ciphertext)
    .bind(&payload.nonce)
    .bind(&payload.encrypted_dek)
    .bind(&payload.dek_nonce)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn select_secret(pool: &PgPool, name: &String,) -> Result<Json<Secret>, sqlx::Error> {
    let secret = sqlx::query_as::<_, Secret>(
        "SELECT * FROM secrets WHERE name = $1"
    )
    .bind(name)
    .fetch_one(pool)
    .await?;

    Ok(Json(secret))
}
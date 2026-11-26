use crate::models::secret::Secret;

pub async fn insert_secret(pool: &sqlx::PgPool, payload: &Secret) -> Result<(), sqlx::Error> {

    sqlx::query(
        r#"
        INSERT INTO secrets (id, name, ciphertext, nonce)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(payload.id)
    .bind(&payload.name)
    .bind(&payload.ciphertext)
    .bind(&payload.nonce)
    .execute(pool)
    .await?;

    Ok(())
}
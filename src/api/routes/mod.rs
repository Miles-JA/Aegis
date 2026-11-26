use axum::{Router, routing::get, routing::post};
use sqlx::{PgPool};

pub mod status;
pub mod health;
pub mod secrets;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/status", get(status::status))
        .route("/secrets", post(secrets::secret))
        .with_state(pool)
}
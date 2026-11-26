use serde_json::{json, Value};
use axum::Json;

pub async fn health_check() -> Json<Value> {
    Json(json!({
        "Status": "ok",
        "Node": "primary",
    }))
}
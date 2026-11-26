use serde_json::{json, Value};
use axum::Json;

pub async fn status() -> Json<Value> {
    let node_id = std::env::var("NODE_ID").unwrap_or_else(|_| "unknown".to_string());

    let role = std::env::var("NODE_ROLE").unwrap_or_else(|_| "unknown".to_string());

    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());

    Json(json!({
        "Node": node_id,
        "Role": role,
        "Port": port,
        "Status": "running", 
    }))
}
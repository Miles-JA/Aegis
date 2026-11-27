
mod api;
mod models;
mod services;
mod crypto;
mod database;

#[tokio::main]
async fn main() {
    let result = dotenv::dotenv().ok();

    let node_id = std::env::var("NODE_ID")
        .unwrap_or_else(|_| "unknown".to_string());

    let node_role = std::env::var("NODE_ROLE")
        .unwrap_or_else(|_| "unknown".to_string());

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8001".to_string());

    let pool = database::create_pool()
        .await
        .expect("Failed to connect to PostgreSQL");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    println!("Connected to PostgreSQL!");

    let app = api::routes::create_router(pool);

    let address = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("Failed to bind TCP listener");

    println!(
        "Aegis {} running as {} on port {}",
        node_id, node_role, port
    );
    axum::serve(listener, app).await.unwrap();
}

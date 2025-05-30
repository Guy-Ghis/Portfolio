use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Build our application with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/health", get(health_check));

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    
    // Updated server binding syntax for Axum 0.7
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    
    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Server error: {}", e);
        std::process::exit(1);
    }
}

// Basic health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

// Basic root endpoint
async fn root() -> &'static str {
    "Hello, World!"
}

// User data structure
#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    email: String,
}

// Endpoint to create a user
async fn create_user(Json(user): Json<User>) -> Json<User> {
    tracing::debug!("Created user: {:?}", user);
    Json(user)
}
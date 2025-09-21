// src/main.rs
mod db;
mod models;
mod routes;

use axum::Router;
use db::init_db;
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Initialize tracing for logging and observability
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "advanced_payroll=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = init_db().await;

    // Define CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build the application with routes, CORS, and tracing
    let app = Router::new()
        .merge(routes::routes(pool))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::debug!("🚀 Server listening on {}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();
}
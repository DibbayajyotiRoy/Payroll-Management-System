mod db;
mod models;
mod routes;

use axum::Router;
use db::init_db;
use tower_http::cors::{CorsLayer, Any};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let pool = init_db().await;
    
    // define CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // build the application with routes and CORS
    let app = Router::new()
        .merge(routes::routes(pool))
        .layer(cors);
    
    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await.unwrap();
    
    println!("🚀 Server running from Rust on {}", addr);
    
    axum::serve(listener, app)
        .await
        .unwrap();
}
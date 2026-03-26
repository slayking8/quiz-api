mod db;
mod docs;
mod handlers;
mod models;
mod repositories;
mod routes;

use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::SqlitePool,
}

#[tokio::main]
async fn main() {
    println!("🚀 Starting the Offline Quiz Server...");

    let db_pool = db::setup_database().await;

    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("--seed")) {
        println!("⚠️ '--seed' flag detected. Starting database seeding process...");
        db::seed_database(&db_pool).await;
    }

    let app_state = AppState { db_pool };

    // ==========================================
    // CORS CONFIGURATION
    // ==========================================
    // This is crucial for PWA Offline Sync. If the host IP changes,
    // the PWA installed on the student's phone will have a different origin.
    // We must permit requests from ANY origin to allow sync to succeed.
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 4. Build the application router AND attach the CORS layer
    let app = routes::create_router(app_state).layer(cors); // <-- ADD .layer(cors) HERE

    let port = 3000;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("🌐 Server is running! Listening on: http://{}", addr);
    println!(
        "📄 Swagger Documentation available at: http://{}/swagger-ui",
        addr
    );

    axum::serve(listener, app).await.unwrap();
}

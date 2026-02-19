mod handlers;
mod models;
mod parser;
mod state;

use std::{path::PathBuf, sync::Arc};

use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use handlers::{attempts, commands, health, sessions, stats};
use state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let log_path = std::env::var("COWRIE_LOG_PATH")
        .unwrap_or_else(|_| "/cowrie/var/log/cowrie/cowrie.json".to_string());

    info!("Reading Cowrie logs from: {}", log_path);

    let state = Arc::new(AppState {
        log_path: PathBuf::from(&log_path),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health::health))
        .route("/api/stats", get(stats::stats))
        .route("/api/attempts", get(attempts::attempts))
        .route("/api/commands", get(commands::commands))
        .route("/api/sessions", get(sessions::sessions))
        .layer(cors)
        .with_state(state);

    let addr = "0.0.0.0:3000";
    info!("Cowrie API listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

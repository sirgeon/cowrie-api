use std::sync::Arc;

use axum::{extract::State, response::Json};

use crate::{models::response::HealthResponse, state::AppState};

pub async fn health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    let (log_file_exists, log_size_bytes) = match std::fs::metadata(&state.log_path) {
        Ok(m) => (true, m.len()),
        Err(_) => (false, 0),
    };

    Json(HealthResponse {
        status: "ok",
        log_file_exists,
        log_size_bytes,
    })
}

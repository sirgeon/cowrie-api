use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Json};

use crate::{
    models::response::{AttemptsResponse, LoginAttempt},
    parser::load_events,
    state::AppState,
};

pub async fn attempts(
    State(state): State<Arc<AppState>>,
) -> Result<Json<AttemptsResponse>, StatusCode> {
    let events = load_events(&state.log_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let attempts: Vec<LoginAttempt> = events
        .iter()
        .filter(|e| {
            matches!(
                e.eventid.as_deref(),
                Some("cowrie.login.failed") | Some("cowrie.login.success")
            )
        })
        .map(|e| LoginAttempt {
            timestamp: e.timestamp.clone().unwrap_or_default(),
            src_ip: e.src_ip.clone().unwrap_or_else(|| "unknown".into()),
            src_port: e.src_port,
            username: e.username.clone().unwrap_or_default(),
            password: e.password.clone().unwrap_or_default(),
            session: e.session.clone().unwrap_or_default(),
            success: e.eventid.as_deref() == Some("cowrie.login.success"),
        })
        .collect();

    let total = attempts.len();
    Ok(Json(AttemptsResponse { total, attempts }))
}

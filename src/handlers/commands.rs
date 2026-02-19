use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Json};

use crate::{
    models::response::{CommandEntry, CommandsResponse},
    parser::load_events,
    state::AppState,
};

pub async fn commands(
    State(state): State<Arc<AppState>>,
) -> Result<Json<CommandsResponse>, StatusCode> {
    let events = load_events(&state.log_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let commands: Vec<CommandEntry> = events
        .iter()
        .filter(|e| e.eventid.as_deref() == Some("cowrie.command.input"))
        .map(|e| CommandEntry {
            timestamp: e.timestamp.clone().unwrap_or_default(),
            src_ip: e.src_ip.clone().unwrap_or_else(|| "unknown".into()),
            session: e.session.clone().unwrap_or_default(),
            command: e.input.clone().unwrap_or_default(),
        })
        .collect();

    let total = commands.len();
    Ok(Json(CommandsResponse { total, commands }))
}

use std::{collections::HashMap, sync::Arc};

use axum::{extract::State, http::StatusCode, response::Json};

use crate::{
    models::response::{SessionInfo, SessionsResponse},
    parser::load_events,
    state::AppState,
};

pub async fn sessions(
    State(state): State<Arc<AppState>>,
) -> Result<Json<SessionsResponse>, StatusCode> {
    let events = load_events(&state.log_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut session_map: HashMap<String, SessionInfo> = HashMap::new();

    for e in &events {
        let session_id = match &e.session {
            Some(s) => s.clone(),
            None => continue,
        };

        let eid = e.eventid.as_deref().unwrap_or("");

        let entry = session_map
            .entry(session_id.clone())
            .or_insert_with(|| SessionInfo {
                session: session_id,
                src_ip: e.src_ip.clone().unwrap_or_else(|| "unknown".into()),
                connected_at: e.timestamp.clone().unwrap_or_default(),
                login_attempts: 0,
                commands: Vec::new(),
            });

        match eid {
            "cowrie.command.input" => {
                if let Some(cmd) = &e.input {
                    entry.commands.push(cmd.clone());
                }
            }
            "cowrie.login.failed" | "cowrie.login.success" => {
                entry.login_attempts += 1;
            }
            _ => {}
        }
    }

    let sessions: Vec<SessionInfo> = session_map.into_values().collect();
    let total = sessions.len();
    Ok(Json(SessionsResponse { total, sessions }))
}

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use axum::{extract::State, http::StatusCode, response::Json};

use crate::{
    models::response::{EventSummary, StatsResponse},
    parser::{load_events, tally, top_n},
    state::AppState,
};

pub async fn stats(State(state): State<Arc<AppState>>) -> Result<Json<StatsResponse>, StatusCode> {
    let events = load_events(&state.log_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut unique_ips: HashSet<String> = HashSet::new();
    let mut login_attempts: usize = 0;
    let mut successful_logins: usize = 0;
    let mut commands_run: usize = 0;
    let mut usernames: HashMap<String, usize> = HashMap::new();
    let mut passwords: HashMap<String, usize> = HashMap::new();
    let mut ips: HashMap<String, usize> = HashMap::new();
    let mut commands: HashMap<String, usize> = HashMap::new();
    let mut recent: Vec<EventSummary> = Vec::new();

    for e in &events {
        if let Some(ip) = &e.src_ip {
            unique_ips.insert(ip.clone());
            tally(&mut ips, ip);
        }

        let eid = e.eventid.as_deref().unwrap_or("");

        match eid {
            "cowrie.login.failed" | "cowrie.login.success" => {
                login_attempts += 1;
                if eid == "cowrie.login.success" {
                    successful_logins += 1;
                }
                if let Some(u) = &e.username {
                    tally(&mut usernames, u);
                }
                if let Some(p) = &e.password {
                    tally(&mut passwords, p);
                }
            }
            "cowrie.command.input" => {
                commands_run += 1;
                if let Some(cmd) = &e.input {
                    let root_cmd = cmd.split_whitespace().next().unwrap_or(cmd);
                    tally(&mut commands, root_cmd);
                }
            }
            _ => {}
        }

        if recent.len() < 20 {
            let detail = match eid {
                "cowrie.login.failed" => format!(
                    "{}:{}",
                    e.username.as_deref().unwrap_or("-"),
                    e.password.as_deref().unwrap_or("-"),
                ),
                "cowrie.login.success" => format!(
                    "SUCCESS {}:{}",
                    e.username.as_deref().unwrap_or("-"),
                    e.password.as_deref().unwrap_or("-"),
                ),
                "cowrie.command.input" => e.input.clone().unwrap_or_default(),
                _ => e.message.clone().unwrap_or_default(),
            };

            recent.push(EventSummary {
                timestamp: e.timestamp.clone().unwrap_or_default(),
                event_type: eid.to_owned(),
                src_ip: e.src_ip.clone().unwrap_or_else(|| "unknown".into()),
                detail,
                session: e.session.clone().unwrap_or_default(),
            });
        }
    }

    Ok(Json(StatsResponse {
        total_events: events.len(),
        unique_ips: unique_ips.len(),
        login_attempts,
        successful_logins,
        commands_run,
        top_usernames: top_n(&usernames, 10),
        top_passwords: top_n(&passwords, 10),
        top_ips: top_n(&ips, 10),
        top_commands: top_n(&commands, 10),
        recent_events: recent,
    }))
}

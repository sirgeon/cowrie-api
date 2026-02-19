use serde::Serialize;

#[derive(Serialize)]
pub struct StatsResponse {
    pub total_events: usize,
    pub unique_ips: usize,
    pub login_attempts: usize,
    pub successful_logins: usize,
    pub commands_run: usize,
    pub top_usernames: Vec<CountEntry>,
    pub top_passwords: Vec<CountEntry>,
    pub top_ips: Vec<CountEntry>,
    pub top_commands: Vec<CountEntry>,
    pub recent_events: Vec<EventSummary>,
}

#[derive(Serialize)]
pub struct CountEntry {
    pub value: String,
    pub count: usize,
}

#[derive(Serialize)]
pub struct EventSummary {
    pub timestamp: String,
    pub event_type: String,
    pub src_ip: String,
    pub detail: String,
    pub session: String,
}

#[derive(Serialize)]
pub struct AttemptsResponse {
    pub total: usize,
    pub attempts: Vec<LoginAttempt>,
}

#[derive(Serialize)]
pub struct LoginAttempt {
    pub timestamp: String,
    pub src_ip: String,
    pub src_port: Option<u16>,
    pub username: String,
    pub password: String,
    pub session: String,
    pub success: bool,
}

#[derive(Serialize)]
pub struct CommandsResponse {
    pub total: usize,
    pub commands: Vec<CommandEntry>,
}

#[derive(Serialize)]
pub struct CommandEntry {
    pub timestamp: String,
    pub src_ip: String,
    pub session: String,
    pub command: String,
}

#[derive(Serialize)]
pub struct SessionsResponse {
    pub total: usize,
    pub sessions: Vec<SessionInfo>,
}

#[derive(Serialize)]
pub struct SessionInfo {
    pub session: String,
    pub src_ip: String,
    pub connected_at: String,
    pub login_attempts: usize,
    pub commands: Vec<String>,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub log_file_exists: bool,
    pub log_size_bytes: u64,
}

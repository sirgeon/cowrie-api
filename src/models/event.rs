use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CowrieEvent {
    pub timestamp: Option<String>,
    pub eventid: Option<String>,
    pub src_ip: Option<String>,
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub input: Option<String>,
    pub message: Option<String>,
    pub sensor: Option<String>,
    pub session: Option<String>,
}

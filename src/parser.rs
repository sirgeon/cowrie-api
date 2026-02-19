use std::{collections::HashMap, fs, path::PathBuf};

use crate::models::{event::CowrieEvent, response::CountEntry};

pub fn load_events(log_path: &PathBuf) -> anyhow::Result<Vec<CowrieEvent>> {
    let content = fs::read_to_string(log_path)?;

    let events = content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|line| serde_json::from_str::<CowrieEvent>(line).ok())
        .collect();

    Ok(events)
}

pub fn top_n(map: &HashMap<String, usize>, n: usize) -> Vec<CountEntry> {
    let mut entries: Vec<CountEntry> = map
        .iter()
        .map(|(k, v)| CountEntry {
            value: k.clone(),
            count: *v,
        })
        .collect();

    entries.sort_by(|a, b| b.count.cmp(&a.count));
    entries.truncate(n);
    entries
}

#[inline]
pub fn tally(map: &mut HashMap<String, usize>, key: &str) {
    *map.entry(key.to_owned()).or_insert(0) += 1;
}

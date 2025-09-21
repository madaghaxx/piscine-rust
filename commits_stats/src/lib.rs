use std::collections::HashMap;
pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut week_counts = HashMap::new();
    if let json::JsonValue::Array(commits) = data {
        for commit in commits {
            if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
                if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(date_str) {
                    let week = datetime.format("%Y-W%V").to_string();
                    *week_counts.entry(week).or_insert(0) += 1;
                }
            }
        }
    }
    week_counts
}
pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut author_counts = HashMap::new();
    if let json::JsonValue::Array(commits) = data {
        for commit in commits {
            if let Some(author_name) = commit["author"]["login"].as_str() {
                *author_counts.entry(author_name.to_string()).or_insert(0) += 1;
            }
        }
    }
    author_counts
}

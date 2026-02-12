use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Flattened DTO for log entries sent to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntryDto {
    pub timestamp: String,
    pub event_type: String,
    pub session_id: String,
    pub tool_name: Option<String>,
    pub rules_matched: Vec<String>,
    pub outcome: String,
    pub processing_ms: u64,
    pub rules_evaluated: usize,
    pub decision: Option<String>,
    pub mode: Option<String>,
    pub priority: Option<i64>,
    pub response_continue: Option<bool>,
    pub response_reason: Option<String>,
    pub event_detail_command: Option<String>,
    pub event_detail_file_path: Option<String>,
}

/// Query parameters for filtering log entries.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogQueryParams {
    pub text_filter: Option<String>,
    pub outcome_filter: Option<String>,
    pub decision_filter: Option<String>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub limit: Option<usize>,
}

/// Statistics about the log file.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogStats {
    pub total_entries: usize,
    pub file_size_bytes: u64,
    pub oldest_entry: Option<String>,
    pub newest_entry: Option<String>,
}

fn get_log_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join(".claude")
        .join("logs")
        .join("rulez.log")
}

fn parse_entry(value: &serde_json::Value) -> LogEntryDto {
    let timing = value.get("timing").unwrap_or(&serde_json::Value::Null);
    let response = value.get("response").unwrap_or(&serde_json::Value::Null);
    let event_details = value
        .get("event_details")
        .unwrap_or(&serde_json::Value::Null);

    // Extract event detail fields based on tool type
    let event_detail_command = event_details
        .get("command")
        .and_then(|v| v.as_str())
        .map(String::from);

    let event_detail_file_path = event_details
        .get("file_path")
        .and_then(|v| v.as_str())
        .map(String::from);

    let rules_matched = value
        .get("rules_matched")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    LogEntryDto {
        timestamp: value
            .get("timestamp")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        event_type: value
            .get("event_type")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string(),
        session_id: value
            .get("session_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        tool_name: value
            .get("tool_name")
            .and_then(|v| v.as_str())
            .map(String::from),
        rules_matched,
        outcome: value
            .get("outcome")
            .and_then(|v| v.as_str())
            .unwrap_or("allow")
            .to_string(),
        processing_ms: timing
            .get("processing_ms")
            .and_then(|v| v.as_u64())
            .unwrap_or(0),
        rules_evaluated: timing
            .get("rules_evaluated")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as usize,
        decision: value
            .get("decision")
            .and_then(|v| v.as_str())
            .map(String::from),
        mode: value.get("mode").and_then(|v| v.as_str()).map(String::from),
        priority: value.get("priority").and_then(|v| v.as_i64()),
        response_continue: response.get("continue").and_then(|v| v.as_bool()),
        response_reason: response
            .get("reason")
            .and_then(|v| v.as_str())
            .map(String::from),
        event_detail_command,
        event_detail_file_path,
    }
}

fn matches_text_filter(entry: &LogEntryDto, text: &str) -> bool {
    let lower = text.to_lowercase();
    entry.event_type.to_lowercase().contains(&lower)
        || entry.session_id.to_lowercase().contains(&lower)
        || entry.outcome.to_lowercase().contains(&lower)
        || entry
            .tool_name
            .as_deref()
            .is_some_and(|t| t.to_lowercase().contains(&lower))
        || entry
            .rules_matched
            .iter()
            .any(|r| r.to_lowercase().contains(&lower))
        || entry
            .response_reason
            .as_deref()
            .is_some_and(|r| r.to_lowercase().contains(&lower))
        || entry
            .event_detail_command
            .as_deref()
            .is_some_and(|c| c.to_lowercase().contains(&lower))
        || entry
            .event_detail_file_path
            .as_deref()
            .is_some_and(|p| p.to_lowercase().contains(&lower))
        || entry
            .decision
            .as_deref()
            .is_some_and(|d| d.to_lowercase().contains(&lower))
}

#[tauri::command]
pub async fn read_logs(params: LogQueryParams) -> Result<Vec<LogEntryDto>, String> {
    let log_path = get_log_path();

    if !log_path.exists() {
        return Ok(Vec::new());
    }

    let content = tokio::fs::read_to_string(&log_path)
        .await
        .map_err(|e| format!("Failed to read log file: {e}"))?;

    let since_dt: Option<DateTime<Utc>> = params
        .since
        .as_deref()
        .and_then(|s| s.parse::<DateTime<Utc>>().ok());

    let until_dt: Option<DateTime<Utc>> = params
        .until
        .as_deref()
        .and_then(|s| s.parse::<DateTime<Utc>>().ok());

    let limit = params.limit.unwrap_or(10_000);

    let mut entries: Vec<LogEntryDto> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let value: serde_json::Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(_) => continue, // Skip malformed lines
        };

        let entry = parse_entry(&value);

        // Apply outcome filter
        if let Some(ref outcome_filter) = params.outcome_filter {
            if entry.outcome != *outcome_filter {
                continue;
            }
        }

        // Apply decision filter
        if let Some(ref decision_filter) = params.decision_filter {
            match &entry.decision {
                Some(d) if d == decision_filter => {}
                _ => continue,
            }
        }

        // Apply time range filters
        if let Some(ref since) = since_dt {
            if let Ok(ts) = entry.timestamp.parse::<DateTime<Utc>>() {
                if ts < *since {
                    continue;
                }
            }
        }
        if let Some(ref until) = until_dt {
            if let Ok(ts) = entry.timestamp.parse::<DateTime<Utc>>() {
                if ts > *until {
                    continue;
                }
            }
        }

        // Apply text filter
        if let Some(ref text) = params.text_filter {
            if !text.is_empty() && !matches_text_filter(&entry, text) {
                continue;
            }
        }

        entries.push(entry);
    }

    // Sort newest first
    entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    // Apply limit
    entries.truncate(limit);

    Ok(entries)
}

#[tauri::command]
pub async fn get_log_stats() -> Result<LogStats, String> {
    let log_path = get_log_path();

    if !log_path.exists() {
        return Ok(LogStats {
            total_entries: 0,
            file_size_bytes: 0,
            oldest_entry: None,
            newest_entry: None,
        });
    }

    let metadata = tokio::fs::metadata(&log_path)
        .await
        .map_err(|e| format!("Failed to read log metadata: {e}"))?;

    let content = tokio::fs::read_to_string(&log_path)
        .await
        .map_err(|e| format!("Failed to read log file: {e}"))?;

    let mut total_entries = 0usize;
    let mut oldest: Option<String> = None;
    let mut newest: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(trimmed) {
            total_entries += 1;
            if let Some(ts) = value.get("timestamp").and_then(|v| v.as_str()) {
                let ts_str = ts.to_string();
                if oldest.is_none() {
                    oldest = Some(ts_str.clone());
                }
                newest = Some(ts_str);
            }
        }
    }

    Ok(LogStats {
        total_entries,
        file_size_bytes: metadata.len(),
        oldest_entry: oldest,
        newest_entry: newest,
    })
}

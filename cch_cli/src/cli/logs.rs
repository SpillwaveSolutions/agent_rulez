use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::logging::{LogQuery, QueryFilters};
use crate::models::{Decision, Outcome, PolicyMode};

/// Query and display logs with optional filtering
///
/// # Arguments
/// * `limit` - Maximum number of entries to return
/// * `since` - Filter entries since this RFC3339 timestamp
/// * `mode` - Filter by policy mode (enforce, warn, audit)
/// * `decision` - Filter by decision (allowed, blocked, warned, audited)
pub async fn run(
    limit: usize,
    since: Option<String>,
    mode: Option<String>,
    decision: Option<String>,
) -> Result<()> {
    let query = LogQuery::new();

    let mut filters = QueryFilters {
        limit: Some(limit),
        ..Default::default()
    };

    // Parse since timestamp
    if let Some(since_str) = since {
        if let Ok(since_time) = DateTime::parse_from_rfc3339(&since_str) {
            filters.since = Some(since_time.with_timezone(&Utc));
        } else {
            println!(
                "Warning: Invalid since timestamp format. Use RFC3339 format (e.g., 2024-01-01T00:00:00Z)"
            );
        }
    }

    // Parse mode filter
    if let Some(mode_str) = mode {
        match mode_str.to_lowercase().as_str() {
            "enforce" => filters.mode = Some(PolicyMode::Enforce),
            "warn" => filters.mode = Some(PolicyMode::Warn),
            "audit" => filters.mode = Some(PolicyMode::Audit),
            _ => {
                println!(
                    "Warning: Invalid mode '{}'. Valid values: enforce, warn, audit",
                    mode_str
                );
            }
        }
    }

    // Parse decision filter
    if let Some(decision_str) = decision {
        match decision_str.parse::<Decision>() {
            Ok(d) => filters.decision = Some(d),
            Err(_) => {
                println!(
                    "Warning: Invalid decision '{}'. Valid values: allowed, blocked, warned, audited",
                    decision_str
                );
            }
        }
    }

    let entries = query.query(filters)?;

    if entries.is_empty() {
        println!("No log entries found.");
        return Ok(());
    }

    println!("Found {} log entries:", entries.len());
    println!(
        "{:<25} {:<15} {:<12} {:<8} {:<8} {:<10} {:>6}",
        "Timestamp", "Event", "Tool", "Mode", "Decision", "Outcome", "Time"
    );

    for entry in entries {
        let tool = entry.tool_name.as_deref().unwrap_or("-");
        let mode_str = entry
            .mode
            .map(|m| format!("{}", m))
            .unwrap_or_else(|| "-".to_string());
        let decision_str = entry
            .decision
            .map(|d| format!("{}", d))
            .unwrap_or_else(|| "-".to_string());
        let outcome = match entry.outcome {
            Outcome::Allow => "ALLOW",
            Outcome::Block => "BLOCK",
            Outcome::Inject => "INJECT",
        };

        println!(
            "{:<25} {:<15} {:<12} {:<8} {:<8} {:<10} {:>6}ms",
            entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
            entry.event_type,
            tool,
            mode_str,
            decision_str,
            outcome,
            entry.timing.processing_ms
        );
    }

    Ok(())
}

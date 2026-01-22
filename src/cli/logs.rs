use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::logging::{LogQuery, QueryFilters};
use crate::models::Outcome;

/// Query and display logs
pub async fn run(limit: usize, since: Option<String>) -> Result<()> {
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
            println!("Warning: Invalid since timestamp format. Use RFC3339 format (e.g., 2024-01-01T00:00:00Z)");
        }
    }

    let entries = query.query(filters)?;

    if entries.is_empty() {
        println!("No log entries found.");
        return Ok(());
    }

    println!("Found {} log entries:", entries.len());
    println!("{:<25} {:<15} {:<12} {:<10} {:<8} {:<6}",
             "Timestamp", "Event", "Tool", "Rules", "Outcome", "Time");

    for entry in entries {
        let tool = entry.tool_name.as_deref().unwrap_or("-");
        let rules_count = entry.rules_matched.len();
        let outcome = match entry.outcome {
            Outcome::Allow => "ALLOW",
            Outcome::Block => "BLOCK",
            Outcome::Inject => "INJECT",
        };

        println!("{:<25} {:<15} {:<12} {:<10} {:<8} {:>6}ms",
                 entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
                 entry.event_type,
                 tool,
                 rules_count,
                 outcome,
                 entry.timing.processing_ms);
    }

    Ok(())
}
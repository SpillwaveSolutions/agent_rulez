use anyhow::Result;

use crate::logging::{LogQuery, QueryFilters};
use crate::models::Outcome;

/// Explain why rules fired for a given event
pub async fn run(event_id: String) -> Result<()> {
    let query = LogQuery::new();

    // For now, we'll search by session ID as a proxy for event ID
    let filters = QueryFilters {
        session_id: Some(event_id.clone()),
        limit: Some(50), // Get recent entries for this session
        ..Default::default()
    };

    let entries = query.query(filters)?;

    if entries.is_empty() {
        println!("No log entries found for event/session: {}", event_id);
        println!("Make sure the event has been processed and logged.");
        return Ok(());
    }

    println!("Explanation for event/session: {}", event_id);
    println!("Found {} related log entries", entries.len());
    println!();

    for (i, entry) in entries.iter().enumerate() {
        println!("Entry {}: {}", i + 1, entry.timestamp.format("%Y-%m-%d %H:%M:%S"));
        println!("  Event Type: {}", entry.event_type);
        println!("  Tool: {}", entry.tool_name.as_deref().unwrap_or("N/A"));
        println!("  Outcome: {:?}", entry.outcome);
        println!("  Processing Time: {}ms", entry.timing.processing_ms);
        println!("  Rules Evaluated: {}", entry.timing.rules_evaluated);

        if !entry.rules_matched.is_empty() {
            println!("  Rules That Matched:");
            for rule in &entry.rules_matched {
                println!("    - {}", rule);
            }
        } else {
            println!("  Rules That Matched: None");
        }

        if let Some(metadata) = &entry.metadata {
            if let Some(ref injected) = metadata.injected_files {
                println!("  Injected Files: {:?}", injected);
            }
            if let Some(ref output) = metadata.validator_output {
                println!("  Validator Output: {}", output);
            }
        }

        println!();
    }

    // Summary
    let blocked_count = entries.iter().filter(|e| matches!(e.outcome, Outcome::Block)).count();
    let injected_count = entries.iter().filter(|e| matches!(e.outcome, Outcome::Inject)).count();
    let allowed_count = entries.iter().filter(|e| matches!(e.outcome, Outcome::Allow)).count();

    println!("Summary:");
    println!("  Blocked: {}", blocked_count);
    println!("  Injected: {}", injected_count);
    println!("  Allowed: {}", allowed_count);

    Ok(())
}
use anyhow::Result;
use serde::Serialize;

use crate::config::Config;
use crate::logging::{LogQuery, QueryFilters};
use crate::models::{Decision, Outcome, PolicyMode, Rule};

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
        println!(
            "Entry {}: {}",
            i + 1,
            entry.timestamp.format("%Y-%m-%d %H:%M:%S")
        );
        println!("  Event Type: {}", entry.event_type);
        println!("  Tool: {}", entry.tool_name.as_deref().unwrap_or("N/A"));
        println!("  Outcome: {:?}", entry.outcome);
        println!("  Processing Time: {}ms", entry.timing.processing_ms);
        println!("  Rules Evaluated: {}", entry.timing.rules_evaluated);

        // Phase 2.2: Show governance fields
        if let Some(mode) = &entry.mode {
            println!("  Mode: {}", mode);
        }
        if let Some(decision) = &entry.decision {
            println!("  Decision: {}", decision);
        }
        if let Some(priority) = entry.priority {
            println!("  Priority: {}", priority);
        }

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
    let blocked_count = entries
        .iter()
        .filter(|e| matches!(e.outcome, Outcome::Block))
        .count();
    let injected_count = entries
        .iter()
        .filter(|e| matches!(e.outcome, Outcome::Inject))
        .count();
    let allowed_count = entries
        .iter()
        .filter(|e| matches!(e.outcome, Outcome::Allow))
        .count();

    println!("Summary:");
    println!("  Blocked: {}", blocked_count);
    println!("  Injected: {}", injected_count);
    println!("  Allowed: {}", allowed_count);

    Ok(())
}

/// Explain a specific rule (P2.3-T01 through P2.3-T03)
///
/// Displays mode, priority, metadata, and activity statistics for a rule.
pub async fn explain_rule(rule_name: String, json_output: bool, no_stats: bool) -> Result<()> {
    // Load configuration
    let config = Config::load(None)?;

    // Find the rule
    let rule = config
        .rules
        .iter()
        .find(|r| r.name == rule_name)
        .ok_or_else(|| anyhow::anyhow!("Rule '{}' not found in configuration", rule_name))?;

    if json_output {
        output_rule_json(rule, no_stats).await
    } else {
        output_rule_text(rule, no_stats).await
    }
}

/// Output rule details as formatted text
async fn output_rule_text(rule: &Rule, no_stats: bool) -> Result<()> {
    println!("Rule: {}", rule.name);
    if let Some(ref desc) = rule.description {
        println!("Description: {}", desc);
    }
    println!();

    // Governance fields (Phase 2.3)
    let mode = rule.effective_mode();
    let priority = rule.effective_priority();

    println!(
        "Mode: {}{}",
        mode,
        if rule.mode.is_none() {
            " (default)"
        } else {
            ""
        }
    );
    println!(
        "Priority: {}{}",
        priority,
        if rule.priority.is_none()
            && rule
                .metadata
                .as_ref()
                .map(|m| m.priority == 0)
                .unwrap_or(true)
        {
            " (default)"
        } else {
            ""
        }
    );
    println!();

    // Matchers
    println!("Matchers:");
    if let Some(ref tools) = rule.matchers.tools {
        println!("  tools: {:?}", tools);
    }
    if let Some(ref extensions) = rule.matchers.extensions {
        println!("  extensions: {:?}", extensions);
    }
    if let Some(ref directories) = rule.matchers.directories {
        println!("  directories: {:?}", directories);
    }
    if let Some(ref operations) = rule.matchers.operations {
        println!("  operations: {:?}", operations);
    }
    if let Some(ref cmd_match) = rule.matchers.command_match {
        println!("  command_match: \"{}\"", cmd_match);
    }
    println!();

    // Actions
    println!("Actions:");
    if let Some(ref inject) = rule.actions.inject {
        println!("  inject: {}", inject);
    }
    if let Some(script_path) = rule.actions.script_path() {
        println!("  run: {}", script_path);
        if let Some(trust) = rule.actions.trust_level() {
            println!("  trust: {}", trust);
        }
    }
    if let Some(block) = rule.actions.block {
        println!("  block: {}", block);
    }
    if let Some(ref block_if) = rule.actions.block_if_match {
        println!("  block_if_match: \"{}\"", block_if);
    }
    println!();

    // Governance metadata
    if let Some(ref gov) = rule.governance {
        println!("Governance:");
        if let Some(ref author) = gov.author {
            println!("  author: {}", author);
        }
        if let Some(ref created_by) = gov.created_by {
            println!("  created_by: {}", created_by);
        }
        if let Some(ref reason) = gov.reason {
            println!("  reason: {}", reason);
        }
        if let Some(ref confidence) = gov.confidence {
            println!("  confidence: {}", confidence);
        }
        if let Some(ref last_reviewed) = gov.last_reviewed {
            println!("  last_reviewed: {}", last_reviewed);
        }
        if let Some(ref ticket) = gov.ticket {
            println!("  ticket: {}", ticket);
        }
        if let Some(ref tags) = gov.tags {
            println!("  tags: {:?}", tags);
        }
        println!();
    }

    // Activity statistics (P2.3-T02)
    if !no_stats {
        print_activity_stats(&rule.name).await?;
    }

    Ok(())
}

/// Output rule details as JSON (P2.3-T03)
async fn output_rule_json(rule: &Rule, no_stats: bool) -> Result<()> {
    #[derive(Serialize)]
    struct RuleOutput<'a> {
        name: &'a str,
        description: Option<&'a str>,
        mode: PolicyMode,
        mode_is_default: bool,
        priority: i32,
        priority_is_default: bool,
        matchers: &'a crate::models::Matchers,
        actions: ActionsOutput<'a>,
        governance: Option<&'a crate::models::GovernanceMetadata>,
        #[serde(skip_serializing_if = "Option::is_none")]
        activity: Option<ActivityStats>,
    }

    #[derive(Serialize)]
    struct ActionsOutput<'a> {
        inject: Option<&'a str>,
        run: Option<&'a str>,
        trust: Option<crate::models::TrustLevel>,
        block: Option<bool>,
        block_if_match: Option<&'a str>,
    }

    #[derive(Serialize)]
    struct ActivityStats {
        total_triggers: usize,
        blocked: usize,
        warned: usize,
        audited: usize,
        allowed: usize,
        last_trigger: Option<String>,
    }

    let mode = rule.effective_mode();
    let priority = rule.effective_priority();
    let mode_is_default = rule.mode.is_none();
    let priority_is_default = rule.priority.is_none()
        && rule
            .metadata
            .as_ref()
            .map(|m| m.priority == 0)
            .unwrap_or(true);

    let actions = ActionsOutput {
        inject: rule.actions.inject.as_deref(),
        run: rule.actions.script_path(),
        trust: rule.actions.trust_level(),
        block: rule.actions.block,
        block_if_match: rule.actions.block_if_match.as_deref(),
    };

    let activity: Option<ActivityStats> = if !no_stats {
        get_activity_stats(&rule.name)
            .await
            .ok()
            .map(|s| ActivityStats {
                total_triggers: s.total_triggers,
                blocked: s.blocked,
                warned: s.warned,
                audited: s.audited,
                allowed: s.allowed,
                last_trigger: s
                    .last_trigger
                    .map(|t| t.format("%Y-%m-%d %H:%M").to_string()),
            })
    } else {
        None
    };

    let output = RuleOutput {
        name: &rule.name,
        description: rule.description.as_deref(),
        mode,
        mode_is_default,
        priority,
        priority_is_default,
        matchers: &rule.matchers,
        actions,
        governance: rule.governance.as_ref(),
        activity,
    };

    let json = serde_json::to_string_pretty(&output)?;
    println!("{}", json);

    Ok(())
}

/// Get activity statistics for a rule (P2.3-T02)
async fn get_activity_stats(rule_name: &str) -> Result<ActivityStatsInternal> {
    let query = LogQuery::new();
    let filters = QueryFilters {
        rule_name: Some(rule_name.to_string()),
        limit: Some(1000), // Look at recent entries
        ..Default::default()
    };

    let entries = query.query(filters)?;

    let total_triggers = entries.len();
    let blocked = entries
        .iter()
        .filter(|e| e.decision == Some(Decision::Blocked))
        .count();
    let warned = entries
        .iter()
        .filter(|e| e.decision == Some(Decision::Warned))
        .count();
    let audited = entries
        .iter()
        .filter(|e| e.decision == Some(Decision::Audited))
        .count();
    let allowed = entries
        .iter()
        .filter(|e| e.decision == Some(Decision::Allowed))
        .count();

    let last_trigger = entries.first().map(|e| e.timestamp);

    Ok(ActivityStatsInternal {
        total_triggers,
        blocked,
        warned,
        audited,
        allowed,
        last_trigger,
    })
}

struct ActivityStatsInternal {
    total_triggers: usize,
    blocked: usize,
    warned: usize,
    audited: usize,
    allowed: usize,
    last_trigger: Option<chrono::DateTime<chrono::Utc>>,
}

/// Print activity statistics (P2.3-T02)
async fn print_activity_stats(rule_name: &str) -> Result<()> {
    let stats = get_activity_stats(rule_name).await?;

    println!("Recent Activity:");
    println!("  Triggered: {} times", stats.total_triggers);
    println!("  Blocked: {} times", stats.blocked);
    println!("  Warned: {} times", stats.warned);
    println!("  Audited: {} times", stats.audited);
    println!("  Allowed: {} times", stats.allowed);
    if let Some(last) = stats.last_trigger {
        println!("  Last trigger: {}", last.format("%Y-%m-%d %H:%M"));
    } else {
        println!("  Last trigger: Never");
    }

    Ok(())
}

/// List all rules in the configuration (helper for CLI)
pub async fn list_rules() -> Result<()> {
    let config = Config::load(None)?;

    if config.rules.is_empty() {
        println!("No rules configured.");
        return Ok(());
    }

    println!("Configured rules ({} total):", config.rules.len());
    println!(
        "{:<25} {:<10} {:<8} {:<30}",
        "Name", "Mode", "Priority", "Description"
    );
    println!("{}", "-".repeat(75));

    for rule in config.enabled_rules() {
        let mode = rule.effective_mode();
        let priority = rule.effective_priority();
        let desc = rule
            .description
            .as_deref()
            .unwrap_or("-")
            .chars()
            .take(28)
            .collect::<String>();

        println!(
            "{:<25} {:<10} {:<8} {:<30}",
            rule.name, mode, priority, desc
        );
    }

    Ok(())
}

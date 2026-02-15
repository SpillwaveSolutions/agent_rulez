//! RuleZ Debug Command - Simulate and debug hook events
//!
//! Allows testing rules without invoking Claude Code.

use anyhow::{Context, Result};
use chrono::Utc;
use serde::Serialize;
use serde_json::json;
use std::io::Write;

use crate::config::Config;
use crate::hooks;
use crate::models::{DebugConfig, Event, EventType as ModelEventType};

/// Event type for simulation (CLI parsing)
#[derive(Debug, Clone, Copy)]
pub enum SimEventType {
    PreToolUse,
    PostToolUse,
    SessionStart,
    PermissionRequest,
    UserPromptSubmit,
    SessionEnd,
    PreCompact,
}

impl SimEventType {
    fn as_model_event_type(self) -> ModelEventType {
        match self {
            SimEventType::PreToolUse => ModelEventType::PreToolUse,
            SimEventType::PostToolUse => ModelEventType::PostToolUse,
            SimEventType::SessionStart => ModelEventType::SessionStart,
            SimEventType::PermissionRequest => ModelEventType::PermissionRequest,
            SimEventType::UserPromptSubmit => ModelEventType::UserPromptSubmit,
            SimEventType::SessionEnd => ModelEventType::SessionEnd,
            SimEventType::PreCompact => ModelEventType::PreCompact,
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pretooluse" | "pre" | "pre-tool-use" => Some(SimEventType::PreToolUse),
            "posttooluse" | "post" | "post-tool-use" => Some(SimEventType::PostToolUse),
            "sessionstart" | "session" | "start" => Some(SimEventType::SessionStart),
            "permissionrequest" | "permission" | "perm" => Some(SimEventType::PermissionRequest),
            "userpromptsubmit" | "prompt" | "user-prompt" | "user-prompt-submit" => {
                Some(SimEventType::UserPromptSubmit)
            }
            "sessionend" | "end" | "session-end" => Some(SimEventType::SessionEnd),
            "precompact" | "compact" | "pre-compact" => Some(SimEventType::PreCompact),
            _ => None,
        }
    }
}

/// JSON output structures matching the Tauri DebugResult type
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonDebugResult {
    outcome: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
    matched_rules: Vec<String>,
    evaluation_time_ms: f64,
    evaluations: Vec<JsonRuleEvaluation>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonRuleEvaluation {
    rule_name: String,
    matched: bool,
    time_ms: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<String>,
}

/// Run the debug command
pub async fn run(
    event_type: String,
    tool: Option<String>,
    command: Option<String>,
    path: Option<String>,
    prompt: Option<String>,
    verbose: bool,
    json_output: bool,
) -> Result<()> {
    // Clear regex cache for state isolation between debug invocations
    {
        use crate::hooks::REGEX_CACHE;
        REGEX_CACHE.lock().unwrap().clear();
    }

    let event_type = SimEventType::from_str(&event_type).context(format!(
        "Unknown event type: '{}'\nValid types: PreToolUse, PostToolUse, SessionStart, PermissionRequest, UserPromptSubmit, SessionEnd, PreCompact",
        event_type
    ))?;

    // Load configuration
    let config = Config::load(None)?;

    // Build simulated event
    let event = build_event(
        event_type,
        tool.clone(),
        command.clone(),
        path.clone(),
        prompt.clone(),
    );

    if json_output {
        return run_json_mode(event, &config).await;
    }

    // Human-readable mode
    println!("RuleZ Debug Mode");
    println!("{}", "=".repeat(60));
    println!();
    println!("Loaded {} rules from configuration", config.rules.len());
    println!();

    let event_json = serde_json::to_string_pretty(&event)?;
    println!("Simulated Event:");
    println!("{}", "-".repeat(40));
    println!("{}", event_json);
    println!();

    // Process the event with debug enabled
    let debug_config = DebugConfig::new(true, config.settings.debug_logs);
    let start = std::time::Instant::now();
    let response = hooks::process_event(event, &debug_config).await?;
    let elapsed = start.elapsed();
    let response_json = serde_json::to_string_pretty(&response)?;

    println!("Response:");
    println!("{}", "-".repeat(40));
    println!("{}", response_json);
    println!();

    // Show performance metrics
    println!("Performance:");
    println!("{}", "-".repeat(40));
    let rule_count = response
        .timing
        .as_ref()
        .map(|t| t.rules_evaluated)
        .unwrap_or(0);
    println!(
        "Processed in {}ms ({} rules evaluated)",
        elapsed.as_millis(),
        rule_count
    );
    println!();

    // Show rule evaluation summary
    if verbose {
        print_rule_summary(&config);
    }

    // Explain the outcome
    println!("Summary:");
    println!("{}", "-".repeat(40));
    if response.continue_ {
        if let Some(context) = &response.context {
            println!("✓ Allowed with injected context ({} chars)", context.len());
        } else {
            println!("✓ Allowed (no matching rules)");
        }
    } else {
        println!(
            "✗ Blocked: {}",
            response.reason.as_deref().unwrap_or("No reason provided")
        );
    }

    Ok(())
}

/// Run debug in JSON output mode — single structured JSON object to stdout
async fn run_json_mode(event: Event, config: &Config) -> Result<()> {
    let start = std::time::Instant::now();

    // Evaluate each rule individually to build per-rule traces
    let mut evaluations: Vec<JsonRuleEvaluation> = Vec::new();
    let mut matched_rules: Vec<String> = Vec::new();

    for rule in &config.rules {
        let rule_start = std::time::Instant::now();
        let matches = rule_matches_event(rule, &event);
        let rule_time = rule_start.elapsed().as_secs_f64() * 1000.0;

        let details = if matches {
            Some("Rule matched".to_string())
        } else {
            Some("No match".to_string())
        };

        // Extract pattern info from the rule for display
        let pattern = extract_rule_pattern(rule);
        let input = extract_event_input(&event);

        if matches {
            matched_rules.push(rule.name.clone());
        }

        evaluations.push(JsonRuleEvaluation {
            rule_name: rule.name.clone(),
            matched: matches,
            time_ms: rule_time,
            details,
            pattern,
            input: input.clone(),
        });
    }

    // Process the event to get the actual response
    let debug_config = DebugConfig::new(true, config.settings.debug_logs);
    let response = hooks::process_event(event, &debug_config).await?;
    let total_time = start.elapsed().as_secs_f64() * 1000.0;

    // Determine outcome
    let outcome = if !response.continue_ {
        "Block".to_string()
    } else if response.context.is_some() {
        "Inject".to_string()
    } else {
        "Allow".to_string()
    };

    let result = JsonDebugResult {
        outcome,
        reason: response.reason.clone(),
        matched_rules,
        evaluation_time_ms: total_time,
        evaluations,
    };

    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}

/// Check if a single rule matches the given event (simplified version for JSON trace)
fn rule_matches_event(rule: &crate::models::Rule, event: &Event) -> bool {
    let matchers = &rule.matchers;

    // Check operations (event types)
    if let Some(ref operations) = matchers.operations {
        let event_name = event.hook_event_name.to_string();
        if !operations.contains(&event_name) {
            return false;
        }
    }

    // Check tool filter
    if let Some(ref tools) = matchers.tools {
        if let Some(ref tool_name) = event.tool_name {
            if !tools.contains(tool_name) {
                return false;
            }
        } else {
            return false;
        }
    }

    // Check command_match pattern
    if let Some(ref cmd_pattern) = matchers.command_match {
        if let Some(ref tool_input) = event.tool_input {
            let cmd = tool_input
                .get("command")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if let Ok(re) = regex::Regex::new(cmd_pattern) {
                if !re.is_match(cmd) {
                    return false;
                }
            }
        } else {
            return false;
        }
    }

    // Check file extensions
    if let Some(ref extensions) = matchers.extensions {
        if let Some(ref tool_input) = event.tool_input {
            let file_path = tool_input
                .get("filePath")
                .or_else(|| tool_input.get("file_path"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let path_ext = std::path::Path::new(file_path)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("");
            if !extensions
                .iter()
                .any(|ext| ext == &format!(".{}", path_ext))
            {
                return false;
            }
        } else {
            return false;
        }
    }

    // Check directory patterns
    if let Some(ref directories) = matchers.directories {
        if let Some(ref tool_input) = event.tool_input {
            let file_path = tool_input
                .get("filePath")
                .or_else(|| tool_input.get("file_path"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if !directories.iter().any(|dir| {
                file_path.contains(dir.trim_end_matches("/**"))
                    || file_path.contains(dir.trim_end_matches("/*"))
            }) {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

/// Extract the primary matching pattern from a rule for display
fn extract_rule_pattern(rule: &crate::models::Rule) -> Option<String> {
    if let Some(ref pattern) = rule.matchers.command_match {
        return Some(pattern.clone());
    }
    if let Some(ref tools) = rule.matchers.tools {
        return Some(tools.join(", "));
    }
    if let Some(ref ops) = rule.matchers.operations {
        return Some(ops.join(", "));
    }
    None
}

/// Extract the relevant input from an event for display
fn extract_event_input(event: &Event) -> Option<String> {
    if let Some(ref tool_input) = event.tool_input {
        if let Some(cmd) = tool_input.get("command").and_then(|v| v.as_str()) {
            return Some(cmd.to_string());
        }
        if let Some(path) = tool_input.get("file_path").and_then(|v| v.as_str()) {
            return Some(path.to_string());
        }
    }
    None
}

/// Build a simulated event
fn build_event(
    event_type: SimEventType,
    tool: Option<String>,
    command: Option<String>,
    path: Option<String>,
    prompt: Option<String>,
) -> Event {
    let session_id = format!("debug-{}", uuid_simple());

    // For UserPromptSubmit events, tool/tool_input are optional (not defaulting to Bash)
    let (tool_name, tool_input) = if matches!(event_type, SimEventType::UserPromptSubmit) {
        (tool, None)
    } else {
        let tool_name = tool.unwrap_or_else(|| "Bash".to_string());
        let tool_input = match tool_name.as_str() {
            "Bash" => {
                let cmd = command.unwrap_or_else(|| "echo 'test'".to_string());
                json!({
                    "command": cmd,
                    "description": "Debug simulated command"
                })
            }
            "Write" | "Edit" | "Read" => {
                let file_path = path.unwrap_or_else(|| "src/main.rs".to_string());
                json!({
                    "file_path": file_path,
                    "content": "// Simulated content"
                })
            }
            "Glob" | "Grep" => {
                let pattern = command.unwrap_or_else(|| "*.rs".to_string());
                json!({
                    "pattern": pattern,
                    "path": path.unwrap_or_else(|| ".".to_string())
                })
            }
            _ => {
                json!({
                    "description": "Simulated tool input"
                })
            }
        };
        (Some(tool_name), Some(tool_input))
    };

    Event {
        hook_event_name: event_type.as_model_event_type(),
        session_id,
        tool_name,
        tool_input,
        timestamp: Utc::now(),
        user_id: None,
        transcript_path: None,
        cwd: None,
        permission_mode: None,
        tool_use_id: None,
        prompt,
    }
}

/// Print rule matching summary
fn print_rule_summary(config: &Config) {
    println!("Configured Rules:");
    println!("{}", "-".repeat(40));

    for rule in &config.rules {
        let metadata = rule.metadata.as_ref();
        let enabled = metadata.is_none_or(|m| m.enabled);
        let priority = metadata.map_or(50, |m| m.priority);
        let status = if enabled { "✓" } else { "○" };

        println!("  {} [P{}] {}", status, priority, rule.name,);
        if let Some(desc) = &rule.description {
            println!("      {}", desc);
        }
    }
    println!();
}

/// Generate a simple UUID-like string
fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{:x}", duration.as_nanos())
}

/// Interactive debug mode
pub async fn interactive() -> Result<()> {
    println!("RuleZ Interactive Debug Mode");
    println!("{}", "=".repeat(60));
    println!("Enter events as JSON or use shortcuts:");
    println!("  bash <command>    - Simulate Bash tool");
    println!("  write <path>      - Simulate Write tool");
    println!("  read <path>       - Simulate Read tool");
    println!("  quit              - Exit");
    println!();

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    loop {
        print!("rulez> ");
        stdout.flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "quit" || input == "exit" || input == "q" {
            println!("Goodbye!");
            break;
        }

        // Parse shortcuts
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        match parts.first().map(|s| s.to_lowercase()).as_deref() {
            Some("bash") => {
                let cmd = (*parts.get(1).unwrap_or(&"echo test")).to_string();
                run(
                    "PreToolUse".to_string(),
                    Some("Bash".to_string()),
                    Some(cmd),
                    None,
                    None,
                    false,
                    false,
                )
                .await?;
            }
            Some("write") => {
                let path = (*parts.get(1).unwrap_or(&"test.txt")).to_string();
                run(
                    "PreToolUse".to_string(),
                    Some("Write".to_string()),
                    None,
                    Some(path),
                    None,
                    false,
                    false,
                )
                .await?;
            }
            Some("read") => {
                let path = (*parts.get(1).unwrap_or(&"test.txt")).to_string();
                run(
                    "PreToolUse".to_string(),
                    Some("Read".to_string()),
                    None,
                    Some(path),
                    None,
                    false,
                    false,
                )
                .await?;
            }
            Some("help") => {
                println!("Commands:");
                println!("  bash <command>  - Test a bash command");
                println!("  write <path>    - Test writing to a file");
                println!("  read <path>     - Test reading a file");
                println!("  quit            - Exit");
            }
            _ => {
                // Try to parse as JSON
                match serde_json::from_str::<Event>(input) {
                    Ok(event) => {
                        let config = Config::load(None)?;
                        let debug_config = DebugConfig::new(true, config.settings.debug_logs);
                        let response = hooks::process_event(event, &debug_config).await?;
                        println!("{}", serde_json::to_string_pretty(&response)?);
                    }
                    Err(_) => {
                        println!("Unknown command or invalid JSON. Type 'help' for options.");
                    }
                }
            }
        }
        println!();
    }

    Ok(())
}

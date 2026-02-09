//! CCH Debug Command - Simulate and debug hook events
//!
//! Allows testing rules without invoking Claude Code.

use anyhow::{Context, Result};
use chrono::Utc;
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
}

impl SimEventType {
    fn as_model_event_type(self) -> ModelEventType {
        match self {
            SimEventType::PreToolUse => ModelEventType::PreToolUse,
            SimEventType::PostToolUse => ModelEventType::PostToolUse,
            SimEventType::SessionStart => ModelEventType::SessionStart,
            SimEventType::PermissionRequest => ModelEventType::PermissionRequest,
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pretooluse" | "pre" | "pre-tool-use" => Some(SimEventType::PreToolUse),
            "posttooluse" | "post" | "post-tool-use" => Some(SimEventType::PostToolUse),
            "sessionstart" | "session" | "start" => Some(SimEventType::SessionStart),
            "permissionrequest" | "permission" | "perm" => Some(SimEventType::PermissionRequest),
            _ => None,
        }
    }
}

/// Run the debug command
pub async fn run(
    event_type: String,
    tool: Option<String>,
    command: Option<String>,
    path: Option<String>,
    verbose: bool,
) -> Result<()> {
    let event_type = SimEventType::from_str(&event_type).context(format!(
        "Unknown event type: '{}'\nValid types: PreToolUse, PostToolUse, SessionStart, PermissionRequest",
        event_type
    ))?;

    println!("CCH Debug Mode");
    println!("{}", "=".repeat(60));
    println!();

    // Load configuration
    let config = Config::load(None)?;
    println!("Loaded {} rules from configuration", config.rules.len());
    println!();

    // Build simulated event
    let event = build_event(event_type, tool.clone(), command.clone(), path.clone());
    let event_json = serde_json::to_string_pretty(&event)?;

    println!("Simulated Event:");
    println!("{}", "-".repeat(40));
    println!("{}", event_json);
    println!();

    // Process the event with debug enabled
    let debug_config = DebugConfig::new(true, config.settings.debug_logs);
    let response = hooks::process_event(event, &debug_config).await?;
    let response_json = serde_json::to_string_pretty(&response)?;

    println!("Response:");
    println!("{}", "-".repeat(40));
    println!("{}", response_json);
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

/// Build a simulated event
fn build_event(
    event_type: SimEventType,
    tool: Option<String>,
    command: Option<String>,
    path: Option<String>,
) -> Event {
    let tool_name = tool.unwrap_or_else(|| "Bash".to_string());
    let session_id = format!("debug-{}", uuid_simple());

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

    Event {
        hook_event_name: event_type.as_model_event_type(),
        session_id,
        tool_name: Some(tool_name),
        tool_input: Some(tool_input),
        timestamp: Utc::now(),
        user_id: None,
        transcript_path: None,
        cwd: None,
        permission_mode: None,
        tool_use_id: None,
        prompt: None,
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
    println!("CCH Interactive Debug Mode");
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
        print!("cch> ");
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

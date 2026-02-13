use anyhow::Result;
use serde_json::Value;
use std::io::{self, Read};

use crate::adapters::copilot::{parse_event, translate_response};
use crate::config;
use crate::hooks;
use crate::models::{CopilotDecision, CopilotHookResponse, DebugConfig};

pub async fn run(debug_logs: bool) -> Result<()> {
    match run_inner(debug_logs).await {
        Ok(()) => Ok(()),
        Err(err) => {
            eprintln!("Copilot hook runner error: {}", err);
            emit_safe_response(&format!("Copilot hook runner error: {}", err))?;
            Ok(())
        }
    }
}

async fn run_inner(debug_logs: bool) -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    if buffer.trim().is_empty() {
        emit_safe_response("No input received on stdin")?;
        return Ok(());
    }

    let raw_value: Value = serde_json::from_str(&buffer)?;
    let copilot_event = parse_event(raw_value)?;

    let project_config = config::Config::load(
        copilot_event
            .event
            .cwd
            .as_ref()
            .map(|p| std::path::Path::new(p.as_str())),
    )?;
    let debug_config = DebugConfig::new(debug_logs, project_config.settings.debug_logs);

    let response = hooks::process_event(copilot_event.event.clone(), &debug_config).await?;
    let hook_response = translate_response(&response, &copilot_event);

    let json = serde_json::to_string(&hook_response)?;
    println!("{}", json);

    Ok(())
}

fn emit_safe_response(reason: &str) -> Result<()> {
    let response = CopilotHookResponse {
        permission_decision: CopilotDecision::Allow,
        permission_decision_reason: Some(reason.to_string()),
        tool_input: None,
    };
    let json = serde_json::to_string(&response)?;
    println!("{}", json);
    Ok(())
}

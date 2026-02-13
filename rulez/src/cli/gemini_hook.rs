use anyhow::Result;
use serde_json::Value;
use std::io::{self, Read};

use crate::adapters::gemini::{parse_event, translate_response};
use crate::config;
use crate::hooks;
use crate::models::{DebugConfig, GeminiDecision, GeminiHookResponse};

pub async fn run(debug_logs: bool) -> Result<()> {
    match run_inner(debug_logs).await {
        Ok(()) => Ok(()),
        Err(err) => {
            eprintln!("Gemini hook runner error: {}", err);
            emit_safe_response(&format!("Gemini hook runner error: {}", err))?;
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
    let gemini_event = parse_event(raw_value)?;

    let project_config = config::Config::load(
        gemini_event
            .event
            .cwd
            .as_ref()
            .map(|p| std::path::Path::new(p.as_str())),
    )?;
    let debug_config = DebugConfig::new(debug_logs, project_config.settings.debug_logs);

    let response = hooks::process_event(gemini_event.event.clone(), &debug_config).await?;
    let mut hook_response = translate_response(&response, &gemini_event);

    if gemini_event.is_tool_event {
        ensure_hook_event_name(&mut hook_response, &gemini_event.hook_event_name);
    }

    let json = serde_json::to_string(&hook_response)?;
    println!("{}", json);

    Ok(())
}

fn ensure_hook_event_name(response: &mut GeminiHookResponse, hook_event_name: &str) {
    match response.tool_input.as_mut() {
        Some(Value::Object(map)) => {
            map.entry("gemini_hook_event_name".to_string())
                .or_insert(Value::String(hook_event_name.to_string()));
        }
        Some(other) => {
            let mut map = serde_json::Map::new();
            map.insert("tool_input".to_string(), other.clone());
            map.insert(
                "gemini_hook_event_name".to_string(),
                Value::String(hook_event_name.to_string()),
            );
            response.tool_input = Some(Value::Object(map));
        }
        None => {}
    }
}

fn emit_safe_response(reason: &str) -> Result<()> {
    let response = GeminiHookResponse {
        decision: GeminiDecision::Allow,
        reason: Some(reason.to_string()),
        continue_: None,
        system_message: None,
        tool_input: None,
    };
    let json = serde_json::to_string(&response)?;
    println!("{}", json);
    Ok(())
}

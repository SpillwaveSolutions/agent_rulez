use anyhow::Result;
use serde_json::Value;
use std::io::{self, Read};

use crate::adapters::opencode::parse_event;
use crate::config;
use crate::models::{DebugConfig, Response};
use crate::opencode::config::OpenCodePluginConfig;
use crate::opencode::dispatcher::OpenCodeDispatcher;

pub async fn run(debug_logs: bool) -> Result<()> {
    match run_inner(debug_logs).await {
        Ok(()) => Ok(()),
        Err(err) => {
            eprintln!("OpenCode hook runner error: {}", err);
            emit_safe_response(&format!("OpenCode hook runner error: {}", err))?;
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
    let opencode_event = parse_event(raw_value)?;

    let plugin_config = OpenCodePluginConfig::load().unwrap_or_default();
    let dispatcher = OpenCodeDispatcher::new(plugin_config);

    let project_config = config::Config::load(
        opencode_event
            .event
            .cwd
            .as_ref()
            .map(|p| std::path::Path::new(p.as_str())),
    )?;
    let debug_config = DebugConfig::new(debug_logs, project_config.settings.debug_logs);

    let translated: Value = dispatcher.dispatch(opencode_event, &debug_config).await?;

    let json = serde_json::to_string(&translated)?;
    println!("{}", json);

    // If decision was deny, exit with code 2
    if let Some(cont) = translated.get("continue").and_then(|v| v.as_bool()) {
        if !cont {
            std::process::exit(2);
        }
    }

    Ok(())
}

fn emit_safe_response(reason: &str) -> Result<()> {
    let mut response = Response::allow();
    response.reason = Some(reason.to_string());
    let json = serde_json::to_string(&response)?;
    println!("{}", json);
    Ok(())
}

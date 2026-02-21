use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::models::{CopilotDecision, CopilotHookResponse, Event, EventType, Response};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CopilotHookInput {
    session_id: String,
    hook_event_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_input: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<String>,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CopilotEvent {
    pub hook_event_name: String,
    pub event: Event,
    pub is_tool_event: bool,
}

#[allow(dead_code)]
pub fn parse_event(value: Value) -> Result<CopilotEvent> {
    let input: CopilotHookInput = serde_json::from_value(value)?;
    let (event_type, is_tool_event) = map_event_type(&input.hook_event_name);
    let preserve_name = input.hook_event_name != event_type.to_string();

    // Map-first pattern: unwrap tool_input to Map, merge extra, then wrap back
    let mut tool_input_map = match input.tool_input {
        Some(Value::Object(map)) => map,
        Some(value) => {
            let mut map = Map::new();
            map.insert("tool_input".to_string(), value);
            map
        }
        None => Map::new(),
    };

    for (key, value) in input.extra {
        tool_input_map.entry(key).or_insert(value);
    }

    if preserve_name {
        tool_input_map
            .entry("copilot_hook_event_name".to_string())
            .or_insert(Value::String(input.hook_event_name.clone()));
    }

    // Canonicalize tool name and preserve original platform name
    let (canonical_tool_name, original_tool_name) = match input.tool_name {
        Some(ref name) => {
            let canonical = map_tool_name(name);
            let original = if canonical != *name {
                Some(name.clone())
            } else {
                None
            };
            (Some(canonical), original)
        }
        None => (None, None),
    };

    // Inject platform_tool_name into tool_input if the name was mapped
    if let Some(ref orig) = original_tool_name {
        tool_input_map.insert(
            "platform_tool_name".to_string(),
            Value::String(orig.clone()),
        );
    }

    let event = Event {
        hook_event_name: event_type,
        tool_name: canonical_tool_name,
        tool_input: if tool_input_map.is_empty() {
            None
        } else {
            Some(Value::Object(tool_input_map))
        },
        session_id: input.session_id,
        timestamp: input.timestamp.unwrap_or_else(Utc::now),
        user_id: input.user_id,
        transcript_path: None,
        cwd: input.cwd,
        permission_mode: None,
        tool_use_id: None,
        prompt: None,
    };

    Ok(CopilotEvent {
        hook_event_name: input.hook_event_name,
        event,
        is_tool_event,
    })
}

#[allow(dead_code)]
pub fn translate_response(
    response: &Response,
    copilot_event: &CopilotEvent,
) -> CopilotHookResponse {
    let decision = if response.continue_ {
        CopilotDecision::Allow
    } else {
        CopilotDecision::Deny
    };

    let mut tool_input_override = None;
    if copilot_event.is_tool_event {
        if let Some(context) = response.context.as_ref() {
            if let Ok(value) = serde_json::from_str::<Value>(context) {
                if value.is_object() {
                    tool_input_override = Some(value);
                }
            }
        }
    }

    let reason = if response.continue_ {
        None
    } else {
        response.reason.clone()
    };

    CopilotHookResponse {
        permission_decision: decision,
        permission_decision_reason: reason,
        tool_input: tool_input_override,
    }
}

#[allow(dead_code)]
fn map_event_type(hook_event_name: &str) -> (EventType, bool) {
    match hook_event_name {
        "preToolUse" => (EventType::PreToolUse, true),
        "postToolUse" => (EventType::PostToolUse, true),
        "promptSubmit" => (EventType::UserPromptSubmit, false),
        "sessionStart" => (EventType::SessionStart, false),
        "sessionEnd" => (EventType::SessionEnd, false),
        "errorOccurred" => (EventType::PostToolUseFailure, false),
        "preCompact" => (EventType::PreCompact, false),
        _ => (EventType::Notification, false),
    }
}

/// Map a Copilot tool name to the canonical (Claude Code) tool name.
///
/// Copilot tool names are mostly PascalCase like Claude Code, but some
/// differ (e.g., `shell` → `Bash`). Unknown names pass through unchanged.
#[allow(dead_code)]
fn map_tool_name(platform_name: &str) -> String {
    match platform_name {
        "shell" => "Bash".to_string(),
        "write" => "Write".to_string(),
        "edit" => "Edit".to_string(),
        "read" => "Read".to_string(),
        "glob" => "Glob".to_string(),
        "grep" => "Grep".to_string(),
        "task" => "Task".to_string(),
        "fetch" => "WebFetch".to_string(),
        _ => platform_name.to_string(),
    }
}

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

    let tool_input = merge_tool_input(
        input.tool_input,
        input.extra,
        preserve_name.then(|| input.hook_event_name.clone()),
    );

    let event = Event {
        hook_event_name: event_type,
        tool_name: input.tool_name,
        tool_input,
        session_id: input.session_id,
        timestamp: input.timestamp.unwrap_or_else(Utc::now),
        user_id: input.user_id,
        transcript_path: None,
        cwd: input.cwd,
        permission_mode: None,
        tool_use_id: None,
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
        _ => (EventType::Notification, false),
    }
}

#[allow(dead_code)]
fn merge_tool_input(
    tool_input: Option<Value>,
    extra: Map<String, Value>,
    preserve_name: Option<String>,
) -> Option<Value> {
    let mut merged = match tool_input {
        Some(Value::Object(map)) => map,
        Some(value) => {
            let mut map = Map::new();
            map.insert("tool_input".to_string(), value);
            map
        }
        None => Map::new(),
    };

    for (key, value) in extra {
        if !merged.contains_key(&key) {
            merged.insert(key, value);
        }
    }

    if let Some(name) = preserve_name {
        merged
            .entry("copilot_hook_event_name".to_string())
            .or_insert(Value::String(name));
    }

    if merged.is_empty() {
        None
    } else {
        Some(Value::Object(merged))
    }
}

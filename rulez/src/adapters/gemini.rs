use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::models::{Event, EventType, GeminiDecision, GeminiHookResponse, Response};

#[derive(Debug, Deserialize)]
struct GeminiHookInput {
    session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    transcript_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cwd: Option<String>,
    hook_event_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_input: Option<Value>,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

#[derive(Debug, Clone)]
pub struct GeminiEvent {
    pub hook_event_name: String,
    pub event: Event,
    pub is_tool_event: bool,
    /// Additional event types to evaluate (dual-fire support).
    /// Consumed by the platform-specific hook runner when processing events.
    #[allow(dead_code)]
    pub additional_event_types: Vec<EventType>,
}

pub fn parse_event(value: Value) -> Result<GeminiEvent> {
    let input: GeminiHookInput = serde_json::from_value(value)?;
    let mappings = map_event_type(
        &input.hook_event_name,
        input.tool_input.as_ref(),
        &input.extra,
    );

    // Primary mapping is always the first one
    let (primary_event_type, is_tool_event) = mappings[0];
    let additional_event_types: Vec<EventType> =
        mappings.iter().skip(1).map(|(et, _)| *et).collect();

    let preserve_name = input.hook_event_name != primary_event_type.to_string();

    let tool_input = merge_tool_input(
        input.tool_input,
        input.extra,
        preserve_name.then(|| input.hook_event_name.clone()),
    );

    let event = Event {
        hook_event_name: primary_event_type,
        tool_name: input.tool_name,
        tool_input,
        session_id: input.session_id,
        timestamp: input.timestamp.unwrap_or_else(Utc::now),
        user_id: input.user_id,
        transcript_path: input.transcript_path,
        cwd: input.cwd,
        permission_mode: None,
        tool_use_id: None,
        prompt: None,
    };

    Ok(GeminiEvent {
        hook_event_name: input.hook_event_name,
        event,
        is_tool_event,
        additional_event_types,
    })
}

pub fn translate_response(response: &Response, gemini_event: &GeminiEvent) -> GeminiHookResponse {
    let decision = if response.continue_ {
        GeminiDecision::Allow
    } else {
        GeminiDecision::Deny
    };

    let mut system_message = None;
    let mut tool_input_override = None;

    if let Some(context) = response.context.as_ref() {
        if gemini_event.is_tool_event {
            if let Ok(value) = serde_json::from_str::<Value>(context) {
                if value.is_object() {
                    tool_input_override = Some(value);
                } else {
                    system_message = Some(context.clone());
                }
            } else {
                system_message = Some(context.clone());
            }
        } else {
            system_message = Some(context.clone());
        }
    }

    let continue_ = if response.continue_ || gemini_event.is_tool_event {
        None
    } else {
        Some(false)
    };

    GeminiHookResponse {
        decision,
        reason: response.reason.clone(),
        continue_,
        system_message,
        tool_input: tool_input_override,
    }
}

/// Map a Gemini hook event name to one or more RuleZ event types.
///
/// Returns a Vec of (EventType, is_tool_event) tuples. The first entry is the primary
/// event type; subsequent entries are dual-fire targets evaluated in addition.
fn map_event_type(
    hook_event_name: &str,
    tool_input: Option<&Value>,
    extra: &Map<String, Value>,
) -> Vec<(EventType, bool)> {
    match hook_event_name {
        "BeforeTool" => vec![(EventType::PreToolUse, true)],
        "AfterTool" => {
            let mut types = vec![(EventType::PostToolUse, true)];
            // Dual-fire: if the tool result indicates failure, also fire PostToolUseFailure
            if is_tool_failure(tool_input, extra) {
                types.push((EventType::PostToolUseFailure, false));
            }
            types
        }
        "BeforeAgent" => {
            // Dual-fire: BeforeAgent also fires UserPromptSubmit (payload has prompt field)
            vec![
                (EventType::BeforeAgent, false),
                (EventType::UserPromptSubmit, false),
            ]
        }
        "AfterAgent" => vec![(EventType::AfterAgent, false)],
        "BeforeModel" => vec![(EventType::BeforeModel, false)],
        "AfterModel" => vec![(EventType::AfterModel, false)],
        "BeforeToolSelection" => vec![(EventType::BeforeToolSelection, false)],
        "SessionStart" => vec![(EventType::SessionStart, false)],
        "SessionEnd" => vec![(EventType::SessionEnd, false)],
        "PreCompact" => vec![(EventType::PreCompact, false)],
        "Notification" => {
            let mut types = vec![(EventType::Notification, false)];
            // Dual-fire: if notification_type is "ToolPermission", also fire PermissionRequest
            if is_tool_permission_notification(tool_input, extra) {
                types.push((EventType::PermissionRequest, false));
            }
            types
        }
        _ => vec![(EventType::Notification, false)],
    }
}

/// Check if a tool result indicates failure (for AfterTool dual-fire)
fn is_tool_failure(tool_input: Option<&Value>, extra: &Map<String, Value>) -> bool {
    if let Some(Value::Object(map)) = tool_input {
        if let Some(Value::Bool(false)) = map.get("success") {
            return true;
        }
        if map.contains_key("error") {
            return true;
        }
    }
    if let Some(Value::Bool(false)) = extra.get("success") {
        return true;
    }
    if extra.contains_key("error") {
        return true;
    }
    false
}

/// Check if a Notification is a ToolPermission notification
fn is_tool_permission_notification(tool_input: Option<&Value>, extra: &Map<String, Value>) -> bool {
    if let Some(Value::Object(map)) = tool_input {
        if let Some(Value::String(nt)) = map.get("notification_type") {
            return nt == "ToolPermission";
        }
    }
    if let Some(Value::String(nt)) = extra.get("notification_type") {
        return nt == "ToolPermission";
    }
    false
}

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
            .entry("gemini_hook_event_name".to_string())
            .or_insert(Value::String(name));
    }

    if merged.is_empty() {
        None
    } else {
        Some(Value::Object(merged))
    }
}

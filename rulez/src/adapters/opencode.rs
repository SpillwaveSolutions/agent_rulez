use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::models::{Event, EventType, Response};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct OpenCodeHookInput {
    session_id: String,
    hook_event_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_input: Option<Value>,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OpenCodeEvent {
    pub hook_event_name: String,
    pub event: Event,
    /// Additional event types to evaluate (dual-fire support)
    pub additional_event_types: Vec<EventType>,
}

pub fn parse_event(value: Value) -> Result<OpenCodeEvent> {
    let input: OpenCodeHookInput = serde_json::from_value(value)?;
    let event_types = map_event_type(
        &input.hook_event_name,
        input.tool_input.as_ref(),
        &input.extra,
    );

    let primary_event_type = event_types[0];
    let additional_event_types: Vec<EventType> = event_types.into_iter().skip(1).collect();

    let mut tool_input = match input.tool_input {
        Some(Value::Object(map)) => map,
        Some(v) => {
            let mut map = Map::new();
            map.insert("tool_input".to_string(), v);
            map
        }
        None => Map::new(),
    };

    for (k, v) in input.extra {
        tool_input.entry(k).or_insert(v);
    }

    let event = Event {
        hook_event_name: primary_event_type,
        tool_name: input.tool_name,
        tool_input: if tool_input.is_empty() {
            None
        } else {
            Some(Value::Object(tool_input))
        },
        session_id: input.session_id,
        timestamp: input.timestamp.unwrap_or_else(Utc::now),
        user_id: None,
        transcript_path: None,
        cwd: input.cwd,
        permission_mode: None,
        tool_use_id: None,
        prompt: None,
    };

    Ok(OpenCodeEvent {
        hook_event_name: input.hook_event_name,
        event,
        additional_event_types,
    })
}

pub fn translate_response(response: &Response, _opencode_event: &OpenCodeEvent) -> Value {
    let mut map = Map::new();
    map.insert("continue".to_string(), Value::Bool(response.continue_));

    if let Some(reason) = &response.reason {
        map.insert("reason".to_string(), Value::String(reason.clone()));
    }

    if let Some(context) = &response.context {
        map.insert("context".to_string(), Value::String(context.clone()));
    }

    // Register tools available in the OpenCode environment
    let mut tools = Vec::new();

    let mut check_tool = Map::new();
    check_tool.insert("name".to_string(), Value::String("rulez.check".to_string()));
    check_tool.insert(
        "description".to_string(),
        Value::String("Run a RuleZ policy check on demand".to_string()),
    );
    tools.push(Value::Object(check_tool));

    let mut explain_tool = Map::new();
    explain_tool.insert(
        "name".to_string(),
        Value::String("rulez.explain".to_string()),
    );
    explain_tool.insert(
        "description".to_string(),
        Value::String("Explain why a policy decision was made".to_string()),
    );
    tools.push(Value::Object(explain_tool));

    map.insert("tools".to_string(), Value::Array(tools));

    Value::Object(map)
}

/// Map an OpenCode hook event name to one or more RuleZ event types.
///
/// Returns a Vec of EventTypes. The first entry is the primary event type;
/// subsequent entries are dual-fire targets evaluated in addition.
fn map_event_type(
    hook_event_name: &str,
    tool_input: Option<&Value>,
    extra: &Map<String, Value>,
) -> Vec<EventType> {
    match hook_event_name {
        "tool.execute.before" => vec![EventType::PreToolUse],
        "tool.execute.after" => {
            let mut types = vec![EventType::PostToolUse];
            // Dual-fire: if payload indicates failure, also fire PostToolUseFailure
            if is_tool_failure(tool_input, extra) {
                types.push(EventType::PostToolUseFailure);
            }
            types
        }
        "session.created" => vec![EventType::SessionStart],
        "session.deleted" => vec![EventType::SessionEnd],
        "session.updated" => vec![EventType::UserPromptSubmit],
        "session.compacted" => vec![EventType::PreCompact],
        _ => vec![EventType::Notification],
    }
}

/// Check if a tool execution result indicates failure
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

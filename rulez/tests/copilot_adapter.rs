use rulez::adapters::copilot::{parse_event, translate_response};
use rulez::models::{CopilotDecision, EventType, Response};
use serde_json::json;

#[test]
fn test_parse_tool_event_maps_pre_tool_use() {
    let input = json!({
        "session_id": "sess-1",
        "hook_event_name": "preToolUse",
        "timestamp": "2026-02-11T12:34:56Z",
        "tool_name": "shell",
        "tool_input": {
            "command": "ls -la"
        }
    });

    let parsed = parse_event(input).expect("parse event");
    assert_eq!(parsed.event.hook_event_name, EventType::PreToolUse);
    // shell is canonicalized to Bash
    assert_eq!(parsed.event.tool_name.as_deref(), Some("Bash"));

    let tool_input = parsed.event.tool_input.expect("tool input");
    let map = tool_input.as_object().expect("tool input object");
    assert_eq!(map.get("command").and_then(|v| v.as_str()), Some("ls -la"));
    assert_eq!(
        map.get("copilot_hook_event_name").and_then(|v| v.as_str()),
        Some("preToolUse")
    );
    // Original platform tool name preserved
    assert_eq!(
        map.get("platform_tool_name").and_then(|v| v.as_str()),
        Some("shell")
    );
}

#[test]
fn test_unknown_tool_passes_through() {
    let input = json!({
        "session_id": "sess-unk",
        "hook_event_name": "preToolUse",
        "tool_name": "custom_tool",
        "tool_input": {"arg": "val"}
    });

    let parsed = parse_event(input).expect("parse event");
    assert_eq!(parsed.event.tool_name.as_deref(), Some("custom_tool"));

    let tool_input = parsed.event.tool_input.expect("tool input");
    let map = tool_input.as_object().expect("tool input object");
    // No platform_tool_name for pass-through tools
    assert!(map.get("platform_tool_name").is_none());
}

#[test]
fn test_pascal_case_tool_passes_through() {
    // Copilot tools already in PascalCase should pass through unchanged
    let input = json!({
        "session_id": "sess-pc",
        "hook_event_name": "preToolUse",
        "tool_name": "Write",
        "tool_input": {"path": "/tmp/f.txt"}
    });

    let parsed = parse_event(input).expect("parse event");
    assert_eq!(parsed.event.tool_name.as_deref(), Some("Write"));

    let tool_input = parsed.event.tool_input.expect("tool input");
    let map = tool_input.as_object().expect("tool input object");
    // No platform_tool_name since it's already canonical
    assert!(map.get("platform_tool_name").is_none());
}

#[test]
fn test_parse_prompt_submit_keeps_prompt_fields() {
    let input = json!({
        "session_id": "sess-2",
        "hook_event_name": "promptSubmit",
        "prompt": "Explain this",
        "prompt_response": "Explanation"
    });

    let parsed = parse_event(input).expect("parse event");
    assert_eq!(parsed.event.hook_event_name, EventType::UserPromptSubmit);

    let tool_input = parsed.event.tool_input.expect("tool input");
    let map = tool_input.as_object().expect("tool input object");
    assert_eq!(
        map.get("prompt").and_then(|v| v.as_str()),
        Some("Explain this")
    );
    assert_eq!(
        map.get("prompt_response").and_then(|v| v.as_str()),
        Some("Explanation")
    );
    assert_eq!(
        map.get("copilot_hook_event_name").and_then(|v| v.as_str()),
        Some("promptSubmit")
    );
}

#[test]
fn test_translate_response_allow_deny() {
    let input = json!({
        "session_id": "sess-3",
        "hook_event_name": "preToolUse",
        "tool_name": "shell",
        "tool_input": {"command": "pwd"}
    });
    let parsed = parse_event(input).expect("parse event");

    let allow = Response::allow();
    let allow_output = translate_response(&allow, &parsed);
    assert_eq!(allow_output.permission_decision, CopilotDecision::Allow);
    assert!(allow_output.permission_decision_reason.is_none());

    let deny = Response::block("policy denied");
    let deny_output = translate_response(&deny, &parsed);
    assert_eq!(deny_output.permission_decision, CopilotDecision::Deny);
    assert_eq!(
        deny_output.permission_decision_reason.as_deref(),
        Some("policy denied")
    );
}

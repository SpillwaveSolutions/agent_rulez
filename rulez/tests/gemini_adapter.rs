use rulez::adapters::gemini::{parse_event, translate_response};
use rulez::models::{EventType, GeminiDecision, Response};
use serde_json::json;

#[test]
fn test_parse_tool_event_maps_pre_tool_use() {
    let input = json!({
        "session_id": "sess-1",
        "hook_event_name": "BeforeTool",
        "timestamp": "2026-02-11T12:34:56Z",
        "tool_name": "write_file",
        "tool_input": {
            "file_path": "/tmp/file.txt",
            "content": "hello"
        }
    });

    let parsed = parse_event(input).expect("parse event");
    assert_eq!(parsed.event.hook_event_name, EventType::PreToolUse);
    // write_file is canonicalized to Write
    assert_eq!(parsed.event.tool_name.as_deref(), Some("Write"));

    let tool_input = parsed.event.tool_input.expect("tool input");
    let map = tool_input.as_object().expect("tool input object");
    assert_eq!(
        map.get("file_path").and_then(|v| v.as_str()),
        Some("/tmp/file.txt")
    );
    assert_eq!(
        map.get("gemini_hook_event_name").and_then(|v| v.as_str()),
        Some("BeforeTool")
    );
    // Original platform tool name preserved
    assert_eq!(
        map.get("platform_tool_name").and_then(|v| v.as_str()),
        Some("write_file")
    );
}

#[test]
fn test_tool_name_canonicalized_bash() {
    let input = json!({
        "session_id": "sess-bash",
        "hook_event_name": "BeforeTool",
        "tool_name": "run_shell_command",
        "tool_input": {"command": "ls"}
    });

    let parsed = parse_event(input).expect("parse event");
    assert_eq!(parsed.event.tool_name.as_deref(), Some("Bash"));

    let tool_input = parsed.event.tool_input.expect("tool input");
    let map = tool_input.as_object().expect("tool input object");
    assert_eq!(
        map.get("platform_tool_name").and_then(|v| v.as_str()),
        Some("run_shell_command")
    );
}

#[test]
fn test_tool_name_canonicalized_grep() {
    let input = json!({
        "session_id": "sess-grep",
        "hook_event_name": "BeforeTool",
        "tool_name": "search_file_content",
        "tool_input": {"pattern": "foo"}
    });

    let parsed = parse_event(input).expect("parse event");
    assert_eq!(parsed.event.tool_name.as_deref(), Some("Grep"));

    let map = parsed
        .event
        .tool_input
        .unwrap()
        .as_object()
        .unwrap()
        .clone();
    assert_eq!(
        map.get("platform_tool_name").and_then(|v| v.as_str()),
        Some("search_file_content")
    );
}

#[test]
fn test_unknown_tool_passes_through() {
    let input = json!({
        "session_id": "sess-unk",
        "hook_event_name": "BeforeTool",
        "tool_name": "mcp__custom__tool",
        "tool_input": {"arg": "val"}
    });

    let parsed = parse_event(input).expect("parse event");
    assert_eq!(parsed.event.tool_name.as_deref(), Some("mcp__custom__tool"));

    let tool_input = parsed.event.tool_input.expect("tool input");
    let map = tool_input.as_object().expect("tool input object");
    // No platform_tool_name for pass-through tools
    assert!(map.get("platform_tool_name").is_none());
}

#[test]
fn test_no_tool_name_event() {
    let input = json!({
        "session_id": "sess-no-tool",
        "hook_event_name": "BeforeAgent"
    });

    let parsed = parse_event(input).expect("parse event");
    assert!(parsed.event.tool_name.is_none());
}

#[test]
fn test_parse_agent_event_keeps_prompt_fields() {
    let input = json!({
        "session_id": "sess-2",
        "hook_event_name": "BeforeAgent",
        "prompt": "Summarize this",
        "prompt_response": "Summary"
    });

    let parsed = parse_event(input).expect("parse event");
    assert_eq!(parsed.event.hook_event_name, EventType::BeforeAgent);

    let tool_input = parsed.event.tool_input.expect("tool input");
    let map = tool_input.as_object().expect("tool input object");
    assert_eq!(
        map.get("prompt").and_then(|v| v.as_str()),
        Some("Summarize this")
    );
    assert_eq!(
        map.get("prompt_response").and_then(|v| v.as_str()),
        Some("Summary")
    );
}

#[test]
fn test_parse_unknown_event_preserves_name() {
    let input = json!({
        "session_id": "sess-3",
        "hook_event_name": "CustomEvent"
    });

    let parsed = parse_event(input).expect("parse event");
    assert_eq!(parsed.event.hook_event_name, EventType::Notification);

    let tool_input = parsed.event.tool_input.expect("tool input");
    let map = tool_input.as_object().expect("tool input object");
    assert_eq!(
        map.get("gemini_hook_event_name").and_then(|v| v.as_str()),
        Some("CustomEvent")
    );
}

#[test]
fn test_translate_response_tool_override_from_json_context() {
    let input = json!({
        "session_id": "sess-4",
        "hook_event_name": "BeforeTool",
        "tool_name": "replace",
        "tool_input": {"file_path": "/tmp/old.txt"}
    });
    let parsed = parse_event(input).expect("parse event");

    let response = Response::inject(r#"{"file_path":"/tmp/new.txt"}"#);
    let output = translate_response(&response, &parsed);

    assert_eq!(output.decision, GeminiDecision::Allow);
    let tool_input = output.tool_input.expect("tool input override");
    let map = tool_input.as_object().expect("tool input object");
    assert_eq!(
        map.get("file_path").and_then(|v| v.as_str()),
        Some("/tmp/new.txt")
    );
    assert!(output.system_message.is_none());
}

#[test]
fn test_translate_response_non_tool_block_sets_continue_false() {
    let input = json!({
        "session_id": "sess-5",
        "hook_event_name": "AfterAgent"
    });
    let parsed = parse_event(input).expect("parse event");

    let response = Response::block("policy denied");
    let output = translate_response(&response, &parsed);

    assert_eq!(output.decision, GeminiDecision::Deny);
    assert_eq!(output.reason.as_deref(), Some("policy denied"));
    assert_eq!(output.continue_, Some(false));
}

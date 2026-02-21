use rulez::adapters::opencode::parse_event;
use rulez::models::EventType;
use serde_json::json;

#[test]
fn test_parse_tool_execute_before() {
    let payload = json!({
        "session_id": "test-session",
        "hook_event_name": "tool.execute.before",
        "tool_name": "bash",
        "tool_input": {
            "command": "ls -la"
        },
        "cwd": "/test/path"
    });

    let result = parse_event(payload).unwrap();
    assert_eq!(result.hook_event_name, "tool.execute.before");
    assert_eq!(result.event.hook_event_name, EventType::PreToolUse);
    // bash is canonicalized to Bash
    assert_eq!(result.event.tool_name, Some("Bash".to_string()));
    assert_eq!(result.event.session_id, "test-session");
    assert_eq!(result.event.cwd, Some("/test/path".to_string()));

    let tool_input = result.event.tool_input.unwrap();
    assert_eq!(tool_input["command"], "ls -la");
    // Original platform tool name preserved
    assert_eq!(tool_input["platform_tool_name"], "bash");
}

#[test]
fn test_tool_name_canonicalized_webfetch() {
    let payload = json!({
        "session_id": "test-session",
        "hook_event_name": "tool.execute.before",
        "tool_name": "webfetch",
        "tool_input": {"url": "https://example.com"}
    });

    let result = parse_event(payload).unwrap();
    assert_eq!(result.event.tool_name, Some("WebFetch".to_string()));

    let tool_input = result.event.tool_input.unwrap();
    assert_eq!(tool_input["platform_tool_name"], "webfetch");
}

#[test]
fn test_tool_name_canonicalized_fetch_alias() {
    let payload = json!({
        "session_id": "test-session",
        "hook_event_name": "tool.execute.before",
        "tool_name": "fetch",
        "tool_input": {"url": "https://example.com"}
    });

    let result = parse_event(payload).unwrap();
    assert_eq!(result.event.tool_name, Some("WebFetch".to_string()));

    let tool_input = result.event.tool_input.unwrap();
    assert_eq!(tool_input["platform_tool_name"], "fetch");
}

#[test]
fn test_unknown_tool_passes_through() {
    let payload = json!({
        "session_id": "test-session",
        "hook_event_name": "tool.execute.before",
        "tool_name": "custom_mcp_tool",
        "tool_input": {"arg": "val"}
    });

    let result = parse_event(payload).unwrap();
    assert_eq!(result.event.tool_name, Some("custom_mcp_tool".to_string()));

    let tool_input = result.event.tool_input.unwrap();
    // No platform_tool_name for pass-through tools
    assert!(tool_input.get("platform_tool_name").is_none());
}

#[test]
fn test_parse_file_edited() {
    let payload = json!({
        "session_id": "test-session",
        "hook_event_name": "file.edited",
        "extra_field": "some-value"
    });

    let result = parse_event(payload).unwrap();
    assert_eq!(result.hook_event_name, "file.edited");
    assert_eq!(result.event.hook_event_name, EventType::Notification);

    let tool_input = result.event.tool_input.unwrap();
    assert_eq!(tool_input["extra_field"], "some-value");
}

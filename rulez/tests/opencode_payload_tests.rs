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
    assert_eq!(result.event.tool_name, Some("bash".to_string()));
    assert_eq!(result.event.session_id, "test-session");
    assert_eq!(result.event.cwd, Some("/test/path".to_string()));

    let tool_input = result.event.tool_input.unwrap();
    assert_eq!(tool_input["command"], "ls -la");
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

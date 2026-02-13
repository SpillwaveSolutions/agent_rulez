use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use std::path::Path;

fn write_hooks_config(path: &Path, contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)?;
    Ok(())
}

#[test]
fn gemini_hook_runner_outputs_json_with_hook_event_name() -> Result<(), Box<dyn std::error::Error>>
{
    let temp_dir = tempfile::tempdir()?;
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir)?;

    let inject_path = temp_dir.path().join("tool_override.json");
    fs::write(&inject_path, r#"{"filePath":"/tmp/override.txt"}"#)?;

    let hooks_yaml = format!(
        "version: \"1.0\"\n\nrules:\n  - name: inject-tool-input\n    description: \"inject override\"\n    matchers:\n      tools: [\"Write\"]\n      operations: [\"PreToolUse\"]\n    actions:\n      inject: \"{}\"\n\nsettings:\n  log_level: \"debug\"\n  fail_open: false\n",
        inject_path.display()
    );
    write_hooks_config(&claude_dir.join("hooks.yaml"), &hooks_yaml)?;

    let input = serde_json::json!({
        "session_id": "sess-1",
        "hook_event_name": "BeforeTool",
        "cwd": temp_dir.path().to_string_lossy(),
        "tool_name": "Write",
        "tool_input": {
            "filePath": "/tmp/file.txt",
            "content": "hello"
        }
    });

    let output = Command::new(assert_cmd::cargo::cargo_bin!("rulez"))
        .current_dir(temp_dir.path())
        .arg("gemini")
        .arg("hook")
        .write_stdin(input.to_string())
        .output()?;

    assert!(output.status.success());

    let payload: Value = serde_json::from_slice(&output.stdout)?;
    assert!(payload.get("decision").is_some());

    let tool_input = payload
        .get("tool_input")
        .and_then(Value::as_object)
        .expect("tool_input missing");
    assert_eq!(
        tool_input
            .get("gemini_hook_event_name")
            .and_then(Value::as_str),
        Some("BeforeTool")
    );

    Ok(())
}

#[test]
fn gemini_hook_runner_denies_with_override() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir()?;
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir)?;

    let inject_path = temp_dir.path().join("deny_override.json");
    fs::write(&inject_path, r#"{"reason":"policy"}"#)?;

    let hooks_yaml = format!(
        "version: \"1.0\"\n\nrules:\n  - name: block-bash\n    description: \"block bash\"\n    priority: 10\n    matchers:\n      tools: [\"Bash\"]\n      command_match: \"git push.*\"\n      operations: [\"PreToolUse\"]\n    actions:\n      block: true\n\n  - name: inject-override\n    description: \"inject override\"\n    priority: 1\n    matchers:\n      tools: [\"Bash\"]\n      operations: [\"PreToolUse\"]\n    actions:\n      inject: \"{}\"\n\nsettings:\n  log_level: \"debug\"\n  fail_open: false\n",
        inject_path.display()
    );
    write_hooks_config(&claude_dir.join("hooks.yaml"), &hooks_yaml)?;

    let input = serde_json::json!({
        "session_id": "sess-2",
        "hook_event_name": "BeforeTool",
        "cwd": temp_dir.path().to_string_lossy(),
        "tool_name": "Bash",
        "tool_input": {
            "command": "git push --force"
        }
    });

    let output = Command::new(assert_cmd::cargo::cargo_bin!("rulez"))
        .current_dir(temp_dir.path())
        .arg("gemini")
        .arg("hook")
        .write_stdin(input.to_string())
        .output()?;

    assert!(output.status.success());

    let payload: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(
        payload.get("decision").and_then(Value::as_str),
        Some("deny")
    );

    let tool_input = payload
        .get("tool_input")
        .and_then(Value::as_object)
        .expect("tool_input missing");
    assert_eq!(
        tool_input
            .get("gemini_hook_event_name")
            .and_then(Value::as_str),
        Some("BeforeTool")
    );

    Ok(())
}

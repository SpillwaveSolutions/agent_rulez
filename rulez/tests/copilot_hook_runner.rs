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
fn copilot_hook_runner_outputs_allow_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir()?;
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir)?;

    let hooks_yaml = "version: \"1.0\"\n\nrules:\n  - name: allow-all\n    description: \"allow everything\"\n    matchers:\n      tools: [\"*\"]\n      operations: [\"PreToolUse\"]\n    actions:\n      allow: true\n\nsettings:\n  log_level: \"debug\"\n  fail_open: true\n";
    write_hooks_config(&claude_dir.join("hooks.yaml"), hooks_yaml)?;

    let input = serde_json::json!({
        "session_id": "sess-1",
        "hook_event_name": "preToolUse",
        "cwd": temp_dir.path().to_string_lossy(),
        "tool_name": "shell",
        "tool_input": {
            "command": "ls -la"
        }
    });

    let output = Command::new(assert_cmd::cargo::cargo_bin!("rulez"))
        .current_dir(temp_dir.path())
        .arg("copilot")
        .arg("hook")
        .write_stdin(input.to_string())
        .output()?;

    assert!(output.status.success());

    let payload: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(
        payload.get("permissionDecision").and_then(Value::as_str),
        Some("allow")
    );

    Ok(())
}

#[test]
fn copilot_hook_runner_denies_blocked_tool() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir()?;
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir)?;

    let hooks_yaml = "version: \"1.0\"\n\nrules:\n  - name: block-bash\n    description: \"block bash\"\n    priority: 10\n    matchers:\n      tools: [\"Bash\"]\n      command_match: \"git push.*\"\n      operations: [\"PreToolUse\"]\n    actions:\n      block: true\n\nsettings:\n  log_level: \"debug\"\n  fail_open: false\n";
    write_hooks_config(&claude_dir.join("hooks.yaml"), hooks_yaml)?;

    let input = serde_json::json!({
        "session_id": "sess-2",
        "hook_event_name": "preToolUse",
        "cwd": temp_dir.path().to_string_lossy(),
        "tool_name": "Bash",
        "tool_input": {
            "command": "git push --force"
        }
    });

    let output = Command::new(assert_cmd::cargo::cargo_bin!("rulez"))
        .current_dir(temp_dir.path())
        .arg("copilot")
        .arg("hook")
        .write_stdin(input.to_string())
        .output()?;

    assert!(output.status.success());

    let payload: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(
        payload.get("permissionDecision").and_then(Value::as_str),
        Some("deny")
    );
    assert!(payload.get("permissionDecisionReason").is_some());

    Ok(())
}

#[test]
fn copilot_hook_runner_handles_empty_stdin() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new(assert_cmd::cargo::cargo_bin!("rulez"))
        .arg("copilot")
        .arg("hook")
        .write_stdin("")
        .output()?;

    assert!(output.status.success());

    let payload: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(
        payload.get("permissionDecision").and_then(Value::as_str),
        Some("allow")
    );
    assert!(
        payload
            .get("permissionDecisionReason")
            .and_then(Value::as_str)
            .unwrap_or("")
            .contains("No input")
    );

    Ok(())
}

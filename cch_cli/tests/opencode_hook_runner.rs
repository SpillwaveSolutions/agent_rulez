use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::process::Stdio;

fn create_project_with_hooks(dir: &std::path::Path) {
    fs::write(
        dir.join("hooks.yaml"),
        "rules: []\nsettings:\n  debug_logs: false\n",
    )
    .unwrap();
}

#[test]
fn opencode_hook_allow_response() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempfile::tempdir()?;
    create_project_with_hooks(tmp.path());

    let input = r#"{"session_id":"test","hook_event_name":"tool.execute.before","tool_name":"bash","tool_input":{"command":"echo hello"},"cwd":"."}"#;

    let mut child = std::process::Command::new(assert_cmd::cargo::cargo_bin!("cch"))
        .current_dir(tmp.path())
        .arg("opencode")
        .arg("hook")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    child.stdin.as_mut().unwrap().write_all(input.as_bytes())?;
    let output = child.wait_with_output()?;

    assert!(output.status.success());
    let response: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(response["continue"], true);

    Ok(())
}

#[test]
fn opencode_hook_empty_stdin() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempfile::tempdir()?;
    create_project_with_hooks(tmp.path());

    let output = Command::new(assert_cmd::cargo::cargo_bin!("cch"))
        .current_dir(tmp.path())
        .arg("opencode")
        .arg("hook")
        .write_stdin("")
        .output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("continue"));

    Ok(())
}

#[test]
fn opencode_hook_file_edited_event() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempfile::tempdir()?;
    create_project_with_hooks(tmp.path());

    let input =
        r#"{"session_id":"test","hook_event_name":"file.edited","file_path":"/test/file.rs"}"#;

    let mut child = std::process::Command::new(assert_cmd::cargo::cargo_bin!("cch"))
        .current_dir(tmp.path())
        .arg("opencode")
        .arg("hook")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    child.stdin.as_mut().unwrap().write_all(input.as_bytes())?;
    let output = child.wait_with_output()?;

    assert!(output.status.success());
    let response: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(response["continue"], true);

    Ok(())
}

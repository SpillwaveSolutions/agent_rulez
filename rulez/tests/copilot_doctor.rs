use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use std::path::Path;

fn write_hook_file(path: &Path, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}

#[test]
fn copilot_doctor_reports_installed_status() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir()?;
    let hooks_dir = temp_dir.path().join(".github/hooks");

    let hook_json = serde_json::json!({
        "version": 1,
        "hooks": {
            "preToolUse": [
                {
                    "type": "command",
                    "bash": "/usr/local/bin/cch copilot hook",
                    "timeoutSec": 10
                }
            ]
        }
    });
    write_hook_file(
        &hooks_dir.join("rulez.json"),
        &serde_json::to_string_pretty(&hook_json)?,
    )?;

    let output = Command::new(assert_cmd::cargo::cargo_bin!("rulez"))
        .current_dir(temp_dir.path())
        .arg("copilot")
        .arg("doctor")
        .arg("--json")
        .output()?;

    assert!(output.status.success());

    let report: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(report["hooks_dir_exists"], true);

    let files = report
        .get("hook_files")
        .and_then(Value::as_array)
        .expect("hook_files array missing");
    assert!(!files.is_empty());

    let rulez_file = files
        .iter()
        .find(|f| f.get("name").and_then(Value::as_str) == Some("rulez.json"))
        .expect("rulez.json not found");
    assert_eq!(rulez_file["status"], "installed");

    Ok(())
}

#[test]
fn copilot_doctor_reports_missing_hooks_dir() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir()?;

    let output = Command::new(assert_cmd::cargo::cargo_bin!("rulez"))
        .current_dir(temp_dir.path())
        .arg("copilot")
        .arg("doctor")
        .arg("--json")
        .output()?;

    assert!(output.status.success());

    let report: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(report["hooks_dir_exists"], false);

    let files = report
        .get("hook_files")
        .and_then(Value::as_array)
        .expect("hook_files array missing");
    assert!(files.is_empty());

    Ok(())
}

#[test]
fn copilot_doctor_reports_misconfigured_hooks() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir()?;
    let hooks_dir = temp_dir.path().join(".github/hooks");

    let hook_json = serde_json::json!({
        "version": 1,
        "hooks": {
            "preToolUse": [
                {
                    "type": "command",
                    "bash": "/usr/local/bin/other-tool",
                    "timeoutSec": 10
                }
            ]
        }
    });
    write_hook_file(
        &hooks_dir.join("other.json"),
        &serde_json::to_string_pretty(&hook_json)?,
    )?;

    let output = Command::new(assert_cmd::cargo::cargo_bin!("rulez"))
        .current_dir(temp_dir.path())
        .arg("copilot")
        .arg("doctor")
        .arg("--json")
        .output()?;

    assert!(output.status.success());

    let report: Value = serde_json::from_slice(&output.stdout)?;
    let files = report
        .get("hook_files")
        .and_then(Value::as_array)
        .expect("hook_files array missing");

    let other_file = files
        .iter()
        .find(|f| f.get("name").and_then(Value::as_str) == Some("other.json"))
        .expect("other.json not found");
    assert_eq!(other_file["status"], "misconfigured");

    Ok(())
}

#[test]
fn copilot_doctor_reports_outdated_cch_hooks() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir()?;
    let hooks_dir = temp_dir.path().join(".github/hooks");

    let hook_json = serde_json::json!({
        "version": 1,
        "hooks": {
            "preToolUse": [
                {
                    "type": "command",
                    "bash": "/usr/local/bin/cch hook",
                    "timeoutSec": 10
                }
            ]
        }
    });
    write_hook_file(
        &hooks_dir.join("old.json"),
        &serde_json::to_string_pretty(&hook_json)?,
    )?;

    let output = Command::new(assert_cmd::cargo::cargo_bin!("rulez"))
        .current_dir(temp_dir.path())
        .arg("copilot")
        .arg("doctor")
        .arg("--json")
        .output()?;

    assert!(output.status.success());

    let report: Value = serde_json::from_slice(&output.stdout)?;
    let files = report
        .get("hook_files")
        .and_then(Value::as_array)
        .expect("hook_files array missing");

    let old_file = files
        .iter()
        .find(|f| f.get("name").and_then(Value::as_str) == Some("old.json"))
        .expect("old.json not found");
    assert_eq!(old_file["status"], "misconfigured");

    let details = old_file
        .get("details")
        .and_then(Value::as_str)
        .expect("details missing");
    assert!(details.contains("docs/COPILOT_CLI_HOOKS.md"));
    assert!(details.contains("cch copilot install"));

    Ok(())
}

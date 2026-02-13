use assert_cmd::Command;
use serde_json::Value;
use std::fs;

#[test]
fn opencode_doctor_missing_configs() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempfile::tempdir()?;

    let output = Command::new(assert_cmd::cargo::cargo_bin!("cch"))
        .current_dir(tmp.path())
        .arg("opencode")
        .arg("doctor")
        .output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("MISSING"));

    Ok(())
}

#[test]
fn opencode_doctor_json_output() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempfile::tempdir()?;

    let output = Command::new(assert_cmd::cargo::cargo_bin!("cch"))
        .current_dir(tmp.path())
        .arg("opencode")
        .arg("doctor")
        .arg("--json")
        .output()?;

    assert!(output.status.success());
    let report: Value = serde_json::from_slice(&output.stdout)?;
    assert!(report.get("scopes").is_some());
    assert!(report.get("summary").is_some());

    Ok(())
}

#[test]
fn opencode_doctor_installed_project() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempfile::tempdir()?;
    let opencode_dir = tmp.path().join(".opencode");
    fs::create_dir_all(&opencode_dir)?;
    fs::write(
        opencode_dir.join("settings.json"),
        r#"{
            "hooks": {
                "tool.execute.before": [{"type": "command", "command": "cch opencode hook", "timeout": 5}],
                "tool.execute.after": [{"type": "command", "command": "cch opencode hook", "timeout": 5}],
                "file.edited": [{"type": "command", "command": "cch opencode hook", "timeout": 5}],
                "session.updated": [{"type": "command", "command": "cch opencode hook", "timeout": 5}]
            }
        }"#,
    )?;

    let output = Command::new(assert_cmd::cargo::cargo_bin!("cch"))
        .current_dir(tmp.path())
        .arg("opencode")
        .arg("doctor")
        .arg("--json")
        .output()?;

    assert!(output.status.success());
    let report: Value = serde_json::from_slice(&output.stdout)?;
    let scopes = report["scopes"].as_array().unwrap();
    let project = &scopes[0];
    assert_eq!(project["scope"], "project");
    assert_eq!(project["status"], "installed");

    Ok(())
}

#[test]
fn opencode_doctor_misconfigured_outdated() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempfile::tempdir()?;
    let opencode_dir = tmp.path().join(".opencode");
    fs::create_dir_all(&opencode_dir)?;
    fs::write(
        opencode_dir.join("settings.json"),
        r#"{
            "hooks": {
                "tool.execute.before": [{"type": "command", "command": "cch hook", "timeout": 5}]
            }
        }"#,
    )?;

    let output = Command::new(assert_cmd::cargo::cargo_bin!("cch"))
        .current_dir(tmp.path())
        .arg("opencode")
        .arg("doctor")
        .arg("--json")
        .output()?;

    assert!(output.status.success());
    let report: Value = serde_json::from_slice(&output.stdout)?;
    let scopes = report["scopes"].as_array().unwrap();
    let project = &scopes[0];
    assert_eq!(project["status"], "misconfigured");

    Ok(())
}

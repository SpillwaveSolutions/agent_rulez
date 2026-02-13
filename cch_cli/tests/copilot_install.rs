use assert_cmd::Command;
use serde_json::Value;
use std::fs;

fn cch_cmd() -> Command {
    assert_cmd::cargo::cargo_bin_cmd!("cch")
}

#[test]
fn copilot_install_creates_hooks_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir()?;
    let hooks_path = temp_dir.path().join(".github/hooks/rulez.json");

    let binary = assert_cmd::cargo::cargo_bin!("cch");

    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["copilot", "install", "--binary", binary.to_str().unwrap()])
        .assert()
        .success();

    assert!(hooks_path.exists(), "Should create rulez.json");

    let contents = fs::read_to_string(&hooks_path)?;
    let value: Value = serde_json::from_str(&contents)?;

    assert_eq!(value.get("version").and_then(Value::as_u64), Some(1));

    let hooks = value.get("hooks").and_then(Value::as_object).unwrap();
    assert!(hooks.contains_key("preToolUse"));
    assert!(hooks.contains_key("postToolUse"));

    let pre_tool = hooks.get("preToolUse").and_then(Value::as_array).unwrap();
    let serialized = serde_json::to_string(pre_tool)?;
    assert!(
        serialized.contains("copilot hook"),
        "Should reference cch copilot hook"
    );

    Ok(())
}

#[test]
fn copilot_install_merges_existing_hooks() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir()?;
    let hooks_dir = temp_dir.path().join(".github/hooks");
    fs::create_dir_all(&hooks_dir)?;

    let existing = serde_json::json!({
        "version": 1,
        "hooks": {
            "preToolUse": [
                {
                    "type": "command",
                    "bash": "/usr/local/bin/other-hook",
                    "timeoutSec": 5
                }
            ]
        }
    });
    fs::write(
        hooks_dir.join("rulez.json"),
        serde_json::to_string_pretty(&existing)?,
    )?;

    let binary = assert_cmd::cargo::cargo_bin!("cch");

    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["copilot", "install", "--binary", binary.to_str().unwrap()])
        .assert()
        .success();

    let contents = fs::read_to_string(hooks_dir.join("rulez.json"))?;
    let value: Value = serde_json::from_str(&contents)?;

    let pre_tool = value
        .get("hooks")
        .and_then(|h| h.get("preToolUse"))
        .and_then(Value::as_array)
        .unwrap();

    let serialized = serde_json::to_string(pre_tool)?;
    assert!(
        serialized.contains("/usr/local/bin/other-hook"),
        "Should preserve existing non-cch hooks"
    );
    assert!(
        serialized.contains("copilot hook"),
        "Should add cch copilot hook runner"
    );

    Ok(())
}

#[test]
fn copilot_install_prints_snippet_without_writing() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir()?;
    let hooks_path = temp_dir.path().join(".github/hooks/rulez.json");

    let binary = assert_cmd::cargo::cargo_bin!("cch");

    let output = cch_cmd()
        .current_dir(temp_dir.path())
        .args([
            "copilot",
            "install",
            "--print",
            "--binary",
            binary.to_str().unwrap(),
        ])
        .output()?;

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("copilot hook"));
    assert!(!hooks_path.exists(), "Should not write hooks file");

    Ok(())
}

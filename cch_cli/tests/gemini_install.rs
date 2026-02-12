use assert_cmd::Command;
use serde_json::Value;
use std::fs;

fn cch_cmd() -> Command {
    Command::cargo_bin("cch").unwrap()
}

#[test]
fn gemini_install_merges_existing_hooks() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir()?;
    let settings_path = temp_dir.path().join(".gemini/settings.json");

    let existing = serde_json::json!({
        "hooks": {
            "BeforeTool": [
                {
                    "matcher": ".*",
                    "hooks": [
                        { "type": "command", "command": "/usr/local/bin/other" }
                    ]
                }
            ]
        },
        "extra": { "keep": true }
    });
    fs::create_dir_all(settings_path.parent().unwrap())?;
    fs::write(&settings_path, serde_json::to_string_pretty(&existing)?)?;

    let binary = assert_cmd::cargo::cargo_bin("cch");

    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["gemini", "install", "--binary", binary.to_str().unwrap()])
        .assert()
        .success();

    let contents = fs::read_to_string(&settings_path)?;
    let value: Value = serde_json::from_str(&contents)?;

    let hooks = value.get("hooks").and_then(Value::as_object).unwrap();
    let before_tool = hooks.get("BeforeTool").and_then(Value::as_array).unwrap();

    let serialized = serde_json::to_string(before_tool)?;
    assert!(
        serialized.contains("/usr/local/bin/other"),
        "Should preserve existing non-cch hooks"
    );
    assert!(
        serialized.contains("gemini hook"),
        "Should add cch gemini hook runner"
    );

    Ok(())
}

#[test]
fn gemini_install_prints_snippet_without_writing() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir()?;
    let settings_path = temp_dir.path().join(".gemini/settings.json");

    let binary = assert_cmd::cargo::cargo_bin("cch");

    let output = cch_cmd()
        .current_dir(temp_dir.path())
        .args([
            "gemini",
            "install",
            "--print",
            "--binary",
            binary.to_str().unwrap(),
        ])
        .output()?;

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("gemini hook"));
    assert!(!settings_path.exists(), "Should not write settings.json");

    Ok(())
}

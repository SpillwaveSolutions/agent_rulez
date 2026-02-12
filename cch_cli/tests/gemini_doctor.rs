use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use std::path::Path;

fn write_settings(path: &Path, command: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let settings = serde_json::json!({
        "hooks": {
            "BeforeTool": [
                {
                    "matcher": ".*",
                    "hooks": [
                        { "type": "command", "command": command }
                    ]
                }
            ]
        }
    });

    let contents = serde_json::to_string_pretty(&settings)?;
    fs::write(path, contents)?;
    Ok(())
}

fn find_scope<'a>(scopes: &'a [Value], name: &str) -> &'a Value {
    scopes
        .iter()
        .find(|value| value.get("scope").and_then(Value::as_str) == Some(name))
        .unwrap_or_else(|| panic!("Scope '{}' not found", name))
}

fn find_hook_file<'a>(entries: &'a [Value], name: &str) -> &'a Value {
    entries
        .iter()
        .find(|value| value.get("name").and_then(Value::as_str) == Some(name))
        .unwrap_or_else(|| panic!("Hook file '{}' not found", name))
}

#[test]
fn gemini_doctor_reports_scope_and_extension_statuses() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = tempfile::tempdir()?;
    let home_dir = tempfile::tempdir()?;
    let system_dir = tempfile::tempdir()?;

    let project_settings = project_dir.path().join(".gemini/settings.json");
    write_settings(&project_settings, "/usr/local/bin/cch gemini hook")?;

    let user_settings = home_dir.path().join(".gemini/settings.json");
    write_settings(&user_settings, "/usr/local/bin/other")?;

    let system_settings = system_dir.path().join("settings.json");
    write_settings(&system_settings, "/usr/local/bin/cch gemini hook")?;

    let extension_hooks = home_dir
        .path()
        .join(".gemini/extensions/test-ext/hooks/hooks.json");
    write_settings(&extension_hooks, "/usr/local/bin/cch gemini hook")?;

    let shared_hooks = home_dir.path().join(".gemini/hooks/shared.json");
    write_settings(&shared_hooks, "/usr/local/bin/other")?;

    let output = Command::new(assert_cmd::cargo::cargo_bin!("cch"))
        .current_dir(project_dir.path())
        .env("HOME", home_dir.path())
        .env("GEMINI_SYSTEM_SETTINGS_PATH", &system_settings)
        .arg("gemini")
        .arg("doctor")
        .arg("--json")
        .output()?;

    assert!(output.status.success());

    let report: Value = serde_json::from_slice(&output.stdout)?;
    let scopes = report
        .get("scopes")
        .and_then(Value::as_array)
        .expect("scopes array missing");

    let project_scope = find_scope(scopes, "project");
    assert_eq!(project_scope["status"], "installed");

    let user_scope = find_scope(scopes, "user");
    assert_eq!(user_scope["status"], "misconfigured");

    let system_scope = find_scope(scopes, "system");
    assert_eq!(system_scope["status"], "installed");

    let extensions = report.get("extensions").expect("extensions missing");

    let extension_entries = extensions
        .get("extensions")
        .and_then(Value::as_array)
        .expect("extensions array missing");
    let extension_report = find_hook_file(extension_entries, "test-ext");
    assert_eq!(extension_report["status"], "installed");

    let shared_entries = extensions
        .get("shared_hooks")
        .and_then(Value::as_array)
        .expect("shared_hooks array missing");
    let shared_report = find_hook_file(shared_entries, "shared.json");
    assert_eq!(shared_report["status"], "misconfigured");

    Ok(())
}

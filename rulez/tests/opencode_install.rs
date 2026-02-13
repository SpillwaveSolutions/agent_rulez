use assert_cmd::Command;
use serde_json::Value;

fn cch_cmd() -> Command {
    assert_cmd::cargo::cargo_bin_cmd!("rulez")
}

#[test]
fn opencode_install_print_mode_shows_json() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempfile::tempdir()?;
    let binary = assert_cmd::cargo::cargo_bin!("rulez");

    let output = cch_cmd()
        .current_dir(tmp.path())
        .args([
            "opencode",
            "install",
            "--print",
            "--binary",
            binary.to_str().unwrap(),
        ])
        .output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("tool.execute.before"));
    assert!(stdout.contains("file.edited"));
    assert!(stdout.contains("opencode hook"));

    Ok(())
}

#[test]
fn opencode_install_print_contains_all_events() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempfile::tempdir()?;
    let binary = assert_cmd::cargo::cargo_bin!("rulez");

    let output = cch_cmd()
        .current_dir(tmp.path())
        .args([
            "opencode",
            "install",
            "--print",
            "--binary",
            binary.to_str().unwrap(),
        ])
        .output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("tool.execute.before"));
    assert!(stdout.contains("tool.execute.after"));
    assert!(stdout.contains("file.edited"));
    assert!(stdout.contains("session.updated"));

    Ok(())
}

#[test]
fn opencode_install_print_is_valid_json() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = tempfile::tempdir()?;
    let binary = assert_cmd::cargo::cargo_bin!("rulez");

    let output = cch_cmd()
        .current_dir(tmp.path())
        .args([
            "opencode",
            "install",
            "--print",
            "--binary",
            binary.to_str().unwrap(),
        ])
        .output()?;

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: Value = serde_json::from_str(&stdout)?;
    assert!(parsed.get("hooks").is_some());

    Ok(())
}

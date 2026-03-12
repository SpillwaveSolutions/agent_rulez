//! Integration tests for the `rulez lint` command

#![allow(deprecated)] // cargo_bin deprecation - matches other test files

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn rulez_cmd() -> Command {
    Command::cargo_bin("rulez").unwrap()
}

/// Initialize a temp directory with default hooks config
fn init_project(dir: &TempDir) {
    rulez_cmd()
        .current_dir(dir.path())
        .args(["init"])
        .assert()
        .success();
}

#[test]
fn lint_help_works() {
    rulez_cmd()
        .args(["lint", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rule quality"))
        .stdout(predicate::str::contains("--verbose"));
}

#[test]
fn lint_valid_config_exits_zero() {
    let temp_dir = TempDir::new().unwrap();
    init_project(&temp_dir);

    rulez_cmd()
        .current_dir(temp_dir.path())
        .args(["lint", "--config", ".claude/hooks.yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Rule Quality Analysis"));
}

#[test]
fn lint_duplicate_names_exits_nonzero() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&config_dir).unwrap();

    // Config with duplicate rule names — Config::from_file rejects this
    // so lint exits with failure and the error mentions the duplicate
    let config = r#"
version: "1.0"
rules:
  - name: "my-rule"
    description: "First rule"
    matchers:
      tools: ["Bash"]
    actions:
      block: true
  - name: "my-rule"
    description: "Duplicate rule"
    matchers:
      tools: ["Bash"]
    actions:
      block: true
"#;
    fs::write(config_dir.join("hooks.yaml"), config).unwrap();

    rulez_cmd()
        .current_dir(temp_dir.path())
        .args(["lint", "--config", ".claude/hooks.yaml"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Duplicate rule name"));
}

#[test]
fn lint_no_description_warning() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&config_dir).unwrap();

    let config = r#"
version: "1.0"
rules:
  - name: "no-desc-rule"
    matchers:
      tools: ["Bash"]
    actions:
      block: true
"#;
    fs::write(config_dir.join("hooks.yaml"), config).unwrap();

    rulez_cmd()
        .current_dir(temp_dir.path())
        .args(["lint", "--config", ".claude/hooks.yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[WARN]  no-description"));
}

#[test]
fn lint_conflicting_actions_error() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&config_dir).unwrap();

    let config = r#"
version: "1.0"
rules:
  - name: "conflict-rule"
    description: "Has both block and inject"
    matchers:
      tools: ["Bash"]
    actions:
      block: true
      inject_inline: "some context"
"#;
    fs::write(config_dir.join("hooks.yaml"), config).unwrap();

    rulez_cmd()
        .current_dir(temp_dir.path())
        .args(["lint", "--config", ".claude/hooks.yaml"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("[ERROR] conflicting-actions"));
}

#[test]
fn lint_missing_priority_info() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&config_dir).unwrap();

    let config = r#"
version: "1.0"
rules:
  - name: "no-priority-rule"
    description: "No priority set"
    matchers:
      tools: ["Bash"]
    actions:
      block: true
"#;
    fs::write(config_dir.join("hooks.yaml"), config).unwrap();

    rulez_cmd()
        .current_dir(temp_dir.path())
        .args(["lint", "--config", ".claude/hooks.yaml"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[INFO]  missing-priority"));
}

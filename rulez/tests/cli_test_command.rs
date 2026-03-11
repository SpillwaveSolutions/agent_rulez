//! Integration tests for the `rulez test` batch command

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
fn test_command_help() {
    rulez_cmd()
        .args(["test", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test scenarios"))
        .stdout(predicate::str::contains("TEST_FILE"));
}

#[test]
fn test_all_passing_scenarios() {
    let temp_dir = TempDir::new().unwrap();
    init_project(&temp_dir);

    // Create test file with scenarios that match default config
    let test_yaml = temp_dir.path().join("test-scenarios.yaml");
    let content = r#"
tests:
  - name: "block force push"
    event_type: PreToolUse
    tool: Bash
    command: "git push --force origin main"
    expected: block

  - name: "allow echo"
    event_type: PreToolUse
    tool: Bash
    command: "echo hello"
    expected: allow
"#;
    fs::write(&test_yaml, content).unwrap();

    rulez_cmd()
        .current_dir(temp_dir.path())
        .args(["test", test_yaml.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("2 passed, 0 failed, 2 total"));
}

#[test]
fn test_failing_scenario_exits_nonzero() {
    let temp_dir = TempDir::new().unwrap();
    init_project(&temp_dir);

    // Create test file with a wrong expectation
    let test_yaml = temp_dir.path().join("test-fail.yaml");
    let content = r#"
tests:
  - name: "should fail - echo is not blocked"
    event_type: PreToolUse
    tool: Bash
    command: "echo hello"
    expected: block
"#;
    fs::write(&test_yaml, content).unwrap();

    rulez_cmd()
        .current_dir(temp_dir.path())
        .args(["test", test_yaml.to_str().unwrap()])
        .assert()
        .failure()
        .stdout(predicate::str::contains("FAIL"))
        .stdout(predicate::str::contains("0 passed, 1 failed, 1 total"));
}

#[test]
fn test_verbose_shows_details() {
    let temp_dir = TempDir::new().unwrap();
    init_project(&temp_dir);

    // Create test file with a failing test to see verbose output
    let test_yaml = temp_dir.path().join("test-verbose.yaml");
    let content = r#"
tests:
  - name: "wrong expectation"
    event_type: PreToolUse
    tool: Bash
    command: "git push --force origin main"
    expected: allow
"#;
    fs::write(&test_yaml, content).unwrap();

    rulez_cmd()
        .current_dir(temp_dir.path())
        .args(["test", test_yaml.to_str().unwrap(), "--verbose"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("FAIL"))
        .stdout(predicate::str::contains("expected: allow, actual: block"));
}

#[test]
fn test_missing_file_errors() {
    let temp_dir = TempDir::new().unwrap();
    init_project(&temp_dir);

    rulez_cmd()
        .current_dir(temp_dir.path())
        .args(["test", "nonexistent.yaml"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read test file"));
}

#[test]
fn test_invalid_yaml_errors() {
    let temp_dir = TempDir::new().unwrap();
    init_project(&temp_dir);

    let test_yaml = temp_dir.path().join("bad.yaml");
    fs::write(&test_yaml, "not: valid: yaml: [[[").unwrap();

    rulez_cmd()
        .current_dir(temp_dir.path())
        .args(["test", test_yaml.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse test file"));
}

#[test]
fn test_mixed_pass_fail() {
    let temp_dir = TempDir::new().unwrap();
    init_project(&temp_dir);

    let test_yaml = temp_dir.path().join("test-mixed.yaml");
    let content = r#"
tests:
  - name: "correct block"
    event_type: PreToolUse
    tool: Bash
    command: "git push --force origin main"
    expected: block

  - name: "wrong allow"
    event_type: PreToolUse
    tool: Bash
    command: "git push --force origin master"
    expected: allow

  - name: "correct allow"
    event_type: PreToolUse
    tool: Bash
    command: "echo hello"
    expected: allow
"#;
    fs::write(&test_yaml, content).unwrap();

    rulez_cmd()
        .current_dir(temp_dir.path())
        .args(["test", test_yaml.to_str().unwrap()])
        .assert()
        .failure()
        .stdout(predicate::str::contains("2 passed, 1 failed, 3 total"));
}

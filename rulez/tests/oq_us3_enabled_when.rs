//! Operational Qualification (OQ) Tests - enabled_when Conditional Activation
//!
//! Phase 3: Tests for conditional rule activation using enabled_when expressions.
//!
//! These tests verify:
//! - Rules with true enabled_when conditions are active
//! - Rules with false enabled_when conditions are skipped
//! - Expression context includes tool_name and event_type
//! - CLI validation catches invalid enabled_when expressions
//! - Logical operators (&&, ||) work correctly in expressions

#![allow(deprecated)] // cargo_bin deprecation

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[path = "common/mod.rs"]
mod common;
use common::{evidence_dir, TestEvidence, Timer};

/// Test that a rule with enabled_when: 'true' is active and blocks
#[test]
fn test_enabled_when_true_rule_active() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("enabled_when_true_active", "OQ-US3-WHEN");

    // Setup test environment with config that has enabled_when: 'true'
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Create hooks.yaml with enabled_when: 'true' (always active)
    let config = r#"
version: "1.0"
rules:
  - name: always-block
    enabled_when: 'true'
    matchers:
      tools: [Bash]
      command_match: "git push"
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Create event
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {
            "command": "git push origin main"
        },
        "session_id": "test-session"
    }"#;

    // Run rulez with the event
    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    // Should block (exit code 2) because enabled_when is true
    assert_eq!(
        output.status.code(),
        Some(2),
        "Rule with enabled_when='true' should block. stdout: {}, stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("always-block") || stderr.contains("Blocked"),
        "stderr should mention the rule or blocking: {}",
        stderr
    );

    evidence.pass(
        "Rule with enabled_when='true' correctly blocked the operation",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that a rule with enabled_when: 'false' is skipped
#[test]
fn test_enabled_when_false_rule_skipped() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("enabled_when_false_skipped", "OQ-US3-WHEN");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Create hooks.yaml with enabled_when: 'false' (never active)
    let config = r#"
version: "1.0"
rules:
  - name: never-block
    enabled_when: 'false'
    matchers:
      tools: [Bash]
      command_match: "git push"
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Create event that would match the rule if it were active
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {
            "command": "git push origin main"
        },
        "session_id": "test-session"
    }"#;

    // Run rulez
    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .assert()
        .success(); // Should allow because rule is disabled

    // Response should allow the operation
    result.stdout(
        predicate::str::contains(r#""continue":true"#)
            .or(predicate::str::contains(r#""continue": true"#)),
    );

    evidence.pass(
        "Rule with enabled_when='false' was correctly skipped, operation allowed",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that tool_name context variable works in enabled_when expressions
#[test]
fn test_enabled_when_tool_name_condition() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("enabled_when_tool_name", "OQ-US3-WHEN");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Create hooks.yaml that only activates for Bash tool
    let config = r#"
version: "1.0"
rules:
  - name: bash-only-block
    enabled_when: 'tool_name == "Bash"'
    matchers:
      tools: [Bash, Edit]
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Test 1: Bash event should trigger the rule
    let bash_event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {"command": "echo hello"},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(bash_event)
        .output()
        .expect("command should run");

    assert_eq!(
        output.status.code(),
        Some(2),
        "Bash tool should trigger block because tool_name=='Bash'"
    );

    // Test 2: Edit event should not trigger the rule (enabled_when is false for Edit)
    let edit_event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Edit",
        "tool_input": {"filePath": "/test.txt"},
        "session_id": "test-session"
    }"#;

    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(edit_event)
        .assert()
        .success();

    result.stdout(
        predicate::str::contains(r#""continue":true"#)
            .or(predicate::str::contains(r#""continue": true"#)),
    );

    evidence.pass(
        "tool_name context variable works correctly in enabled_when expressions",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that rulez validate command catches invalid enabled_when expressions
#[test]
fn test_validate_invalid_enabled_when() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("validate_invalid_enabled_when", "OQ-US3-WHEN");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Create hooks.yaml with invalid enabled_when expression (unclosed parenthesis)
    let config = r#"
version: "1.0"
rules:
  - name: invalid-syntax
    enabled_when: 'env_CI == ("true'
    matchers:
      tools: [Bash]
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Run validate command
    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .arg("validate")
        .current_dir(temp_dir.path())
        .output()
        .expect("command should run");

    // Should fail validation
    assert!(
        !output.status.success(),
        "Validation should fail for invalid expression"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{} {}", stderr, stdout);

    // Error should mention the rule name
    assert!(
        combined.contains("invalid-syntax") || combined.contains("enabled_when"),
        "Error should mention rule name or enabled_when: stderr={}, stdout={}",
        stderr,
        stdout
    );

    evidence.pass(
        "rulez validate correctly catches invalid enabled_when expressions",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that logical operators work in enabled_when expressions
#[test]
fn test_enabled_when_logical_operators() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("enabled_when_logical_operators", "OQ-US3-WHEN");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Create hooks.yaml with && expression (always false because both conditions can't be true)
    let config = r#"
version: "1.0"
rules:
  - name: complex-condition
    enabled_when: 'tool_name == "Bash" && tool_name == "Edit"'
    matchers:
      tools: [Bash]
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Create Bash event - this would match matchers but enabled_when is false
    // because tool_name can't be both Bash AND Edit
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {"command": "echo hello"},
        "session_id": "test-session"
    }"#;

    // Run rulez
    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .assert()
        .success();

    // Should allow because enabled_when is false (impossible condition)
    result.stdout(
        predicate::str::contains(r#""continue":true"#)
            .or(predicate::str::contains(r#""continue": true"#)),
    );

    // Now test with || operator (always true for Bash)
    let or_config = r#"
version: "1.0"
rules:
  - name: or-condition
    enabled_when: 'tool_name == "Bash" || tool_name == "Edit"'
    matchers:
      tools: [Bash]
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), or_config).expect("Failed to write config");

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    // Should block because tool_name is Bash (matches first condition of ||)
    assert_eq!(
        output.status.code(),
        Some(2),
        "Rule with || should block when first condition is true"
    );

    evidence.pass(
        "Logical operators && and || work correctly in enabled_when expressions",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

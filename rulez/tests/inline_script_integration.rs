//! Inline Script Block Integration Tests (OQ - Operational Qualification)
//!
//! End-to-end tests verifying inline script blocks through the full stack:
//! YAML config -> Config::from_file -> process_event -> Response
//!
//! Requirements covered: SCRIPT-01, SCRIPT-02, SCRIPT-03, SCRIPT-04, SCRIPT-05, SCRIPT-06
//!
//! SCRIPT-01: validate_expr in YAML config evaluates expressions before actions
//! SCRIPT-02: Custom functions get_field() and has_field() in expressions
//! SCRIPT-03: Boolean return from validate_expr (true allows, false blocks)
//! SCRIPT-04: Inline shell scripts execute with event JSON on stdin
//! SCRIPT-05: Timeout protection for long-running scripts (fail-closed)
//! SCRIPT-06: Config validation rejects invalid expressions and mutual exclusion

#![allow(deprecated)] // cargo_bin deprecation

use assert_cmd::Command;
use std::fs;

#[path = "common/mod.rs"]
mod common;
use common::{TestEvidence, Timer, evidence_dir};

// =============================================================================
// SCRIPT-01: validate_expr in YAML
// =============================================================================

#[test]
fn test_e2e_validate_expr_passes_allows() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_validate_expr_passes", "OQ-SCRIPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: validate-file-path
    matchers:
      tools: [Write]
    actions:
      validate_expr: 'has_field("file_path")'
      inject_inline: "File path validated"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event WITH file_path - should pass validation and inject
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Write",
        "tool_input": {"file_path": "/test/file.txt", "content": "hello"},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    assert!(
        output.status.success(),
        "Should allow when validation passes. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(r#""continue":true"#) || stdout.contains(r#""continue": true"#),
        "Should continue: {}",
        stdout
    );
    assert!(
        stdout.contains("File path validated"),
        "Should inject context: {}",
        stdout
    );

    evidence.pass(
        "validate_expr with has_field passes and allows operation",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

#[test]
fn test_e2e_validate_expr_fails_blocks() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_validate_expr_fails", "OQ-SCRIPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: validate-missing
    matchers:
      tools: [Write]
    actions:
      validate_expr: 'has_field("required_field")'
      inject_inline: "Should not appear"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event WITHOUT required_field - should fail validation and block
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Write",
        "tool_input": {"file_path": "/test/file.txt"},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    // Validation failure exits with code 2 (validation error) and writes to stderr
    assert!(
        !output.status.success(),
        "Should exit non-zero when validation fails"
    );
    let exit_code = output.status.code().unwrap_or(-1);
    assert_eq!(
        exit_code, 2,
        "Should exit with code 2 (validation error), got {}",
        exit_code
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Validation failed") || stderr.contains("validate-missing"),
        "Should report validation failure: {}",
        stderr
    );

    evidence.pass(
        "validate_expr returning false blocks operation",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

#[test]
fn test_e2e_validate_expr_with_get_field() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_validate_expr_get_field", "OQ-SCRIPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: validate-language
    matchers:
      tools: [Edit]
    actions:
      validate_expr: 'get_field("language") == "rust"'
      inject_inline: "Rust file validated"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event with language=rust - should pass
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Edit",
        "tool_input": {"file_path": "/test/file.rs", "language": "rust"},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    assert!(
        output.status.success(),
        "Should allow when get_field comparison passes. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(r#""continue":true"#) || stdout.contains(r#""continue": true"#),
        "Should continue: {}",
        stdout
    );
    assert!(
        stdout.contains("Rust file validated"),
        "Should inject context: {}",
        stdout
    );

    evidence.pass(
        "validate_expr with get_field comparison works correctly",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// =============================================================================
// SCRIPT-02: Custom Functions
// =============================================================================

#[test]
fn test_e2e_get_field_nested() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_get_field_nested", "OQ-SCRIPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: validate-nested-field
    matchers:
      tools: [API]
    actions:
      validate_expr: 'get_field("user.name") != ""'
      inject_inline: "User name present"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event with nested field
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "API",
        "tool_input": {"user": {"name": "Alice", "id": 123}},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    assert!(
        output.status.success(),
        "Should allow when nested field present. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(r#""continue":true"#) || stdout.contains(r#""continue": true"#),
        "Should continue: {}",
        stdout
    );

    evidence.pass(
        "get_field works with nested field paths (dot notation)",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

#[test]
fn test_e2e_has_field_with_null() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_has_field_null", "OQ-SCRIPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: validate-not-null
    matchers:
      tools: [API]
    actions:
      validate_expr: 'has_field("value")'
      inject_inline: "Value is present"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event with null value - has_field should return false
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "API",
        "tool_input": {"value": null},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    // Validation failure exits with code 2
    assert!(
        !output.status.success(),
        "Should exit non-zero when validation fails"
    );
    let exit_code = output.status.code().unwrap_or(-1);
    assert_eq!(exit_code, 2, "Should exit with code 2, got {}", exit_code);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Validation failed") || stderr.contains("validate-not-null"),
        "Should report validation failure: {}",
        stderr
    );

    evidence.pass(
        "has_field returns false for null values (null = missing)",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// =============================================================================
// SCRIPT-03: Boolean Semantics
// =============================================================================

#[test]
fn test_e2e_validate_expr_complex_boolean() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_validate_expr_complex_boolean", "OQ-SCRIPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: validate-multiple
    matchers:
      tools: [API]
    actions:
      validate_expr: 'has_field("name") && has_field("email")'
      inject_inline: "Both fields present"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event with both fields
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "API",
        "tool_input": {"name": "Alice", "email": "alice@example.com"},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    assert!(
        output.status.success(),
        "Should allow when both fields present. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(r#""continue":true"#) || stdout.contains(r#""continue": true"#),
        "Should continue: {}",
        stdout
    );

    evidence.pass(
        "Complex boolean expressions with && work correctly",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

#[test]
fn test_e2e_validate_expr_false_expression() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_validate_expr_always_false", "OQ-SCRIPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: always-block
    matchers:
      tools: [Bash]
    actions:
      validate_expr: '1 == 2'
      inject_inline: "Should never appear"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {"command": "ls"},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    // Validation failure exits with code 2
    assert!(
        !output.status.success(),
        "Should exit non-zero when validation fails"
    );
    let exit_code = output.status.code().unwrap_or(-1);
    assert_eq!(exit_code, 2, "Should exit with code 2, got {}", exit_code);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Validation failed") || stderr.contains("always-block"),
        "Should report validation failure: {}",
        stderr
    );

    evidence.pass(
        "Expression always returning false blocks operation",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// =============================================================================
// SCRIPT-04: Inline Shell Scripts
// =============================================================================

#[test]
#[cfg(unix)] // Inline scripts use #!/bin/bash shebangs — not available on Windows
fn test_e2e_inline_script_exit_zero_allows() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_inline_script_exit_zero", "OQ-SCRIPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: script-allow
    matchers:
      tools: [Bash]
    actions:
      inline_script: |
        #!/bin/bash
        exit 0
      inject_inline: "Script passed"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {"command": "echo hello"},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    assert!(
        output.status.success(),
        "Should allow when script exits 0. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(r#""continue":true"#) || stdout.contains(r#""continue": true"#),
        "Should continue: {}",
        stdout
    );
    assert!(
        stdout.contains("Script passed"),
        "Should inject context: {}",
        stdout
    );

    evidence.pass(
        "inline_script with exit 0 allows operation",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

#[test]
#[cfg(unix)] // Inline scripts use #!/bin/bash shebangs — not available on Windows
fn test_e2e_inline_script_exit_nonzero_blocks() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_inline_script_exit_nonzero", "OQ-SCRIPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: script-block
    matchers:
      tools: [Bash]
    actions:
      inline_script: |
        #!/bin/bash
        exit 1
      inject_inline: "Should not appear"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {"command": "rm -rf /"},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    // Script failure exits with code 2
    assert!(
        !output.status.success(),
        "Should exit non-zero when script fails"
    );
    let exit_code = output.status.code().unwrap_or(-1);
    assert_eq!(exit_code, 2, "Should exit with code 2, got {}", exit_code);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Inline script validation failed") || stderr.contains("script-block"),
        "Should report script failure: {}",
        stderr
    );

    evidence.pass(
        "inline_script with exit 1 blocks operation",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

#[test]
#[cfg(unix)] // Inline scripts use #!/bin/bash shebangs — not available on Windows
fn test_e2e_inline_script_reads_stdin() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_inline_script_stdin", "OQ-SCRIPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Script that reads stdin and checks for a field
    let config = r#"
version: "1.0"
rules:
  - name: script-read-stdin
    matchers:
      tools: [Write]
    actions:
      inline_script: |
        #!/bin/bash
        # Read JSON from stdin
        INPUT=$(cat)
        # Check if file_path exists in tool_input (basic string check)
        if echo "$INPUT" | grep -q '"file_path"'; then
          exit 0
        else
          exit 1
        fi
      inject_inline: "File path detected in stdin"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Write",
        "tool_input": {"file_path": "/test/file.txt", "content": "hello"},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    assert!(
        output.status.success(),
        "Should succeed when script finds field. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(r#""continue":true"#) || stdout.contains(r#""continue": true"#),
        "Should continue when script succeeds: {}",
        stdout
    );

    evidence.pass(
        "inline_script receives event JSON on stdin",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// =============================================================================
// SCRIPT-05: Timeout Protection
// =============================================================================

#[test]
#[cfg(unix)] // Inline scripts use #!/bin/bash shebangs — not available on Windows
fn test_e2e_inline_script_timeout_blocks() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_inline_script_timeout", "OQ-SCRIPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Script that sleeps longer than timeout
    let config = r#"
version: "1.0"
rules:
  - name: script-timeout
    matchers:
      tools: [Bash]
    actions:
      inline_script: |
        #!/bin/bash
        sleep 30
        exit 0
    metadata:
      timeout: 1
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {"command": "echo test"},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    // Timeout exits with code 2 (fail-closed)
    assert!(!output.status.success(), "Should exit non-zero on timeout");
    let exit_code = output.status.code().unwrap_or(-1);
    assert_eq!(exit_code, 2, "Should exit with code 2, got {}", exit_code);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Inline script validation failed") || stderr.contains("script-timeout"),
        "Should report script timeout: {}",
        stderr
    );

    let duration = timer.elapsed_ms();
    // Should timeout quickly (within ~1.5 seconds, not 30 seconds)
    assert!(
        duration < 5000,
        "Should timeout quickly, got {}ms",
        duration
    );

    evidence.pass(
        "inline_script timeout protection works (fail-closed)",
        duration,
    );
    let _ = evidence.save(&evidence_dir());
}

// =============================================================================
// SCRIPT-06: Config Validation
// =============================================================================

#[test]
fn test_e2e_invalid_validate_expr_rejected() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_invalid_validate_expr", "OQ-SCRIPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Invalid expression with unclosed parenthesis
    let config = r#"
version: "1.0"
rules:
  - name: invalid-expr
    matchers:
      tools: [Write]
    actions:
      validate_expr: '((('
      inject_inline: "Should not load"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Write",
        "tool_input": {"file_path": "/test/file.txt"},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    // Should fail at config load time
    assert!(
        !output.status.success(),
        "Should fail when config has invalid validate_expr"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Invalid validate_expr")
            || stderr.contains("invalid-expr")
            || stderr.contains("Failed to parse"),
        "Should mention validation error: {}",
        stderr
    );

    evidence.pass(
        "Invalid validate_expr rejected at config load",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

#[test]
fn test_e2e_mutual_exclusion_rejected() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_mutual_exclusion", "OQ-SCRIPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Both validate_expr and inline_script present
    let config = r#"
version: "1.0"
rules:
  - name: both-present
    matchers:
      tools: [Write]
    actions:
      validate_expr: 'has_field("file_path")'
      inline_script: |
        #!/bin/bash
        exit 0
      inject_inline: "Should not load"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Write",
        "tool_input": {"file_path": "/test/file.txt"},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    // Should fail at config load time
    assert!(
        !output.status.success(),
        "Should fail when both validate_expr and inline_script present"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("mutually exclusive") || stderr.contains("both-present"),
        "Should mention mutual exclusion: {}",
        stderr
    );

    evidence.pass(
        "Mutual exclusion between validate_expr and inline_script enforced",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// =============================================================================
// Combined Tests
// =============================================================================

#[test]
fn test_e2e_validate_expr_with_tool_matcher() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_validate_expr_with_matcher", "OQ-SCRIPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: combined-matchers
    matchers:
      tools: [Write]
    actions:
      validate_expr: 'has_field("file_path")'
      inject_inline: "Matched and validated"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Test 1: Wrong tool - rule should not match at all
    let event_wrong_tool = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Read",
        "tool_input": {"file_path": "/test/file.txt"},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_wrong_tool)
        .output()
        .expect("command should run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(r#""continue":true"#) || stdout.contains(r#""continue": true"#),
        "Should allow when tool doesn't match: {}",
        stdout
    );
    assert!(
        !stdout.contains("Matched and validated"),
        "Should not inject when tool doesn't match: {}",
        stdout
    );

    // Test 2: Correct tool but missing field - should block
    let event_missing_field = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Write",
        "tool_input": {"content": "hello"},
        "session_id": "test-session"
    }"#;

    let output2 = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_missing_field)
        .output()
        .expect("command should run");

    // Validation failure exits with code 2
    assert!(
        !output2.status.success(),
        "Should exit non-zero when validation fails"
    );
    let exit_code2 = output2.status.code().unwrap_or(-1);
    assert_eq!(exit_code2, 2, "Should exit with code 2, got {}", exit_code2);

    let stderr2 = String::from_utf8_lossy(&output2.stderr);
    assert!(
        stderr2.contains("Validation failed") || stderr2.contains("combined-matchers"),
        "Should report validation failure: {}",
        stderr2
    );

    evidence.pass(
        "validate_expr works correctly with tool matchers",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

#[test]
#[cfg(unix)] // Inline scripts use #!/bin/bash shebangs — not available on Windows
fn test_e2e_inline_script_with_inject_inline() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_inline_script_with_inject", "OQ-SCRIPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: script-then-inject
    matchers:
      tools: [Bash]
    actions:
      inline_script: |
        #!/bin/bash
        exit 0
      inject_inline: "Script validated successfully"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {"command": "echo hello"},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    assert!(
        output.status.success(),
        "Should succeed when script passes. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(r#""continue":true"#) || stdout.contains(r#""continue": true"#),
        "Should continue: {}",
        stdout
    );
    assert!(
        stdout.contains("Script validated successfully"),
        "Should inject context after script passes: {}",
        stdout
    );

    evidence.pass(
        "inline_script passes then inject_inline content appears",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

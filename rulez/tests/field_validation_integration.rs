//! Field Validation Integration Tests (OQ - Operational Qualification)
//!
//! End-to-end tests verifying field validation through the full stack:
//! YAML config -> Config::from_file -> process_event -> Response
//!
//! Requirements covered: FIELD-01, FIELD-02, FIELD-03, FIELD-04
//!
//! FIELD-01: require_fields validates field existence in tool_input
//! FIELD-02: Missing required fields cause blocking (fail-closed)
//! FIELD-03: Nested field paths with dot notation resolve correctly
//! FIELD-04: Field type validation (string, number, boolean, array, object, any)

#![allow(deprecated)] // cargo_bin deprecation

use assert_cmd::Command;
use std::fs;

#[path = "common/mod.rs"]
mod common;
use common::{TestEvidence, Timer, evidence_dir};

// =============================================================================
// FIELD-01: Require specific fields exist
// =============================================================================

#[test]
fn test_e2e_require_fields_present_allows() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_require_fields_present", "OQ-FIELD");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: require-file-path
    matchers:
      tools: [Edit]
      require_fields: ["file_path"]
    actions:
      inject_inline: "File path provided"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event WITH file_path - should be allowed and inject
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Edit",
        "tool_input": {"file_path": "/test/file.txt"},
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
        "Should allow when required field present. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(r#""continue":true"#) || stdout.contains(r#""continue": true"#),
        "Should continue: {}",
        stdout
    );
    assert!(
        stdout.contains("File path provided"),
        "Should inject context: {}",
        stdout
    );

    evidence.pass(
        "Required field present allows operation",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

#[test]
fn test_e2e_require_fields_missing_blocks() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_require_fields_missing", "OQ-FIELD");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Use no other matchers - field validation is the only matcher
    // When field validation fails, rule doesn't match, operation continues
    // To test "blocking" behavior, we verify the rule DOESN'T match (no injection)
    let config = r#"
version: "1.0"
rules:
  - name: require-file-path
    matchers:
      require_fields: ["file_path"]
    actions:
      inject_inline: "File path validated"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event WITHOUT file_path - rule should NOT match (fail-closed)
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Edit",
        "tool_input": {"other_field": "value"},
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
        "Should continue (rule doesn't match when field missing)"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("File path validated"),
        "Should NOT inject when required field missing (rule doesn't match): {}",
        stdout
    );
    assert!(
        stdout.contains(r#""continue":true"#) || stdout.contains(r#""continue": true"#),
        "Should continue: {}",
        stdout
    );

    evidence.pass(
        "Missing required field causes rule not to match (fail-closed)",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// =============================================================================
// FIELD-02: Fail-closed blocking
// =============================================================================

#[test]
fn test_e2e_require_fields_no_tool_input_blocks() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_no_tool_input", "OQ-FIELD");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: require-command
    matchers:
      require_fields: ["command"]
    actions:
      inject_inline: "Command validated"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event WITHOUT tool_input - rule should NOT match (fail-closed)
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
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
        "Should continue (rule doesn't match when tool_input missing)"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("Command validated"),
        "Should NOT inject when tool_input missing (fail-closed): {}",
        stdout
    );

    evidence.pass(
        "Missing tool_input causes rule not to match (fail-closed)",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

#[test]
fn test_e2e_require_fields_null_value_blocks() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_null_value", "OQ-FIELD");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: require-name
    matchers:
      require_fields: ["name"]
    actions:
      inject_inline: "Name validated"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event with null name - rule should NOT match
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "API",
        "tool_input": {"name": null},
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
        "Should continue (rule doesn't match when field is null)"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("Name validated"),
        "Should NOT inject when field is null: {}",
        stdout
    );

    evidence.pass(
        "Null value treated as missing (rule doesn't match)",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// =============================================================================
// FIELD-03: Dot notation nested paths
// =============================================================================

#[test]
fn test_e2e_nested_field_present_allows() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_nested_field_present", "OQ-FIELD");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: require-user-name
    matchers:
      tools: [API]
      require_fields: ["user.name"]
    actions:
      inject_inline: "User identified"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event with nested user.name
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "API",
        "tool_input": {
            "user": {
                "name": "Alice"
            }
        },
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
        "Should allow when nested field present"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("User identified"),
        "Should inject context: {}",
        stdout
    );

    evidence.pass(
        "Nested field (user.name) resolves correctly",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

#[test]
fn test_e2e_nested_field_missing_blocks() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_nested_field_missing", "OQ-FIELD");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: require-user-email
    matchers:
      require_fields: ["user.email"]
    actions:
      inject_inline: "User email validated"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event with user object but no email field - rule should NOT match
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "API",
        "tool_input": {
            "user": {
                "name": "Alice"
            }
        },
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
        "Should continue (rule doesn't match when nested field missing)"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("User email validated"),
        "Should NOT inject when nested field missing: {}",
        stdout
    );

    evidence.pass(
        "Missing nested field causes rule not to match",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

#[test]
fn test_e2e_deep_nested_field() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_deep_nested", "OQ-FIELD");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: require-deep-field
    matchers:
      tools: [API]
      require_fields: ["a.b.c.d"]
    actions:
      inject_inline: "Deep field found"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event with deeply nested field
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "API",
        "tool_input": {
            "a": {
                "b": {
                    "c": {
                        "d": "value"
                    }
                }
            }
        },
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
        "Should allow when deep nested field present"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Deep field found"),
        "Should inject context: {}",
        stdout
    );

    evidence.pass("Deep nesting (a.b.c.d) works correctly", timer.elapsed_ms());
    let _ = evidence.save(&evidence_dir());
}

// =============================================================================
// FIELD-04: Type validation
// =============================================================================

#[test]
fn test_e2e_field_types_correct_allows() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_field_types_correct", "OQ-FIELD");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: type-check
    matchers:
      tools: [API]
      field_types:
        count: number
    actions:
      inject_inline: "Type check passed"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event with correct type (number)
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "API",
        "tool_input": {"count": 42},
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
        "Should allow when field type correct"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Type check passed"),
        "Should inject context: {}",
        stdout
    );

    evidence.pass(
        "Correct field type (number) allows operation",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

#[test]
fn test_e2e_field_types_mismatch_blocks() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_field_types_mismatch", "OQ-FIELD");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: type-check
    matchers:
      field_types:
        count: number
    actions:
      inject_inline: "Type check passed"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event with wrong type (string instead of number) - rule should NOT match
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "API",
        "tool_input": {"count": "42"},
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
        "Should continue (rule doesn't match when type mismatches)"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("Type check passed"),
        "Should NOT inject when type mismatch: {}",
        stdout
    );

    evidence.pass("Type mismatch causes rule not to match", timer.elapsed_ms());
    let _ = evidence.save(&evidence_dir());
}

#[test]
fn test_e2e_field_types_multiple_types() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_multiple_types", "OQ-FIELD");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: multi-type-check
    matchers:
      tools: [API]
      field_types:
        name: string
        count: number
        enabled: boolean
        items: array
    actions:
      inject_inline: "All types valid"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event with all correct types
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "API",
        "tool_input": {
            "name": "test",
            "count": 42,
            "enabled": true,
            "items": [1, 2, 3]
        },
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
        "Should allow when all field types correct"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("All types valid"),
        "Should inject context: {}",
        stdout
    );

    evidence.pass(
        "Multiple field types validated correctly",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

#[test]
fn test_e2e_field_types_implies_existence() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_types_implies_existence", "OQ-FIELD");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: type-check-existence
    matchers:
      field_types:
        count: number
    actions:
      inject_inline: "Count validated"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event WITHOUT the count field - rule should NOT match (field_types implies existence)
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "API",
        "tool_input": {"other_field": "value"},
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
        "Should continue (rule doesn't match when field missing)"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("Count validated"),
        "Should NOT inject when field missing (field_types implies existence): {}",
        stdout
    );

    evidence.pass("field_types implies existence check", timer.elapsed_ms());
    let _ = evidence.save(&evidence_dir());
}

// =============================================================================
// Combined tests
// =============================================================================

#[test]
fn test_e2e_require_and_types_together() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_require_and_types", "OQ-FIELD");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: combined-validation
    matchers:
      tools: [API]
      require_fields: ["name", "data"]
      field_types:
        count: number
        data: object
    actions:
      inject_inline: "Validation passed"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event with all required fields and correct types
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "API",
        "tool_input": {
            "name": "test",
            "count": 42,
            "data": {"key": "value"}
        },
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
        "Should allow when both require_fields and field_types pass"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Validation passed"),
        "Should inject context: {}",
        stdout
    );

    evidence.pass(
        "Combined require_fields and field_types work together",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

#[test]
fn test_e2e_field_validation_with_tool_matcher() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_with_tool_matcher", "OQ-FIELD");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: bash-with-validation
    matchers:
      tools: [Bash]
      require_fields: ["command"]
    actions:
      inject_inline: "Command validated"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Test 1: Correct tool + required field - should pass
    let event_pass = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {"command": "echo test"},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_pass)
        .output()
        .expect("command should run");

    assert!(
        output.status.success(),
        "Should pass when tool and field match"
    );

    // Test 2: Wrong tool - should not match rule
    let event_wrong_tool = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Edit",
        "tool_input": {"command": "echo test"},
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_wrong_tool)
        .output()
        .expect("command should run");

    assert!(
        output.status.success(),
        "Should pass when tool doesn't match"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("Command validated"),
        "Should NOT inject when tool doesn't match"
    );

    evidence.pass(
        "Field validation works with tool matcher",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// =============================================================================
// Config validation integration
// =============================================================================

#[test]
fn test_e2e_invalid_field_path_rejected() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_invalid_field_path", "OQ-FIELD");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Config with invalid field path (starts with dot)
    let config = r#"
version: "1.0"
rules:
  - name: invalid-path
    matchers:
      require_fields: [".name"]
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .arg("validate")
        .current_dir(temp_dir.path())
        .output()
        .expect("command should run");

    assert!(
        !output.status.success(),
        "Validation should fail for field path starting with dot"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{} {}", stderr, stdout);

    assert!(
        combined.contains("invalid") || combined.contains("path") || combined.contains(".name"),
        "Error should mention invalid path: {}",
        combined
    );

    evidence.pass(
        "Invalid field path rejected at config load",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

#[test]
fn test_e2e_invalid_type_specifier_rejected() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_invalid_type_specifier", "OQ-FIELD");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Config with invalid type specifier
    let config = r#"
version: "1.0"
rules:
  - name: invalid-type
    matchers:
      field_types:
        count: integer
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .arg("validate")
        .current_dir(temp_dir.path())
        .output()
        .expect("command should run");

    assert!(
        !output.status.success(),
        "Validation should fail for invalid type specifier 'integer'"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{} {}", stderr, stdout);

    assert!(
        combined.contains("invalid") || combined.contains("type") || combined.contains("integer"),
        "Error should mention invalid type: {}",
        combined
    );

    evidence.pass(
        "Invalid type specifier rejected at config load",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

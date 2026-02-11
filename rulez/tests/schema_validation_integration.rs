//! Integration tests for JSON Schema validation via CLI.
//!
//! Tests verify:
//! - REQ-SCHEMA-05: Malformed JSON produces exit code 1
//! - Fail-open schema validation (warns but continues)
//! - Fail-closed serde deserialization (missing required fields are fatal)
//! - REQ-PERF-01: Event processing completes within 100ms
//! - REQ-PERF-02: Binary size remains under 5MB

#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::time::Instant;
use tempfile::TempDir;

/// Create a test config directory with minimal valid hooks.yaml
fn create_test_config(temp_dir: &TempDir) {
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let hooks_yaml = r#"
version: "1.0"
rules: []
settings:
  fail_open: true
"#;

    fs::write(claude_dir.join("hooks.yaml"), hooks_yaml).expect("Failed to write hooks.yaml");
}

/// Test that malformed JSON exits with code 1 (REQ-SCHEMA-05)
#[test]
fn test_malformed_json_exits_code_1() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    create_test_config(&temp_dir);

    // Pipe invalid JSON to rulez binary
    let invalid_json = r#"{"invalid json"#;

    Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(invalid_json)
        .assert()
        // Should exit with code 1 (config/data error, not policy block)
        .code(1)
        // Should contain error message about JSON parsing (tracing outputs to stdout)
        .stdout(predicate::str::contains("parse").or(predicate::str::contains("JSON")));
}

/// Test that empty stdin exits with code 1
#[test]
fn test_empty_stdin_exits_code_1() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    create_test_config(&temp_dir);

    Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin("")
        .assert()
        // Should exit with code 1
        .code(1)
        // Should contain error message about no input (tracing outputs to stdout)
        .stdout(predicate::str::contains("No input"));
}

/// Test that valid event processes successfully
#[test]
fn test_valid_event_processes_successfully() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    create_test_config(&temp_dir);

    // Create a valid event JSON
    let event = format!(
        r#"{{"hook_event_name":"PreToolUse","session_id":"test-123","tool_name":"Bash","tool_input":{{"command":"echo hello"}},"cwd":"{}"}}"#,
        temp_dir.path().display()
    );

    Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .assert()
        // Should exit with code 0 (allowed)
        .success()
        // Should contain "continue":true in JSON stdout
        .stdout(predicate::str::contains(r#""continue":true"#));
}

/// Test that missing required fields fails deserialization (fail-closed)
///
/// This test verifies the distinction between fail-open schema validation
/// and fail-closed serde deserialization:
/// - Step 1: JSON parsing succeeds (valid JSON syntax)
/// - Step 2: Schema validation is fail-open (warns about missing fields, continues)
/// - Step 3: Serde deserialization FAILS because hook_event_name and session_id
///   are required by the Event struct and cannot be constructed without them
///
/// The deserialization error propagates via `?` and main() exits with non-zero.
#[test]
fn test_missing_required_fields_fails_deserialization() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    create_test_config(&temp_dir);

    // Valid JSON but missing required serde fields
    let event = r#"{"tool_name":"Bash"}"#;

    Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .assert()
        // Should exit with non-zero code (not 0, not 2)
        // Deserialization errors typically exit with code 1
        .failure()
        // Should contain error message about deserialization (tracing outputs to stdout)
        .stdout(
            predicate::str::contains("deserialize")
                .or(predicate::str::contains("missing field"))
                .or(predicate::str::contains("hook_event_name")),
        );
}

/// Test that events with extra fields are accepted
#[test]
fn test_event_with_extra_fields_accepted() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    create_test_config(&temp_dir);

    // Event with all required fields plus unknown extra fields
    let event = format!(
        r#"{{"hook_event_name":"PreToolUse","session_id":"test-123","extra_unknown_field":"should be fine","cwd":"{}"}}"#,
        temp_dir.path().display()
    );

    Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .assert()
        // Should exit with code 0 (extra fields don't cause errors)
        .success();
}

/// Test that binary size is under 5MB (REQ-PERF-02)
///
/// This test is marked #[ignore] because it requires a release build.
/// Run with: cargo test --release -- --ignored test_binary_size_under_5mb
#[test]
#[ignore]
fn test_binary_size_under_5mb() {
    let binary_path = std::env::current_exe()
        .expect("Failed to get current exe path")
        .parent()
        .expect("Failed to get parent dir")
        .parent()
        .expect("Failed to get parent dir")
        .join("release")
        .join("rulez");

    if !binary_path.exists() {
        panic!(
            "Release binary not found at {}. Run: cargo build --release",
            binary_path.display()
        );
    }

    let metadata = fs::metadata(&binary_path).expect("Failed to get binary metadata");
    let size_bytes = metadata.len();
    let size_mb = size_bytes as f64 / (1024.0 * 1024.0);

    assert!(
        size_bytes < 5 * 1024 * 1024,
        "Binary size {:.2} MB exceeds 5MB limit (REQ-PERF-02)",
        size_mb
    );
}

/// Test that events with wrong types fail deserialization
#[test]
fn test_event_with_wrong_types_fails_deserialization() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    create_test_config(&temp_dir);

    // JSON where hook_event_name is an integer instead of string
    let event = r#"{"hook_event_name":42,"session_id":"test-123"}"#;

    Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .assert()
        // Should exit with non-zero code (deserialization error)
        .failure();
}

/// Test that event processing completes within 2 seconds (REQ-PERF-01)
///
/// This is a conservative wall-clock timing test that includes process spawn
/// overhead. The actual event processing latency requirement is <10ms p95 for
/// the event processing logic itself, but this test includes full process spawn,
/// so we allow 2 seconds to catch gross performance regressions without flakiness.
#[test]
fn test_event_processing_completes_within_2_seconds() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    create_test_config(&temp_dir);

    // Create a valid event JSON
    let event = format!(
        r#"{{"hook_event_name":"PreToolUse","session_id":"test-123","tool_name":"Bash","tool_input":{{"command":"echo hello"}},"cwd":"{}"}}"#,
        temp_dir.path().display()
    );

    let start = Instant::now();

    Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .assert()
        // Should succeed
        .success();

    let elapsed = start.elapsed();

    // Should complete within 2 seconds (conservative, includes process spawn overhead)
    assert!(
        elapsed.as_secs() < 2,
        "Event processing took {}ms, expected <2s (REQ-PERF-01 regression test)",
        elapsed.as_millis()
    );
}

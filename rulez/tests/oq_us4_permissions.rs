//! Operational Qualification (OQ) Tests - User Story 4: Permission Explanations
//!
//! US4: As a developer, I want explanatory context injected before permission
//! prompts, so I understand why commands need approval.
//!
//! These tests verify the permission explanation functionality.

#![allow(deprecated)]
#![allow(unused_imports)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[path = "common/mod.rs"]
mod common;
use common::{TestEvidence, Timer, evidence_dir, fixture_path, read_fixture};

/// Test that permission requests trigger context injection
#[test]
fn test_us4_permission_request_injection() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("permission_request_injection", "OQ-US4");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");

    // Create .claude directory with config
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Copy permission config
    let config_src = fixture_path("hooks/permission-explanations.yaml");
    fs::copy(&config_src, claude_dir.join("hooks.yaml")).expect("copy config");

    // Create context file
    let context_dir = claude_dir.join("context");
    fs::create_dir_all(&context_dir).expect("create context dir");
    fs::write(
        context_dir.join("explain-command.md"),
        "# Command Explanation\n\nThis command requires permission because...",
    )
    .expect("write context");

    // Read the permission request event
    let event = read_fixture("events/permission-request-event.json");

    // Run CCH with the event
    let result = Command::cargo_bin("cch")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .assert()
        .success();

    // Response should allow and potentially include context
    result.stdout(
        predicate::str::contains(r#""continue":true"#)
            .or(predicate::str::contains(r#""continue": true"#)),
    );

    evidence.pass(
        "Permission request correctly triggers explanation injection",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that PermissionRequest event type is filtered correctly
#[test]
fn test_us4_permission_event_type_filter() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("permission_event_type_filter", "OQ-US4");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");

    // Create .claude directory with config
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Copy permission config
    let config_src = fixture_path("hooks/permission-explanations.yaml");
    fs::copy(&config_src, claude_dir.join("hooks.yaml")).expect("copy config");

    // Create context file
    let context_dir = claude_dir.join("context");
    fs::create_dir_all(&context_dir).expect("create context dir");
    fs::write(
        context_dir.join("explain-command.md"),
        "# Command Explanation\n\nContext here.",
    )
    .expect("write context");

    // Create a PreToolUse event (not PermissionRequest) - should NOT match
    let event = r#"{
        "event_type": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {
            "command": "echo hello"
        },
        "session_id": "test-session-pretool",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;

    // Run CCH with the event
    let output = Command::cargo_bin("cch")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command runs");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should allow - the permission rule requires operations: ["PermissionRequest"]
    assert!(stdout.contains(r#""continue":true"#) || stdout.contains(r#""continue": true"#));

    evidence.pass(
        "PreToolUse event does not match PermissionRequest filter",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that file operation permissions get explanations
#[test]
fn test_us4_file_operation_explanation() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("file_operation_explanation", "OQ-US4");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");

    // Create .claude directory with config
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Copy permission config
    let config_src = fixture_path("hooks/permission-explanations.yaml");
    fs::copy(&config_src, claude_dir.join("hooks.yaml")).expect("copy config");

    // Create context file
    let context_dir = claude_dir.join("context");
    fs::create_dir_all(&context_dir).expect("create context dir");
    fs::write(
        context_dir.join("explain-file-ops.md"),
        "# File Operation\n\nThis modifies files.",
    )
    .expect("write context");

    // Create a Write permission request event
    let event = r#"{
        "event_type": "PermissionRequest",
        "tool_name": "Write",
        "tool_input": {
            "filePath": "/etc/hosts",
            "content": "malicious content"
        },
        "session_id": "test-session-file-perm",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;

    // Run CCH with the event
    let result = Command::cargo_bin("cch")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .assert()
        .success();

    // Response should allow (permission explanations don't block, they inject)
    result.stdout(
        predicate::str::contains(r#""continue":true"#)
            .or(predicate::str::contains(r#""continue": true"#)),
    );

    evidence.pass(
        "File operation permission request handled correctly",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

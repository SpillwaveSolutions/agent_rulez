//! Operational Qualification (OQ) Tests - User Story 1: Block Dangerous Operations
//!
//! US1: As a developer, I want to automatically block dangerous git operations
//! like force push, so that I don't accidentally overwrite remote history.
//!
//! These tests verify the blocking functionality works correctly.
//!
//! Claude Code hooks protocol for blocking:
//! - Exit code 0 = allow (JSON stdout parsed for context injection)
//! - Exit code 2 = BLOCK the tool call (stderr = reason fed to Claude)
//! - Other exit codes = non-blocking error

#![allow(deprecated)]
#![allow(unused_imports)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[path = "common/mod.rs"]
mod common;
use common::{TestEvidence, Timer, evidence_dir, fixture_path, read_fixture, setup_test_env};

/// Test that force push is blocked when configured
#[test]
fn test_us1_force_push_blocked() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("force_push_blocked", "OQ-US1");

    // Setup test environment with blocking config
    let temp_dir = setup_test_env("block-force-push.yaml");

    // Read the force push event
    let event = read_fixture("events/force-push-event.json");

    // Run CCH with the event
    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    // Claude Code protocol: exit code 2 = BLOCK the tool
    assert_eq!(
        output.status.code(),
        Some(2),
        "Blocked commands MUST exit with code 2 (Claude Code blocking protocol)"
    );

    // stderr contains the blocking reason (fed to Claude)
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("block-force-push") || stderr.contains("Blocked"),
        "stderr should contain the rule name or blocking message, got: {stderr}"
    );

    evidence.pass(
        &format!(
            "Force push event correctly blocked with exit code 2, stderr: {}",
            stderr.trim()
        ),
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that safe push is allowed when force push is blocked
#[test]
fn test_us1_safe_push_allowed() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("safe_push_allowed", "OQ-US1");

    // Setup test environment with blocking config
    let temp_dir = setup_test_env("block-force-push.yaml");

    // Read the safe push event
    let event = read_fixture("events/safe-push-event.json");

    // Run CCH with the event
    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .assert()
        .success();

    // Response should allow the operation
    result.stdout(
        predicate::str::contains(r#""continue":true"#)
            .or(predicate::str::contains(r#""continue": true"#)),
    );

    evidence.pass("Safe push event correctly allowed", timer.elapsed_ms());
    let _ = evidence.save(&evidence_dir());
}

/// Test that hard reset is blocked when configured
#[test]
fn test_us1_hard_reset_blocked() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("hard_reset_blocked", "OQ-US1");

    // Setup test environment with blocking config
    let temp_dir = setup_test_env("block-force-push.yaml");

    // Create hard reset event
    let event = r#"{
        "event_type": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {
            "command": "git reset --hard HEAD~5"
        },
        "session_id": "test-session-reset",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;

    // Run CCH with the event
    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    // Claude Code protocol: exit code 2 = BLOCK the tool
    assert_eq!(
        output.status.code(),
        Some(2),
        "Hard reset MUST exit with code 2 (blocked)"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("block-hard-reset") || stderr.contains("Blocked"),
        "stderr should contain rule name or blocking message, got: {stderr}"
    );

    evidence.pass(
        &format!(
            "Hard reset correctly blocked with exit code 2, stderr: {}",
            stderr.trim()
        ),
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that blocking provides a clear reason
#[test]
fn test_us1_block_reason_provided() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("block_reason_provided", "OQ-US1");

    // Setup test environment with blocking config
    let temp_dir = setup_test_env("block-force-push.yaml");

    // Read the force push event
    let event = read_fixture("events/force-push-event.json");

    // Run CCH with the event
    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    // Claude Code protocol: exit code 2 = BLOCK the tool
    assert_eq!(
        output.status.code(),
        Some(2),
        "Blocked commands MUST exit with code 2"
    );

    // Blocking reason is on stderr (fed to Claude)
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Blocked"),
        "stderr should mention blocking, got: {stderr}"
    );

    evidence.pass(
        &format!(
            "Block response includes clear reason on stderr: {}",
            stderr.trim()
        ),
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

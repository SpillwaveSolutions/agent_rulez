//! Operational Qualification (OQ) Tests - User Story 5: Log Querying
//!
//! US5: As a developer, I want to query CCH logs to understand why rules
//! did or didn't fire, for troubleshooting.
//!
//! These tests verify the logging and querying functionality.

#![allow(deprecated)]
#![allow(unused_imports)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[path = "common/mod.rs"]
mod common;
use common::{TestEvidence, Timer, evidence_dir, fixture_path};

/// Test that logs subcommand works
#[test]
fn test_us5_logs_command_works() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("logs_command_works", "OQ-US5");

    // Run logs command
    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .args(["logs", "--limit", "5"])
        .assert()
        .success();

    // Should output something (even if "No log entries found")
    result.stdout(
        predicate::str::is_empty()
            .not()
            .or(predicate::str::contains("No log entries")),
    );

    evidence.pass("Logs command executes successfully", timer.elapsed_ms());
    let _ = evidence.save(&evidence_dir());
}

/// Test that explain subcommand works
#[test]
fn test_us5_explain_command_works() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("explain_command_works", "OQ-US5");

    // Run explain command with a fake session ID
    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .args(["explain", "test-session-123"])
        .assert()
        .success();

    // Should output something (even if "No log entries found")
    result.stdout(
        predicate::str::is_empty()
            .not()
            .or(predicate::str::contains("No log entries")),
    );

    evidence.pass("Explain command executes successfully", timer.elapsed_ms());
    let _ = evidence.save(&evidence_dir());
}

/// Test that validate subcommand creates default config
#[test]
fn test_us5_validate_creates_default() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("validate_creates_default", "OQ-US5");

    // Create temp directory without config
    let temp_dir = tempfile::tempdir().expect("create temp dir");

    // Run validate command
    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .args(["validate"])
        .assert()
        .success();

    // Should indicate creating default config
    result.stdout(
        predicate::str::contains("Creating default")
            .or(predicate::str::contains("Created default")),
    );

    // Config file should now exist
    assert!(temp_dir.path().join(".claude/hooks.yaml").exists());

    evidence.pass(
        "Validate command creates default configuration when none exists",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that validate subcommand validates existing config
#[test]
fn test_us5_validate_existing_config() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("validate_existing_config", "OQ-US5");

    // Create temp directory with valid config
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Copy valid config
    let config_src = fixture_path("hooks/block-force-push.yaml");
    fs::copy(&config_src, claude_dir.join("hooks.yaml")).expect("copy config");

    // Run validate command
    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .args(["validate"])
        .assert()
        .success();

    // Should indicate valid configuration
    result.stdout(predicate::str::contains("valid").or(predicate::str::contains("Rules loaded")));

    evidence.pass(
        "Validate command correctly validates existing configuration",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that validate catches invalid config
#[test]
fn test_us5_validate_invalid_config() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("validate_invalid_config", "OQ-US5");

    // Create temp directory with invalid config
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Write invalid config (bad YAML)
    fs::write(
        claude_dir.join("hooks.yaml"),
        "version: [invalid\nrules:\n  - bad yaml here",
    )
    .expect("write invalid config");

    // Run validate command - should fail
    Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .args(["validate"])
        .assert()
        .failure();

    evidence.pass(
        "Validate command correctly rejects invalid configuration",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that logs show processing time
#[test]
fn test_us5_logs_show_timing() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("logs_show_timing", "OQ-US5");

    // Setup test environment and process an event to create log
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Copy config
    let config_src = fixture_path("hooks/block-force-push.yaml");
    fs::copy(&config_src, claude_dir.join("hooks.yaml")).expect("copy config");

    // Process an event to generate logs
    let event = r#"{
        "event_type": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {"command": "echo test"},
        "session_id": "timing-test",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;

    // Run event processing
    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command runs");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Response should include timing information
    assert!(stdout.contains("timing") || stdout.contains("processing_ms"));

    evidence.pass(
        "Event processing includes timing information",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

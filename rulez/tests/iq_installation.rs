//! Installation Qualification (IQ) Tests
//!
//! These tests verify that the CCH binary is correctly built and installed.
//! IQ tests check:
//! - Binary compiles and runs
//! - --version returns expected version
//! - --help displays usage information
//! - Binary exits cleanly with no input

#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;

#[path = "common/mod.rs"]
mod common;
use common::{TestEvidence, Timer, evidence_dir};

/// Test that the binary compiles and can be executed
#[test]
fn test_binary_exists_and_runs() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("binary_exists_and_runs", "IQ");

    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .arg("--help")
        .assert()
        .success();

    evidence.pass(
        "Binary compiled and executed --help successfully",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());

    result.stdout(predicate::str::contains("RuleZ"));
}

/// Test that --version returns the correct version
#[test]
fn test_version_output() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("version_output", "IQ");

    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .arg("--version")
        .assert()
        .success();

    // Version should contain "rulez" and a semver pattern
    result.stdout(
        predicate::str::contains("rulez").and(predicate::str::is_match(r"\d+\.\d+\.\d+").unwrap()),
    );

    evidence.pass(
        "Version output contains expected format (rulez x.y.z)",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that --help displays comprehensive usage information
#[test]
fn test_help_output() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("help_output", "IQ");

    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .arg("--help")
        .assert()
        .success();

    // Help should contain subcommands
    result.stdout(
        predicate::str::contains("validate")
            .and(predicate::str::contains("logs"))
            .and(predicate::str::contains("explain")),
    );

    evidence.pass(
        "Help output contains all expected subcommands (validate, logs, explain)",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that validate subcommand has help
#[test]
fn test_validate_help() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("validate_help", "IQ");

    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .args(["validate", "--help"])
        .assert()
        .success();

    result.stdout(predicate::str::contains("config"));

    evidence.pass(
        "Validate subcommand help displays correctly",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that logs subcommand has help
#[test]
fn test_logs_help() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("logs_help", "IQ");

    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .args(["logs", "--help"])
        .assert()
        .success();

    result.stdout(predicate::str::contains("limit").and(predicate::str::contains("since")));

    evidence.pass(
        "Logs subcommand help displays --limit and --since options",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that explain subcommand has help
#[test]
fn test_explain_help() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("explain_help", "IQ");

    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .args(["explain", "--help"])
        .assert()
        .success();

    result.stdout(predicate::str::contains("event"));

    evidence.pass(
        "Explain subcommand help displays correctly",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that binary handles empty stdin gracefully
#[test]
fn test_empty_stdin_error() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("empty_stdin_error", "IQ");

    // When no subcommand is provided and stdin is empty, should exit with error
    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .write_stdin("")
        .assert()
        .failure();

    // Error message may be in stdout (via tracing) or stderr
    result.stdout(predicate::str::contains("No input received"));

    evidence.pass(
        "Binary correctly reports error on empty stdin",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

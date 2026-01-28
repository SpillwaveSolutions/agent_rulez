//! Operational Qualification (OQ) Tests - User Story 3: Custom Validators
//!
//! US3: As a developer, I want to run custom Python scripts that validate
//! code before it's written, so I can enforce complex rules.
//!
//! These tests verify the custom validator functionality.

#![allow(deprecated)]
#![allow(unused_imports)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[path = "common/mod.rs"]
mod common;
use common::{TestEvidence, Timer, evidence_dir, fixture_path, read_fixture};

/// Test that validator blocks code with console.log
#[test]
fn test_us3_validator_blocks_console_log() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("validator_blocks_console_log", "OQ-US3");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");

    // Create .claude directory with config
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Copy validator config
    let config_src = fixture_path("hooks/validate-no-console.yaml");
    fs::copy(&config_src, claude_dir.join("hooks.yaml")).expect("copy config");

    // Create validators directory and copy the validator script
    let validators_dir = claude_dir.join("validators");
    fs::create_dir_all(&validators_dir).expect("create validators");

    let validator_src = fixture_path("validators/no-console-log.py");
    let validator_dst = validators_dir.join("no-console-log.py");
    fs::copy(&validator_src, &validator_dst).expect("copy validator");

    // Make validator executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&validator_dst).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&validator_dst, perms).unwrap();
    }

    // Read the console.log event
    let event = read_fixture("events/console-log-write-event.json");

    // Run CCH with the event
    let output = Command::cargo_bin("cch")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    // Claude Code protocol: exit code 2 = BLOCK the tool
    assert_eq!(
        output.status.code(),
        Some(2),
        "Validator block MUST exit with code 2"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.is_empty(),
        "Blocked response must have stderr reason"
    );

    evidence.pass(
        &format!(
            "Validator correctly blocks code containing console.log (exit 2, stderr: {})",
            stderr.trim()
        ),
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that validator allows clean code
#[test]
fn test_us3_validator_allows_clean_code() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("validator_allows_clean_code", "OQ-US3");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");

    // Create .claude directory with config
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Copy validator config
    let config_src = fixture_path("hooks/validate-no-console.yaml");
    fs::copy(&config_src, claude_dir.join("hooks.yaml")).expect("copy config");

    // Create validators directory and copy the validator script
    let validators_dir = claude_dir.join("validators");
    fs::create_dir_all(&validators_dir).expect("create validators");

    let validator_src = fixture_path("validators/no-console-log.py");
    let validator_dst = validators_dir.join("no-console-log.py");
    fs::copy(&validator_src, &validator_dst).expect("copy validator");

    // Make validator executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&validator_dst).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&validator_dst, perms).unwrap();
    }

    // Create event with clean code (no console.log)
    let event = r#"{
        "event_type": "PreToolUse",
        "tool_name": "Write",
        "tool_input": {
            "filePath": "src/utils/helper.ts",
            "content": "export function helper() {\n  return 42;\n}\n"
        },
        "session_id": "test-session-clean",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;

    // Run CCH with the event
    let result = Command::cargo_bin("cch")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .assert()
        .success();

    // Response should allow
    result.stdout(
        predicate::str::contains(r#""continue":true"#)
            .or(predicate::str::contains(r#""continue": true"#)),
    );

    evidence.pass(
        "Validator correctly allows clean code without console.log",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that validator timeout is handled
#[test]
fn test_us3_validator_timeout_handling() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("validator_timeout_handling", "OQ-US3");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");

    // Create .claude directory with config
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Create a config with a slow validator and short timeout
    let config = r#"
version: "1.0"
rules:
  - name: slow-validator
    description: "A validator that times out"
    matchers:
      tools: ["Write"]
      extensions: [".slow"]
    actions:
      run: ".claude/validators/slow-script.py"
    metadata:
      timeout: 1

settings:
  fail_open: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("write config");

    // Create a slow validator script
    let validators_dir = claude_dir.join("validators");
    fs::create_dir_all(&validators_dir).expect("create validators");

    let slow_script = r#"#!/usr/bin/env python3
import time
time.sleep(10)
print("Done")
"#;
    let script_path = validators_dir.join("slow-script.py");
    fs::write(&script_path, slow_script).expect("write script");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms).unwrap();
    }

    // Create event for .slow file
    let event = r#"{
        "event_type": "PreToolUse",
        "tool_name": "Write",
        "tool_input": {
            "filePath": "test.slow",
            "content": "test"
        },
        "session_id": "test-session-timeout",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;

    // Run CCH with the event - should complete due to fail_open
    let result = Command::cargo_bin("cch")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .timeout(std::time::Duration::from_secs(5))
        .assert()
        .success();

    // With fail_open=true, should allow on timeout
    result.stdout(
        predicate::str::contains(r#""continue":true"#)
            .or(predicate::str::contains(r#""continue": true"#)),
    );

    evidence.pass(
        "Validator timeout handled correctly with fail_open",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

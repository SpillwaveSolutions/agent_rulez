//! End-to-End Tests: Symlink Resolution
//!
//! Validates that RuleZ correctly resolves symlinks when loading configuration.
//! These tests are Unix-only because Windows symlinks require elevated privileges.

#![allow(deprecated)]
#![allow(unused_imports)]

use assert_cmd::Command;
use std::fs;

#[path = "common/mod.rs"]
mod common;
use common::{TestEvidence, Timer, canonicalize_path, evidence_dir, setup_test_env};

/// Test 1: RuleZ finds hooks.yaml when cwd is a symlink to the project dir.
#[test]
#[cfg(unix)]
fn test_symlink_cwd_resolution() {
    use std::os::unix::fs::symlink;

    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_symlink_cwd", "E2E");

    let temp_dir = setup_test_env("block-all-push.yaml");

    let symlink_dir = tempfile::tempdir().unwrap();
    let symlink_path = symlink_dir.path().join("link-to-project");
    symlink(temp_dir.path(), &symlink_path).expect("create symlink");

    let cwd = symlink_path.to_string_lossy().to_string();
    let event = serde_json::json!({
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": { "command": "git push" },
        "cwd": cwd,
        "session_id": "symlink-test"
    });

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(&symlink_path)
        .write_stdin(serde_json::to_string(&event).unwrap())
        .output()
        .expect("command should run");

    assert_eq!(
        output.status.code(),
        Some(2),
        "Should block git push even when cwd is a symlink. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    evidence.pass("Symlink cwd resolution works — git push blocked via symlink", timer.elapsed_ms());
    let _ = evidence.save(&evidence_dir());

    drop(temp_dir);
    drop(symlink_dir);
}

/// Test 2: Canonical and symlink paths both resolve to same config.
#[test]
#[cfg(unix)]
fn test_symlink_vs_canonical_consistency() {
    use std::os::unix::fs::symlink;

    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_symlink_vs_canonical", "E2E");

    let temp_dir = setup_test_env("block-all-push.yaml");
    let canonical_path = canonicalize_path(temp_dir.path());

    let symlink_dir = tempfile::tempdir().unwrap();
    let symlink_path = symlink_dir.path().join("link-to-project");
    symlink(temp_dir.path(), &symlink_path).expect("create symlink");

    let event_canonical = serde_json::json!({
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": { "command": "git push" },
        "cwd": canonical_path.to_string_lossy().to_string(),
        "session_id": "canonical-test"
    });

    let output_canonical = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(serde_json::to_string(&event_canonical).unwrap())
        .output()
        .expect("command should run");

    let event_symlink = serde_json::json!({
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": { "command": "git push" },
        "cwd": symlink_path.to_string_lossy().to_string(),
        "session_id": "symlink-test"
    });

    let output_symlink = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(&symlink_path)
        .write_stdin(serde_json::to_string(&event_symlink).unwrap())
        .output()
        .expect("command should run");

    assert_eq!(output_canonical.status.code(), Some(2), "Canonical path should block git push");
    assert_eq!(output_symlink.status.code(), Some(2), "Symlink path should also block git push");

    evidence.pass("Both canonical and symlink paths produce same result (exit code 2)", timer.elapsed_ms());
    let _ = evidence.save(&evidence_dir());

    drop(temp_dir);
    drop(symlink_dir);
}

/// Test 3: Safe command allowed via symlink cwd.
#[test]
#[cfg(unix)]
fn test_symlink_cwd_allows_safe_commands() {
    use std::os::unix::fs::symlink;

    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_symlink_safe_cmd", "E2E");

    let temp_dir = setup_test_env("block-all-push.yaml");

    let symlink_dir = tempfile::tempdir().unwrap();
    let symlink_path = symlink_dir.path().join("link-to-project");
    symlink(temp_dir.path(), &symlink_path).expect("create symlink");

    let cwd = symlink_path.to_string_lossy().to_string();
    let event = serde_json::json!({
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": { "command": "git status" },
        "cwd": cwd,
        "session_id": "symlink-safe-test"
    });

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(&symlink_path)
        .write_stdin(serde_json::to_string(&event).unwrap())
        .output()
        .expect("command should run");

    assert!(output.status.success(), "Safe commands should be allowed via symlink cwd (exit 0)");

    evidence.pass("Safe command allowed via symlink cwd", timer.elapsed_ms());
    let _ = evidence.save(&evidence_dir());

    drop(temp_dir);
    drop(symlink_dir);
}

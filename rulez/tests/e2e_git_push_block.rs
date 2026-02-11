//! End-to-End Tests: Git Push Block via Claude Code Protocol
//!
//! These tests simulate exactly what Claude Code does when invoking CCH:
//! - Sends JSON via stdin with `hook_event_name` (NOT `event_type`)
//! - Includes `cwd` field pointing to the project directory
//! - Does NOT send `timestamp` (CCH defaults to Utc::now())
//! - Includes extra fields: transcript_path, permission_mode, tool_use_id
//!
//! Claude Code hooks protocol for blocking:
//! - Exit code 0 = allow (JSON stdout parsed for context injection)
//! - Exit code 2 = BLOCK the tool call (stderr = reason fed to Claude)
//! - Other exit codes = non-blocking error
//!
//! CCH now exits with code 2 when blocking, writing the reason to stderr.

#![allow(deprecated)]
#![allow(unused_imports)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[path = "common/mod.rs"]
mod common;
use common::{
    CchResponse, TestEvidence, Timer, canonicalize_path, evidence_dir, fixture_path, setup_test_env,
};

/// Helper: create a test environment and return (temp_dir, event_json)
/// The event JSON uses `hook_event_name` and has `cwd` set to the temp dir path.
fn setup_claude_code_event(config_name: &str, command: &str) -> (tempfile::TempDir, String) {
    let temp_dir = setup_test_env(config_name);
    let canonical_path = canonicalize_path(temp_dir.path());
    let cwd = canonical_path.to_string_lossy().to_string();

    let event = serde_json::json!({
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {
            "command": command
        },
        "session_id": "e2e-test-session",
        "cwd": cwd,
        "transcript_path": "/tmp/transcript.jsonl",
        "permission_mode": "default",
        "tool_use_id": "toolu_e2e_test"
    });

    (temp_dir, serde_json::to_string(&event).unwrap())
}

// ==========================================================================
// Test 1: Basic git push block — exit code 2 + stderr reason
// ==========================================================================

/// Simulate Claude Code sending a `git push` event.
/// CCH must exit with code 2 and write the blocking reason to stderr.
/// This is how Claude Code knows to BLOCK the tool call.
#[test]
fn test_e2e_git_push_blocked_exit_code_2() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_git_push_blocked_exit2", "E2E");

    let (temp_dir, event_json) = setup_claude_code_event("block-all-push.yaml", "git push");

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_json)
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
        stderr.contains("block-git-push"),
        "stderr should contain the rule name, got: {stderr}"
    );
    assert!(
        stderr.contains("Blocked"),
        "stderr should mention blocking, got: {stderr}"
    );

    evidence.pass(
        &format!(
            "git push blocked with exit code 2, stderr: {}",
            stderr.trim()
        ),
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// ==========================================================================
// Test 2: CRITICAL - CWD-based config loading with exit code 2
// ==========================================================================

/// CCH invoked from a DIFFERENT directory than the project.
/// The event's `cwd` field points to the project with hooks.yaml.
/// Must still block with exit code 2.
#[test]
fn test_e2e_cwd_based_config_loading_exit_code_2() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_cwd_config_loading_exit2", "E2E");

    let (temp_dir, event_json) = setup_claude_code_event("block-all-push.yaml", "git push");

    // Create a DIFFERENT directory that has NO hooks.yaml
    let wrong_dir = tempfile::tempdir().expect("create wrong dir");

    // Run CCH from the WRONG directory, but with cwd pointing to the project
    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(wrong_dir.path()) // <-- WRONG dir, no hooks.yaml here
        .write_stdin(event_json)
        .output()
        .expect("command should run");

    assert_eq!(
        output.status.code(),
        Some(2),
        "Must block with exit 2 even when CWD differs from project dir"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("block-git-push"),
        "stderr should contain rule name, got: {stderr}"
    );

    // Verify hooks.yaml exists in the project dir
    assert!(
        temp_dir.path().join(".claude/hooks.yaml").exists(),
        "hooks.yaml should exist in the project dir"
    );

    evidence.pass(
        "git push blocked via cwd-based config loading (exit code 2, CWD != project dir)",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// ==========================================================================
// Test 3: Safe commands exit 0 with JSON stdout
// ==========================================================================

/// Git status should NOT be blocked — exit code 0 with JSON stdout.
#[test]
fn test_e2e_git_status_allowed_exit_code_0() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_git_status_allowed_exit0", "E2E");

    let (temp_dir, event_json) = setup_claude_code_event("block-all-push.yaml", "git status");

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_json)
        .output()
        .expect("command should run");

    assert!(
        output.status.success(),
        "Allowed commands MUST exit with code 0"
    );

    let response = CchResponse::from_output(&output).expect("should parse JSON response");
    assert!(
        response.continue_,
        "git status should be allowed (continue should be true)"
    );

    evidence.pass(
        "git status correctly allowed (exit 0, JSON)",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// ==========================================================================
// Test 4: Various git push variants all exit code 2
// ==========================================================================

#[test]
fn test_e2e_git_push_variants_exit_code_2() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_git_push_variants_exit2", "E2E");

    let push_commands = vec![
        "git push",
        "git push origin main",
        "git push -u origin feature-branch",
        "git push --force origin main",
        "git push -f origin main",
        "git push --force-with-lease origin main",
        "git push --all",
        "git push origin --tags",
    ];

    for cmd in &push_commands {
        let (temp_dir, event_json) = setup_claude_code_event("block-all-push.yaml", cmd);

        let output = Command::cargo_bin("rulez")
            .expect("binary exists")
            .current_dir(temp_dir.path())
            .write_stdin(event_json)
            .output()
            .expect("command should run");

        assert_eq!(
            output.status.code(),
            Some(2),
            "Command '{cmd}' MUST exit with code 2 (blocked)"
        );
    }

    evidence.pass(
        &format!("All {} git push variants exit code 2", push_commands.len()),
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// ==========================================================================
// Test 5: Non-push git commands all exit code 0
// ==========================================================================

#[test]
fn test_e2e_non_push_git_commands_exit_code_0() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_non_push_exit0", "E2E");

    let safe_commands = vec![
        "git status",
        "git log --oneline -5",
        "git diff",
        "git add .",
        "git commit -m 'test'",
        "git branch -a",
        "git fetch origin",
        "git pull origin main",
        "git stash",
        "git checkout -b new-branch",
    ];

    for cmd in &safe_commands {
        let (temp_dir, event_json) = setup_claude_code_event("block-all-push.yaml", cmd);

        let output = Command::cargo_bin("rulez")
            .expect("binary exists")
            .current_dir(temp_dir.path())
            .write_stdin(event_json)
            .output()
            .expect("command should run");

        assert!(
            output.status.success(),
            "Command '{cmd}' should exit 0 (allowed)"
        );
    }

    evidence.pass(
        &format!(
            "All {} non-push git commands exit code 0",
            safe_commands.len()
        ),
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// ==========================================================================
// Test 6: Blocked = stderr reason, Allowed = JSON stdout
// ==========================================================================

/// Verify the output format matches Claude Code's expectations:
/// - Blocked: exit 2, reason on stderr, NO JSON on stdout
/// - Allowed: exit 0, JSON on stdout with "continue":true
#[test]
fn test_e2e_output_format_claude_code_protocol() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_output_format", "E2E");

    // === Blocked response ===
    let (temp_dir, event_json) = setup_claude_code_event("block-all-push.yaml", "git push");

    let blocked_output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_json)
        .output()
        .expect("command should run");

    assert_eq!(blocked_output.status.code(), Some(2), "Blocked = exit 2");

    let stderr = String::from_utf8_lossy(&blocked_output.stderr);
    assert!(!stderr.is_empty(), "Blocked must have stderr reason");
    assert!(
        stderr.contains("Blocked"),
        "stderr should describe the block"
    );

    // === Allowed response ===
    let (temp_dir2, event_json2) = setup_claude_code_event("block-all-push.yaml", "git status");

    let allowed_output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir2.path())
        .write_stdin(event_json2)
        .output()
        .expect("command should run");

    assert!(allowed_output.status.success(), "Allowed = exit 0");

    let stdout = String::from_utf8_lossy(&allowed_output.stdout);
    let stdout_str = stdout.trim();

    // Must be valid JSON with "continue":true
    assert!(
        stdout_str.contains(r#""continue":true"#) || stdout_str.contains(r#""continue": true"#),
        "Allowed response JSON must have 'continue':true, got: {stdout_str}"
    );

    // Must NOT contain "continue_"
    assert!(
        !stdout_str.contains("continue_"),
        "Must not contain 'continue_', got: {stdout_str}"
    );

    evidence.pass(
        "Output format matches Claude Code protocol (exit 2 + stderr / exit 0 + JSON)",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// ==========================================================================
// Test 7: No config = allow all (exit 0, fail-open)
// ==========================================================================

#[test]
fn test_e2e_no_config_allows_all() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_no_config_allows", "E2E");

    let empty_dir = tempfile::tempdir().expect("create empty dir");
    let cwd = canonicalize_path(empty_dir.path())
        .to_string_lossy()
        .to_string();

    let event = serde_json::json!({
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": { "command": "git push --force" },
        "session_id": "e2e-no-config",
        "cwd": cwd
    });

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(empty_dir.path())
        .write_stdin(serde_json::to_string(&event).unwrap())
        .output()
        .expect("command should run");

    assert!(
        output.status.success(),
        "No config = exit 0 (fail-open, allow all)"
    );

    let response = CchResponse::from_output(&output).expect("should parse response");
    assert!(
        response.continue_,
        "With no hooks.yaml, everything should be allowed"
    );

    evidence.pass("No config = exit 0, all allowed", timer.elapsed_ms());
    let _ = evidence.save(&evidence_dir());
}

// ==========================================================================
// Test 8: CWD + push variants from wrong dir = all exit code 2
// ==========================================================================

#[test]
fn test_e2e_cwd_git_push_variants_from_wrong_dir() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_cwd_push_variants_wrong_dir", "E2E");

    let push_commands = vec![
        "git push",
        "git push origin main",
        "git push --force origin main",
    ];

    let wrong_dir = tempfile::tempdir().expect("create wrong dir");

    for cmd in &push_commands {
        let (_temp_dir, event_json) = setup_claude_code_event("block-all-push.yaml", cmd);

        let output = Command::cargo_bin("rulez")
            .expect("binary exists")
            .current_dir(wrong_dir.path())
            .write_stdin(event_json)
            .output()
            .expect("command should run");

        assert_eq!(
            output.status.code(),
            Some(2),
            "Command '{cmd}' MUST exit 2 even from wrong CWD"
        );
    }

    evidence.pass(
        &format!(
            "All {} push variants exit 2 from wrong CWD",
            push_commands.len()
        ),
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

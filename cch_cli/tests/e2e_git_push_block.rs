//! End-to-End Tests: Git Push Block via Claude Code Protocol
//!
//! These tests simulate exactly what Claude Code does when invoking CCH:
//! - Sends JSON via stdin with `hook_event_name` (NOT `event_type`)
//! - Includes `cwd` field pointing to the project directory
//! - Does NOT send `timestamp` (CCH defaults to Utc::now())
//! - Includes extra fields: transcript_path, permission_mode, tool_use_id
//!
//! The critical scenario tested: CCH is invoked from a DIFFERENT directory
//! than the project, but uses the event's `cwd` to find the project's hooks.yaml.

#![allow(deprecated)]
#![allow(unused_imports)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[path = "common/mod.rs"]
mod common;
use common::{CchResponse, TestEvidence, Timer, evidence_dir, fixture_path, setup_test_env};

/// Helper: create a test environment and return (temp_dir, event_json)
/// The event JSON uses `hook_event_name` and has `cwd` set to the temp dir path.
fn setup_claude_code_event(config_name: &str, command: &str) -> (tempfile::TempDir, String) {
    let temp_dir = setup_test_env(config_name);
    let cwd = temp_dir.path().to_string_lossy().to_string();

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
// Test 1: Basic git push block using Claude Code protocol
// ==========================================================================

/// Simulate Claude Code sending a `git push` event with `hook_event_name` and `cwd`.
/// CCH should block it when the project has a block-all-push rule.
#[test]
fn test_e2e_git_push_blocked_claude_code_protocol() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_git_push_blocked", "E2E");

    let (temp_dir, event_json) = setup_claude_code_event("block-all-push.yaml", "git push");

    // Run CCH with current_dir set to the project (simple case)
    let output = Command::cargo_bin("cch")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_json)
        .output()
        .expect("command should run");

    assert!(output.status.success(), "CCH should exit 0");

    let response = CchResponse::from_output(&output).expect("should parse response");

    assert!(
        !response.continue_,
        "git push MUST be blocked (continue should be false)"
    );
    assert!(
        response.reason.is_some(),
        "blocked response must include a reason"
    );
    let reason = response.reason.unwrap();
    assert!(
        reason.contains("block-git-push"),
        "reason should reference the rule name, got: {reason}"
    );

    evidence.pass(
        &format!("git push correctly blocked with reason: {reason}"),
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// ==========================================================================
// Test 2: CRITICAL - CWD-based config loading (the bug that was fixed)
// ==========================================================================

/// This is the critical test: CCH is invoked from a DIFFERENT directory
/// than the project, but the event's `cwd` field points to the project.
/// CCH must use `cwd` to find the correct hooks.yaml.
///
/// This was the root cause of git push not being blocked in production:
/// Claude Code invokes CCH from an arbitrary directory, and CCH was using
/// `current_dir()` instead of the event's `cwd` to locate hooks.yaml.
#[test]
fn test_e2e_cwd_based_config_loading() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_cwd_config_loading", "E2E");

    let (temp_dir, event_json) = setup_claude_code_event("block-all-push.yaml", "git push");

    // Create a DIFFERENT directory that has NO hooks.yaml
    let wrong_dir = tempfile::tempdir().expect("create wrong dir");

    // Run CCH from the WRONG directory, but with cwd pointing to the project
    let output = Command::cargo_bin("cch")
        .expect("binary exists")
        .current_dir(wrong_dir.path()) // <-- WRONG dir, no hooks.yaml here
        .write_stdin(event_json)
        .output()
        .expect("command should run");

    assert!(output.status.success(), "CCH should exit 0");

    let response = CchResponse::from_output(&output).expect("should parse response");

    assert!(
        !response.continue_,
        "git push MUST be blocked even when CWD differs from project dir.\n\
         CCH must use event.cwd to find hooks.yaml.\n\
         Response: {:?}",
        response.continue_
    );
    assert!(
        response.reason.as_ref().unwrap().contains("block-git-push"),
        "reason should reference the rule name"
    );

    // Also verify the temp_dir still has hooks.yaml
    assert!(
        temp_dir.path().join(".claude/hooks.yaml").exists(),
        "hooks.yaml should exist in the project dir"
    );

    evidence.pass(
        "git push blocked via cwd-based config loading (CWD != project dir)",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// ==========================================================================
// Test 3: Safe commands are allowed
// ==========================================================================

/// Git status should NOT be blocked by the block-all-push rule.
#[test]
fn test_e2e_git_status_allowed() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_git_status_allowed", "E2E");

    let (temp_dir, event_json) = setup_claude_code_event("block-all-push.yaml", "git status");

    let output = Command::cargo_bin("cch")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_json)
        .output()
        .expect("command should run");

    assert!(output.status.success(), "CCH should exit 0");

    let response = CchResponse::from_output(&output).expect("should parse response");

    assert!(
        response.continue_,
        "git status should be allowed (continue should be true)"
    );

    evidence.pass("git status correctly allowed", timer.elapsed_ms());
    let _ = evidence.save(&evidence_dir());
}

// ==========================================================================
// Test 4: Various git push variants are all blocked
// ==========================================================================

#[test]
fn test_e2e_git_push_variants_blocked() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_git_push_variants", "E2E");

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

        let output = Command::cargo_bin("cch")
            .expect("binary exists")
            .current_dir(temp_dir.path())
            .write_stdin(event_json)
            .output()
            .expect("command should run");

        let response = CchResponse::from_output(&output).expect("should parse response");

        assert!(
            !response.continue_,
            "Command '{cmd}' MUST be blocked but was allowed"
        );
    }

    evidence.pass(
        &format!(
            "All {} git push variants correctly blocked",
            push_commands.len()
        ),
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// ==========================================================================
// Test 5: Non-push git commands are allowed
// ==========================================================================

#[test]
fn test_e2e_non_push_git_commands_allowed() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_non_push_allowed", "E2E");

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

        let output = Command::cargo_bin("cch")
            .expect("binary exists")
            .current_dir(temp_dir.path())
            .write_stdin(event_json)
            .output()
            .expect("command should run");

        let response = CchResponse::from_output(&output).expect("should parse response");

        assert!(
            response.continue_,
            "Command '{cmd}' should be ALLOWED but was blocked"
        );
    }

    evidence.pass(
        &format!(
            "All {} non-push git commands correctly allowed",
            safe_commands.len()
        ),
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// ==========================================================================
// Test 6: Response format matches Claude Code expectations
// ==========================================================================

/// Claude Code expects the response JSON to have `"continue"` (not `"continue_"`).
/// Verify the exact JSON output format.
#[test]
fn test_e2e_response_json_format() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_response_format", "E2E");

    // Test blocked response format
    let (temp_dir, event_json) = setup_claude_code_event("block-all-push.yaml", "git push");

    let output = Command::cargo_bin("cch")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_json)
        .output()
        .expect("command should run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stdout_str = stdout.trim();

    // Must contain "continue" (not "continue_")
    assert!(
        stdout_str.contains(r#""continue":false"#) || stdout_str.contains(r#""continue": false"#),
        "Blocked response must contain '\"continue\":false', got: {stdout_str}"
    );

    // Must NOT contain "continue_"
    assert!(
        !stdout_str.contains("continue_"),
        "Response must NOT contain 'continue_' (serde rename required), got: {stdout_str}"
    );

    // Must contain "reason"
    assert!(
        stdout_str.contains(r#""reason""#),
        "Blocked response must contain 'reason' field, got: {stdout_str}"
    );

    // Must be valid JSON
    let parsed: serde_json::Value =
        serde_json::from_str(stdout_str).expect("response must be valid JSON");
    assert_eq!(
        parsed["continue"], false,
        "JSON 'continue' field must be false"
    );

    // Test allowed response format
    let (temp_dir2, event_json2) = setup_claude_code_event("block-all-push.yaml", "git status");

    let output2 = Command::cargo_bin("cch")
        .expect("binary exists")
        .current_dir(temp_dir2.path())
        .write_stdin(event_json2)
        .output()
        .expect("command should run");

    let stdout2 = String::from_utf8_lossy(&output2.stdout);
    let stdout_str2 = stdout2.trim();

    assert!(
        stdout_str2.contains(r#""continue":true"#) || stdout_str2.contains(r#""continue": true"#),
        "Allowed response must contain '\"continue\":true', got: {stdout_str2}"
    );

    assert!(
        !stdout_str2.contains("continue_"),
        "Response must NOT contain 'continue_', got: {stdout_str2}"
    );

    evidence.pass(
        "Response JSON format matches Claude Code expectations",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// ==========================================================================
// Test 7: No config = allow all (fail-open behavior)
// ==========================================================================

/// When there's no hooks.yaml in the project dir and no global config,
/// CCH should allow everything (fail-open).
#[test]
fn test_e2e_no_config_allows_all() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_no_config_allows", "E2E");

    // Create a temp dir with NO .claude/hooks.yaml
    let empty_dir = tempfile::tempdir().expect("create empty dir");
    let cwd = empty_dir.path().to_string_lossy().to_string();

    let event = serde_json::json!({
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": { "command": "git push --force" },
        "session_id": "e2e-no-config",
        "cwd": cwd
    });

    let output = Command::cargo_bin("cch")
        .expect("binary exists")
        .current_dir(empty_dir.path())
        .write_stdin(serde_json::to_string(&event).unwrap())
        .output()
        .expect("command should run");

    assert!(
        output.status.success(),
        "CCH should exit 0 even with no config"
    );

    let response = CchResponse::from_output(&output).expect("should parse response");

    assert!(
        response.continue_,
        "With no hooks.yaml, everything should be allowed (fail-open)"
    );

    evidence.pass(
        "No config = all commands allowed (fail-open)",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// ==========================================================================
// Test 8: CWD-based loading with git push variants from wrong directory
// ==========================================================================

/// The critical combined test: invoked from WRONG dir, with various git push
/// variants, all must be blocked via cwd-based config loading.
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

        // Run from WRONG directory
        let output = Command::cargo_bin("cch")
            .expect("binary exists")
            .current_dir(wrong_dir.path())
            .write_stdin(event_json)
            .output()
            .expect("command should run");

        let response = CchResponse::from_output(&output).expect("should parse response");

        assert!(
            !response.continue_,
            "Command '{cmd}' MUST be blocked even from wrong CWD"
        );
    }

    evidence.pass(
        &format!(
            "All {} push variants blocked from wrong CWD via event.cwd",
            push_commands.len()
        ),
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

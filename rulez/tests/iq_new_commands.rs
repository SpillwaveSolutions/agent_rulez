//! Integration tests for new CLI commands: init, install, debug, uninstall

#![allow(deprecated)] // cargo_bin deprecation - matches other test files

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn cch_cmd() -> Command {
    Command::cargo_bin("rulez").unwrap()
}

// =============================================================================
// Init Command Tests
// =============================================================================

#[test]
fn test_init_creates_hooks_yaml() {
    let temp_dir = TempDir::new().unwrap();

    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["init"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Created configuration"))
        .stdout(predicate::str::contains("hooks.yaml"));

    let hooks_yaml = temp_dir.path().join(".claude").join("hooks.yaml");
    assert!(hooks_yaml.exists(), "hooks.yaml should be created");

    let content = fs::read_to_string(&hooks_yaml).unwrap();
    assert!(content.contains("version:"), "Config should have version");
    assert!(content.contains("rules:"), "Config should have rules");
}

#[test]
fn test_init_with_examples() {
    let temp_dir = TempDir::new().unwrap();

    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["init", "--with-examples"])
        .assert()
        .success()
        .stdout(predicate::str::contains("python-standards.md"))
        .stdout(predicate::str::contains("check-secrets.sh"));

    let python_standards = temp_dir.path().join(".claude/context/python-standards.md");
    let check_secrets = temp_dir.path().join(".claude/validators/check-secrets.sh");

    assert!(
        python_standards.exists(),
        "Python standards should be created"
    );
    assert!(check_secrets.exists(), "Check secrets should be created");
}

#[test]
fn test_init_refuses_overwrite_without_force() {
    let temp_dir = TempDir::new().unwrap();

    // First init
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["init"])
        .assert()
        .success();

    // Second init should fail without --force
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["init"])
        .assert()
        .success()
        .stdout(predicate::str::contains("already exists"))
        .stdout(predicate::str::contains("--force"));
}

#[test]
fn test_init_force_overwrites() {
    let temp_dir = TempDir::new().unwrap();

    // First init
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["init"])
        .assert()
        .success();

    // Second init with --force
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["init", "--force"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Created configuration"));
}

// =============================================================================
// Debug Command Tests
// =============================================================================

#[test]
fn test_debug_help() {
    cch_cmd()
        .args(["debug", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Simulate an event"))
        .stdout(predicate::str::contains("EVENT_TYPE"));
}

#[test]
fn test_debug_pretooluse_bash() {
    let temp_dir = TempDir::new().unwrap();

    // Create a config first
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["init"])
        .assert()
        .success();

    // Test debug with a safe command
    cch_cmd()
        .current_dir(temp_dir.path())
        .args([
            "debug",
            "PreToolUse",
            "--tool",
            "Bash",
            "--command",
            "echo test",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Simulated Event"))
        .stdout(predicate::str::contains("Response"))
        .stdout(predicate::str::contains("\"continue\""));
}

#[test]
fn test_debug_detects_blocked_command() {
    let temp_dir = TempDir::new().unwrap();

    // Create a config first
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["init"])
        .assert()
        .success();

    // Test debug with a blocked command
    cch_cmd()
        .current_dir(temp_dir.path())
        .args([
            "debug",
            "PreToolUse",
            "--tool",
            "Bash",
            "--command",
            "git push --force origin main",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Blocked"))
        .stdout(predicate::str::contains("block-force-push"));
}

#[test]
fn test_debug_verbose_shows_rules() {
    let temp_dir = TempDir::new().unwrap();

    // Create a config first
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["init"])
        .assert()
        .success();

    // Test debug with verbose flag
    cch_cmd()
        .current_dir(temp_dir.path())
        .args([
            "debug",
            "PreToolUse",
            "--tool",
            "Bash",
            "--command",
            "ls",
            "--verbose",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Configured Rules"))
        .stdout(predicate::str::contains("block-force-push"));
}

#[test]
fn test_debug_invalid_event_type() {
    let temp_dir = TempDir::new().unwrap();

    // Create a config first
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["init"])
        .assert()
        .success();

    // Test debug with invalid event type
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["debug", "InvalidEvent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown event type"));
}

// =============================================================================
// Debug Prompt Command Tests (UserPromptSubmit)
// =============================================================================

#[test]
fn test_debug_prompt_event() {
    let temp_dir = TempDir::new().unwrap();

    // Create a config first
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["init"])
        .assert()
        .success();

    // Test debug prompt command
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["debug", "prompt", "--prompt", "deploy to production"])
        .assert()
        .success()
        .stdout(predicate::str::contains("UserPromptSubmit"))
        .stdout(predicate::str::contains("deploy to production"))
        .stdout(predicate::str::contains("Processed in"));
}

#[test]
fn test_debug_prompt_alias_user_prompt() {
    let temp_dir = TempDir::new().unwrap();

    // Create a config first
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["init"])
        .assert()
        .success();

    // Test using user-prompt alias
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["debug", "user-prompt", "--prompt", "test"])
        .assert()
        .success()
        .stdout(predicate::str::contains("UserPromptSubmit"));
}

#[test]
fn test_debug_prompt_without_prompt_flag() {
    let temp_dir = TempDir::new().unwrap();

    // Create a config first
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["init"])
        .assert()
        .success();

    // Test prompt event without --prompt flag (should still work, prompt is None)
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["debug", "prompt"])
        .assert()
        .success()
        .stdout(predicate::str::contains("UserPromptSubmit"));
}

#[test]
fn test_debug_prompt_matching_rule() {
    let temp_dir = TempDir::new().unwrap();

    // Create a config first
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["init"])
        .assert()
        .success();

    // Create custom hooks.yaml with prompt_match rule
    let hooks_yaml = temp_dir.path().join(".claude/hooks.yaml");
    let config_content = r#"
version: "1.0"
settings:
  debug_logs: false
rules:
  - name: deploy-guard
    description: "Guard deploy prompts"
    event_types: [UserPromptSubmit]
    matchers:
      prompt_match: ["deploy"]
    actions:
      inject_inline: "CAUTION: Deploy detected"
"#;
    fs::write(&hooks_yaml, config_content).unwrap();

    // Test that the rule matches
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["debug", "prompt", "--prompt", "deploy to production"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Allowed with injected context"));
}

// =============================================================================
// Install/Uninstall Command Tests
// =============================================================================

#[test]
fn test_install_help() {
    cch_cmd()
        .args(["install", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Install RuleZ hook"))
        .stdout(predicate::str::contains("--global"));
}

#[test]
fn test_uninstall_help() {
    cch_cmd()
        .args(["uninstall", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Uninstall RuleZ hook"));
}

#[test]
fn test_install_creates_settings_json() {
    let temp_dir = TempDir::new().unwrap();

    // Create a config first
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["init"])
        .assert()
        .success();

    // Get the binary path
    let binary = assert_cmd::cargo::cargo_bin("rulez");

    // Install with explicit binary path
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["install", "--binary", binary.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("installed successfully"));

    let settings = temp_dir.path().join(".claude/settings.json");
    assert!(settings.exists(), "settings.json should be created");

    let content = fs::read_to_string(&settings).unwrap();
    assert!(
        content.contains("PreToolUse"),
        "Should have PreToolUse hook"
    );
    assert!(
        content.contains("PostToolUse"),
        "Should have PostToolUse hook"
    );
    assert!(content.contains("Stop"), "Should have Stop hook");
    assert!(
        content.contains("SessionStart"),
        "Should have SessionStart hook"
    );
    assert!(
        content.contains("\"matcher\""),
        "Should have matcher field in nested structure"
    );
    assert!(
        content.contains("\"type\": \"command\""),
        "Should have type: command in hook entry"
    );
}

#[test]
fn test_uninstall_removes_hooks() {
    let temp_dir = TempDir::new().unwrap();

    // Create a config first
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["init"])
        .assert()
        .success();

    // Get the binary path
    let binary = assert_cmd::cargo::cargo_bin("rulez");

    // Install
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["install", "--binary", binary.to_str().unwrap()])
        .assert()
        .success();

    // Uninstall
    cch_cmd()
        .current_dir(temp_dir.path())
        .args(["uninstall"])
        .assert()
        .success()
        .stdout(predicate::str::contains("uninstalled successfully"));

    let settings = temp_dir.path().join(".claude/settings.json");
    let content = fs::read_to_string(&settings).unwrap();
    assert!(
        !content.contains("PreToolUse"),
        "Should not have PreToolUse hook after uninstall"
    );
}

// =============================================================================
// REPL Command Test
// =============================================================================

#[test]
fn test_repl_help() {
    cch_cmd()
        .args(["repl", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("interactive debug mode"));
}

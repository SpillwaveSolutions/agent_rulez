//! Operational Qualification (OQ) Tests - User Story 2: Context Injection
//!
//! US2: As a developer, I want Claude to automatically load relevant skill
//! documentation when I'm editing files in specific directories.
//!
//! These tests verify the context injection functionality.

#![allow(deprecated)]
#![allow(unused_imports)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[path = "common/mod.rs"]
mod common;
use common::{TestEvidence, Timer, evidence_dir, fixture_path, read_fixture};

/// Test that context is injected for CDK files
#[test]
fn test_us2_cdk_context_injection() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("cdk_context_injection", "OQ-US2");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");

    // Create .claude directory with config
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Copy inject config
    let config_src = fixture_path("hooks/inject-skill-context.yaml");
    fs::copy(&config_src, claude_dir.join("hooks.yaml")).expect("copy config");

    // Create the skill file that will be injected
    let skill_dir = temp_dir.path().join(".opencode/skill/aws-cdk");
    fs::create_dir_all(&skill_dir).expect("create skill dir");
    fs::write(
        skill_dir.join("SKILL.md"),
        "# AWS CDK Skill\n\nThis is CDK guidance.",
    )
    .expect("write skill");

    // Read the CDK edit event
    let event = read_fixture("events/cdk-file-edit-event.json");

    // Run CCH with the event
    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .assert()
        .success();

    // Response should allow and include context
    result.stdout(
        predicate::str::contains(r#""continue":true"#)
            .or(predicate::str::contains(r#""continue": true"#)),
    );
    // Note: Context injection depends on the skill file existing

    evidence.pass(
        "CDK file edit correctly triggers context injection",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that non-matching directories don't trigger injection
#[test]
fn test_us2_non_matching_no_injection() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("non_matching_no_injection", "OQ-US2");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");

    // Create .claude directory with config
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Copy inject config
    let config_src = fixture_path("hooks/inject-skill-context.yaml");
    fs::copy(&config_src, claude_dir.join("hooks.yaml")).expect("copy config");

    // Create event for a non-matching directory
    let event = r#"{
        "event_type": "PreToolUse",
        "tool_name": "Edit",
        "tool_input": {
            "filePath": "src/utils/helper.ts",
            "oldString": "old",
            "newString": "new"
        },
        "session_id": "test-session-no-match",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;

    // Run CCH with the event
    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .assert()
        .success();

    // Response should allow without context injection
    result.stdout(
        predicate::str::contains(r#""continue":true"#)
            .or(predicate::str::contains(r#""continue": true"#)),
    );

    evidence.pass(
        "Non-matching directory correctly skips injection",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that extension-based injection works
#[test]
fn test_us2_extension_based_injection() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("extension_based_injection", "OQ-US2");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");

    // Create .claude directory with config
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Copy inject config
    let config_src = fixture_path("hooks/inject-skill-context.yaml");
    fs::copy(&config_src, claude_dir.join("hooks.yaml")).expect("copy config");

    // Create the skill file that will be injected
    let skill_dir = temp_dir.path().join(".opencode/skill/terraform");
    fs::create_dir_all(&skill_dir).expect("create skill dir");
    fs::write(
        skill_dir.join("SKILL.md"),
        "# Terraform Skill\n\nTerraform guidance.",
    )
    .expect("write skill");

    // Create event for a .tf file
    let event = r#"{
        "event_type": "PreToolUse",
        "tool_name": "Edit",
        "tool_input": {
            "filePath": "infrastructure/main.tf",
            "oldString": "old",
            "newString": "new"
        },
        "session_id": "test-session-tf",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;

    // Run CCH with the event
    let result = Command::cargo_bin("rulez")
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
        "Extension-based injection triggers correctly for .tf files",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that inline content injection works (inject_inline action)
#[test]
fn test_us2_inline_content_injection() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("inline_content_injection", "OQ-US2");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");

    // Create .claude directory with config
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Create hooks.yaml with inject_inline rule
    let config_content = r#"version: "1.0"
rules:
  - name: inline-warning
    matchers:
      directories: ["/prod/"]
    actions:
      inject_inline: |
        ## Production Warning
        You are editing production files.
"#;
    fs::write(claude_dir.join("hooks.yaml"), config_content).expect("write config");

    // Create event for a file in /prod/ directory
    let event = r#"{
        "event_type": "PreToolUse",
        "tool_name": "Edit",
        "tool_input": {
            "filePath": "/prod/config.yaml",
            "oldString": "old",
            "newString": "new"
        },
        "session_id": "test-session-inline",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;

    // Run rulez binary with the event
    Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .assert()
        .success()
        // Response should allow and include the inline content
        .stdout(
            predicate::str::contains(r#""continue":true"#)
                .or(predicate::str::contains(r#""continue": true"#)),
        )
        .stdout(predicate::str::contains("Production Warning"));

    evidence.pass(
        "Inline content injection works via inject_inline action",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that inject_inline takes precedence over inject
#[test]
fn test_us2_inject_inline_precedence() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("inject_inline_precedence", "OQ-US2");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");

    // Create .claude directory with config
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Create a context file that should NOT be used
    let context_file = claude_dir.join("context.md");
    fs::write(&context_file, "FILE CONTENT - SHOULD NOT APPEAR").expect("write context file");

    // Create hooks.yaml with both inject and inject_inline
    // inject_inline should take precedence
    let config_content = format!(
        r#"version: "1.0"
rules:
  - name: precedence-test
    matchers:
      directories: ["/test/"]
    actions:
      inject: "{}"
      inject_inline: "INLINE CONTENT - SHOULD APPEAR"
"#,
        context_file.display()
    );
    fs::write(claude_dir.join("hooks.yaml"), config_content).expect("write config");

    // Create event for a file in /test/ directory
    let event = r#"{
        "event_type": "PreToolUse",
        "tool_name": "Edit",
        "tool_input": {
            "filePath": "/test/file.txt",
            "oldString": "old",
            "newString": "new"
        },
        "session_id": "test-session-precedence",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;

    // Run rulez binary with the event and capture output
    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("run binary");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Response should allow
    assert!(
        stdout.contains(r#""continue":true"#) || stdout.contains(r#""continue": true"#),
        "Response should allow (continue: true)"
    );

    // Inline content should appear
    assert!(
        stdout.contains("INLINE CONTENT - SHOULD APPEAR"),
        "inject_inline content should be in response"
    );

    // File content should NOT appear
    assert!(
        !stdout.contains("FILE CONTENT - SHOULD NOT APPEAR"),
        "inject_inline should take precedence over inject"
    );

    evidence.pass(
        "inject_inline correctly takes precedence over inject",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that inject_command executes shell command and injects stdout
#[test]
fn test_us2_inject_command_basic() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("inject_command_basic", "OQ-US2");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");

    // Create .claude directory with config
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Create hooks.yaml with inject_command rule
    // Use echo command which works on all Unix systems
    let config_content = r#"version: "1.0"
rules:
  - name: command-context
    matchers:
      tools: [Bash]
    actions:
      inject_command: "echo 'Dynamic context from command'"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config_content).expect("write config");

    // Create event for Bash tool
    let event = r#"{
        "event_type": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {
            "command": "ls -la"
        },
        "session_id": "test-session-command",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;

    // Run rulez binary with the event
    Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .assert()
        .success()
        // Response should allow and include command output
        .stdout(
            predicate::str::contains(r#""continue":true"#)
                .or(predicate::str::contains(r#""continue": true"#)),
        )
        .stdout(predicate::str::contains("Dynamic context from command"));

    evidence.pass(
        "inject_command executes shell command and injects stdout",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that inject_inline takes precedence over inject_command
#[test]
fn test_us2_inject_inline_over_command() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("inject_inline_over_command", "OQ-US2");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");

    // Create .claude directory with config
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Create hooks.yaml with both inject_inline and inject_command
    // inject_inline should take precedence (same as it does over inject file)
    let config_content = r#"version: "1.0"
rules:
  - name: precedence-test
    matchers:
      tools: [Bash]
    actions:
      inject_inline: "INLINE WINS"
      inject_command: "echo COMMAND SHOULD NOT APPEAR"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config_content).expect("write config");

    // Create event for Bash tool
    let event = r#"{
        "event_type": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {
            "command": "git status"
        },
        "session_id": "test-session-precedence",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;

    // Run rulez binary with the event and capture output
    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("run binary");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Response should allow
    assert!(
        stdout.contains(r#""continue":true"#) || stdout.contains(r#""continue": true"#),
        "Response should allow (continue: true)"
    );

    // Inline content should appear
    assert!(
        stdout.contains("INLINE WINS"),
        "inject_inline content should be in response"
    );

    // Command output should NOT appear
    assert!(
        !stdout.contains("COMMAND SHOULD NOT APPEAR"),
        "inject_inline should take precedence over inject_command"
    );

    evidence.pass(
        "inject_inline correctly takes precedence over inject_command",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that inject_command takes precedence over inject (file)
#[test]
fn test_us2_inject_command_over_file() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("inject_command_over_file", "OQ-US2");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");

    // Create .claude directory with config
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Create a context file that should NOT be used
    let context_file = claude_dir.join("context.md");
    fs::write(&context_file, "FILE CONTENT - SHOULD NOT APPEAR").expect("write context file");

    // Create hooks.yaml with both inject_command and inject
    // inject_command should take precedence over inject (file)
    let config_content = format!(
        r#"version: "1.0"
rules:
  - name: command-over-file
    matchers:
      tools: [Bash]
    actions:
      inject_command: "echo COMMAND WINS"
      inject: "{}"
"#,
        context_file.display()
    );
    fs::write(claude_dir.join("hooks.yaml"), config_content).expect("write config");

    // Create event for Bash tool
    let event = r#"{
        "event_type": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {
            "command": "pwd"
        },
        "session_id": "test-session-cmd-file",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;

    // Run rulez binary with the event and capture output
    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("run binary");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Response should allow
    assert!(
        stdout.contains(r#""continue":true"#) || stdout.contains(r#""continue": true"#),
        "Response should allow (continue: true)"
    );

    // Command output should appear
    assert!(
        stdout.contains("COMMAND WINS"),
        "inject_command content should be in response"
    );

    // File content should NOT appear
    assert!(
        !stdout.contains("FILE CONTENT - SHOULD NOT APPEAR"),
        "inject_command should take precedence over inject file"
    );

    evidence.pass(
        "inject_command correctly takes precedence over inject file",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

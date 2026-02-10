//! Integration Tests for Prompt Matching (Phase 4 Plan 4)
//!
//! End-to-end tests using evaluate_rules() to verify prompt matching:
//! - PROMPT-01: Regex pattern matching on prompt text
//! - PROMPT-02: Case-insensitive matching
//! - PROMPT-03: Multiple patterns with any/all modes
//! - PROMPT-04: Anchor positions (start, end, contains)
//! - PROMPT-05: prompt variable available in evalexpr context
//!
//! These tests verify the full stack from YAML config through rule evaluation.

#![allow(deprecated)] // cargo_bin deprecation

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[path = "common/mod.rs"]
mod common;
use common::{TestEvidence, Timer, evidence_dir};

// =============================================================================
// YAML Round-Trip Tests
// =============================================================================

/// Test that simple array syntax deserializes and matches correctly
#[test]
fn test_yaml_roundtrip_simple_syntax() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("yaml_roundtrip_simple", "OQ-PROMPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Simple array syntax
    let config = r#"
version: "1.0"
rules:
  - name: block-delete
    matchers:
      prompt_match: ["delete", "drop"]
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Validate config parses correctly
    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .arg("validate")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    // Output says "Configuration syntax is valid" with checkmarks
    result.stdout(predicate::str::contains("syntax is valid"));

    evidence.pass(
        "Simple array syntax parses and validates correctly",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that complex object syntax deserializes and matches correctly
#[test]
fn test_yaml_roundtrip_complex_syntax() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("yaml_roundtrip_complex", "OQ-PROMPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Complex object syntax with all options
    let config = r#"
version: "1.0"
rules:
  - name: block-credentials
    matchers:
      prompt_match:
        patterns: ["password", "secret"]
        mode: any
        case_insensitive: true
        anchor: contains
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .arg("validate")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    // Output says "Configuration syntax is valid" with checkmarks
    result.stdout(predicate::str::contains("syntax is valid"));

    evidence.pass(
        "Complex object syntax parses and validates correctly",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

// =============================================================================
// End-to-End Tests: evaluate_rules Integration
// =============================================================================

/// Test that matching prompt pattern blocks the operation
#[test]
fn test_e2e_prompt_match_blocks_matching_prompt() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_prompt_match_blocks", "OQ-PROMPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: block-dangerous-prompts
    matchers:
      prompt_match: ["delete", "drop", "rm -rf"]
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event with matching prompt
    let event = r#"{
        "hook_event_name": "UserPromptSubmit",
        "prompt": "please delete all the files",
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    assert_eq!(
        output.status.code(),
        Some(2),
        "Matching prompt should block. stdout: {}, stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("block-dangerous-prompts") || stderr.contains("Blocked"),
        "stderr should mention rule or blocking: {}",
        stderr
    );

    evidence.pass("Matching prompt correctly blocked", timer.elapsed_ms());
    let _ = evidence.save(&evidence_dir());
}

/// Test that non-matching prompt allows the operation
#[test]
fn test_e2e_prompt_match_no_match_no_block() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_prompt_no_match", "OQ-PROMPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: block-dangerous-prompts
    matchers:
      prompt_match: ["delete", "drop"]
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event with non-matching prompt
    let event = r#"{
        "hook_event_name": "UserPromptSubmit",
        "prompt": "please create a new file",
        "session_id": "test-session"
    }"#;

    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .assert()
        .success();

    result.stdout(
        predicate::str::contains(r#""continue":true"#)
            .or(predicate::str::contains(r#""continue": true"#)),
    );

    evidence.pass("Non-matching prompt correctly allowed", timer.elapsed_ms());
    let _ = evidence.save(&evidence_dir());
}

/// Test case-insensitive matching (PROMPT-02)
#[test]
fn test_e2e_prompt_match_case_insensitive() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_case_insensitive", "OQ-PROMPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: block-delete-any-case
    matchers:
      prompt_match:
        patterns: ["DELETE"]
        case_insensitive: true
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event with lowercase "delete" should match
    let event = r#"{
        "hook_event_name": "UserPromptSubmit",
        "prompt": "please delete the file",
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    assert_eq!(
        output.status.code(),
        Some(2),
        "Case-insensitive match should block"
    );

    evidence.pass(
        "Case-insensitive matching works correctly",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test ALL mode requires both patterns (PROMPT-03)
#[test]
fn test_e2e_prompt_match_all_mode_requires_both() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_all_mode", "OQ-PROMPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: require-both-keywords
    matchers:
      prompt_match:
        patterns: ["database", "production"]
        mode: all
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Test 1: Only one keyword - should NOT block
    let event_one = r#"{
        "hook_event_name": "UserPromptSubmit",
        "prompt": "access the database",
        "session_id": "test-session"
    }"#;

    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_one)
        .assert()
        .success();

    result.stdout(
        predicate::str::contains(r#""continue":true"#)
            .or(predicate::str::contains(r#""continue": true"#)),
    );

    // Test 2: Both keywords - should block
    let event_both = r#"{
        "hook_event_name": "UserPromptSubmit",
        "prompt": "access the production database",
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_both)
        .output()
        .expect("command should run");

    assert_eq!(
        output.status.code(),
        Some(2),
        "Both keywords should block in ALL mode"
    );

    evidence.pass(
        "ALL mode correctly requires all patterns",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test anchor at start position (PROMPT-04)
#[test]
fn test_e2e_prompt_match_anchor_start() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_anchor_start", "OQ-PROMPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: starts-with-please
    matchers:
      prompt_match:
        patterns: ["please"]
        anchor: start
    actions:
      inject_inline: "Polite request detected"
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Test 1: Starts with "please" - should inject
    let event_starts = r#"{
        "hook_event_name": "UserPromptSubmit",
        "prompt": "please help me",
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_starts)
        .output()
        .expect("command should run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Polite request detected"),
        "Should inject when starts with 'please': {}",
        stdout
    );

    // Test 2: "please" in middle - should NOT inject
    let event_middle = r#"{
        "hook_event_name": "UserPromptSubmit",
        "prompt": "could you please help",
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_middle)
        .output()
        .expect("command should run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("Polite request detected"),
        "Should NOT inject when 'please' not at start: {}",
        stdout
    );

    evidence.pass("Anchor start position works correctly", timer.elapsed_ms());
    let _ = evidence.save(&evidence_dir());
}

/// Test negation patterns (not: prefix)
#[test]
fn test_e2e_prompt_match_negation() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_negation", "OQ-PROMPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Block if contains "delete" but NOT "safe"
    let config = r#"
version: "1.0"
rules:
  - name: block-unsafe-delete
    matchers:
      prompt_match:
        patterns: ["delete", "not:safe"]
        mode: all
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Test 1: "delete" without "safe" - should block
    let event_unsafe = r#"{
        "hook_event_name": "UserPromptSubmit",
        "prompt": "delete all the files",
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_unsafe)
        .output()
        .expect("command should run");

    assert_eq!(
        output.status.code(),
        Some(2),
        "Should block: has 'delete' and no 'safe'"
    );

    // Test 2: "delete" with "safe" - should NOT block
    let event_safe = r#"{
        "hook_event_name": "UserPromptSubmit",
        "prompt": "safely delete the temp files",
        "session_id": "test-session"
    }"#;

    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_safe)
        .assert()
        .success();

    result.stdout(
        predicate::str::contains(r#""continue":true"#)
            .or(predicate::str::contains(r#""continue": true"#)),
    );

    evidence.pass("Negation patterns work correctly", timer.elapsed_ms());
    let _ = evidence.save(&evidence_dir());
}

/// Test contains_word shorthand expansion
#[test]
fn test_e2e_prompt_match_contains_word_shorthand() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_contains_word", "OQ-PROMPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: block-delete-word
    matchers:
      prompt_match: ["contains_word:delete"]
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Test 1: "delete" as whole word - should block
    let event_word = r#"{
        "hook_event_name": "UserPromptSubmit",
        "prompt": "please delete the file",
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_word)
        .output()
        .expect("command should run");

    assert_eq!(
        output.status.code(),
        Some(2),
        "Should block: 'delete' is whole word"
    );

    // Test 2: "delete" as part of word - should NOT block
    let event_partial = r#"{
        "hook_event_name": "UserPromptSubmit",
        "prompt": "undelete the file",
        "session_id": "test-session"
    }"#;

    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_partial)
        .assert()
        .success();

    result.stdout(
        predicate::str::contains(r#""continue":true"#)
            .or(predicate::str::contains(r#""continue": true"#)),
    );

    evidence.pass(
        "contains_word shorthand works correctly",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test missing prompt field causes no match
#[test]
fn test_e2e_prompt_match_missing_prompt_no_match() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_missing_prompt", "OQ-PROMPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    let config = r#"
version: "1.0"
rules:
  - name: requires-prompt
    matchers:
      prompt_match: ["test"]
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event WITHOUT prompt field
    let event = r#"{
        "hook_event_name": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {"command": "echo test"},
        "session_id": "test-session"
    }"#;

    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .assert()
        .success();

    result.stdout(
        predicate::str::contains(r#""continue":true"#)
            .or(predicate::str::contains(r#""continue": true"#)),
    );

    evidence.pass(
        "Missing prompt correctly does not match",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test that prompt variable is accessible in enabled_when (PROMPT-05)
#[test]
fn test_e2e_prompt_variable_in_enabled_when() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_prompt_in_enabled_when", "OQ-PROMPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Rule only active when prompt is non-empty
    let config = r#"
version: "1.0"
rules:
  - name: prompt-enabled-rule
    enabled_when: 'prompt != ""'
    matchers:
      prompt_match: ["delete"]
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Event with prompt - rule should be enabled and block
    let event = r#"{
        "hook_event_name": "UserPromptSubmit",
        "prompt": "delete the file",
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command should run");

    assert_eq!(
        output.status.code(),
        Some(2),
        "Rule should block when prompt exists and matches"
    );

    evidence.pass(
        "prompt variable accessible in enabled_when context",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test prompt_match combined with other matchers
#[test]
fn test_e2e_prompt_match_combined_matchers() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("e2e_combined_matchers", "OQ-PROMPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Rule requires both tool match AND prompt match
    let config = r#"
version: "1.0"
rules:
  - name: bash-sudo-prompt
    matchers:
      tools: [Bash]
      prompt_match: ["sudo"]
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    // Test 1: Bash + sudo prompt - should block
    let event_match = r#"{
        "hook_event_name": "UserPromptSubmit",
        "tool_name": "Bash",
        "prompt": "run sudo command",
        "session_id": "test-session"
    }"#;

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_match)
        .output()
        .expect("command should run");

    assert_eq!(
        output.status.code(),
        Some(2),
        "Should block: Bash tool + sudo prompt"
    );

    // Test 2: Edit + sudo prompt - should NOT block (wrong tool)
    let event_wrong_tool = r#"{
        "hook_event_name": "UserPromptSubmit",
        "tool_name": "Edit",
        "prompt": "run sudo command",
        "session_id": "test-session"
    }"#;

    let result = Command::cargo_bin("rulez")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event_wrong_tool)
        .assert()
        .success();

    result.stdout(
        predicate::str::contains(r#""continue":true"#)
            .or(predicate::str::contains(r#""continue": true"#)),
    );

    evidence.pass("Combined matchers work correctly", timer.elapsed_ms());
    let _ = evidence.save(&evidence_dir());
}

/// Test validation rejects invalid regex patterns
#[test]
fn test_validation_invalid_regex_rejected() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("validation_invalid_regex", "OQ-PROMPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Invalid regex pattern (unclosed bracket)
    let config = r#"
version: "1.0"
rules:
  - name: invalid-regex
    matchers:
      prompt_match: ["[invalid"]
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .arg("validate")
        .current_dir(temp_dir.path())
        .output()
        .expect("command should run");

    assert!(
        !output.status.success(),
        "Validation should fail for invalid regex"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{} {}", stderr, stdout);

    assert!(
        combined.contains("invalid") || combined.contains("regex") || combined.contains("pattern"),
        "Error should mention invalid pattern: {}",
        combined
    );

    evidence.pass(
        "Validation correctly rejects invalid regex",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test validation rejects empty patterns
#[test]
fn test_validation_empty_patterns_rejected() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("validation_empty_patterns", "OQ-PROMPT");

    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");

    // Empty patterns array
    let config = r#"
version: "1.0"
rules:
  - name: empty-patterns
    matchers:
      prompt_match: []
    actions:
      block: true
"#;
    fs::write(claude_dir.join("hooks.yaml"), config).expect("Failed to write config");

    let output = Command::cargo_bin("rulez")
        .expect("binary exists")
        .arg("validate")
        .current_dir(temp_dir.path())
        .output()
        .expect("command should run");

    assert!(
        !output.status.success(),
        "Validation should fail for empty patterns"
    );

    evidence.pass(
        "Validation correctly rejects empty patterns",
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

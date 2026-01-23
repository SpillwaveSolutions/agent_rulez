# Feature: cch-binary-v1

## Overview
A Claude Code Hook (CCH) binary that provides safety and productivity features for Claude Code operations. The binary processes hook events to block dangerous operations, inject context, run custom validators, explain commands, and provide logging for troubleshooting.

## User Stories

### User Story 1: Block Dangerous Operations (Priority: P1)
**As a developer, I want CCH to automatically block dangerous git operations (like force push) so that I cannot accidentally overwrite team members' work.**

**Acceptance Criteria:**
- Block `git push --force` with clear error message
- Allow normal `git push` operations
- Log blocked operations with outcome "block"

### User Story 2: Inject Context for Skill Triggers (Priority: P1)
**As a developer, I want relevant skill documentation automatically injected when I edit files in specific directories, so that Claude always has the right context.**

**Acceptance Criteria:**
- Inject SKILL.md content when editing CDK files
- Support multiple directory patterns
- Aggregate contexts from multiple matching rules

### User Story 3: Run Custom Validators (Priority: P2)
**As a developer, I want to run custom Python scripts to enforce complex rules (like no console.log), so that I can create project-specific checks.**

**Acceptance Criteria:**
- Execute validator scripts with event JSON on stdin
- Block operations when script exits non-zero
- Inject script stdout as context when exit code is 0
- Fail-open with timeout (5s default)

### User Story 4: Explain Commands Before Permission (Priority: P2)
**As a developer, I want Claude to provide structured explanations before asking permission for commands, so that I understand what's happening.**

**Acceptance Criteria:**
- Inject explanation templates for PermissionRequest events
- Support required fields for command explanations
- Format explanations with explain-command.md template

### User Story 5: Query Logs for Troubleshooting (Priority: P3)
**As a developer, I want to query CCH logs to understand why rules did or didn't fire, so that I can debug my configuration.**

**Acceptance Criteria:**
- Query logs with `cch logs` command
- JSON Lines output with full provenance data
- Explain rules with `cch explain rule <name>`
- Include timing, rules matched, and outcome information

## Technical Requirements

### Functional Requirements
- Parse JSON hook events from stdin
- Output JSON responses to stdout
- Load configuration from `.claude/hooks.yaml` (project) with fallback to `~/.claude/hooks.yaml` (user)
- Support matchers: tools, extensions, directories, operations, command_match
- Support actions: inject, run, block, block_if_match
- Log all decisions to `~/.claude/logs/cch.log` in JSON Lines format
- Complete processing in <10ms (without validator scripts)
- Execute validator scripts with event JSON on stdin and environment variables
- Interpret validator exit codes: 0=continue, non-zero=block
- Support all Claude Code hook events: PreToolUse, PostToolUse, PermissionRequest, UserPromptSubmit, SessionStart, SessionEnd, PreCompact

### Performance Requirements
- Cold start completes in <5ms (p95), <10ms (p99)
- Rule matching completes in <1ms for up to 100 rules
- All hook events logged with full provenance data
- Zero memory leaks or crashes in 24-hour stress test

## Implementation Notes

This feature is implemented as a Rust binary using:
- **serde** for JSON processing
- **clap** for CLI argument parsing
- **regex** for pattern matching
- **tokio** for async operations

The codebase includes comprehensive tests and follows Rust 2024 edition standards with no unsafe code blocks.
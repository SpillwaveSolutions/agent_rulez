# Changelog

All notable changes to Claude Context Hooks (CCH) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2026-01-28

### Critical Fixes

**v1.0.0 was fundamentally broken for blocking operations.** This release contains essential fixes:

- **Exit Code 2 for Blocking** - v1.0.0 incorrectly used exit code 0 with `continue:false`, which did NOT prevent tool execution. CCH now exits with code 2 when blocking, per Claude Code hook protocol.
- **Event Parsing Fix** - Fixed to correctly parse `hook_event_name` field (not `event_type`) per Claude Code hook event protocol.
- **Config Resolution** - Now uses the event's `cwd` field to locate project-level `hooks.yaml`, fixing incorrect rule matching in some scenarios.

### Added

#### Tooling
- **Taskfile Architecture** - Modular Taskfiles for CLI (`cch_cli/Taskfile.yml`) and UI (`rulez_ui/Taskfile.yml`) with root orchestration
- **Playwright E2E Testing** - Expanded test infrastructure with Page Object Models and CI integration
- **E2E GitHub Workflow** - Automated Playwright tests on push to main/develop

#### RuleZ UI
- Page Object Models for maintainable E2E tests
- Test fixtures for mock configurations and event scenarios
- Enhanced Playwright configuration for CI environments

### Changed

- Root Taskfile now includes subproject Taskfiles via `includes:`
- Orchestrated commands: `task build`, `task test:all`, `task dev`, `task ci-full`
- Playwright config updated with JUnit reporter, video capture on retry, and visual regression settings

### Developer Notes

**Upgrade from v1.0.x is strongly recommended.** Blocking rules were not functioning correctly in v1.0.0-1.0.2.

To verify blocking works correctly after upgrade:
```bash
echo '{"hook_event_name":"PreToolUse","tool_name":"Bash","tool_input":{"command":"git push --force"}}' | cch pre-tool-use
echo $?  # Should output: 2
```

## [1.0.0] - 2026-01-23

### Added

#### Core Features
- **Block Dangerous Operations** - Prevent destructive commands like `git push --force`
- **Inject Context** - Automatically inject context files based on directory patterns
- **Run Custom Validators** - Execute Python/shell scripts to validate tool inputs
- **Permission Explanations** - Provide structured explanations for permission requests

#### CLI Commands
- `cch init` - Create default hooks.yaml with example rules and context files
- `cch install` - Register CCH with Claude Code settings.json
- `cch uninstall` - Remove CCH from Claude Code settings
- `cch validate` - Validate hooks.yaml configuration syntax and schema
- `cch logs` - Query and filter JSON Lines log entries
- `cch explain` - Explain which rules matched an event
- `cch debug` - Simulate events to test rule matching
- `cch repl` - Interactive debug mode for testing rules

#### Configuration
- YAML-based rule configuration in `.claude/hooks.yaml`
- Support for global (`~/.claude/hooks.yaml`) and project-level configs
- Rule matchers: `tools`, `extensions`, `directories`, `operations`, `command_patterns`
- Rule actions: `block`, `block_if_match`, `inject`, `run`

#### Logging & Observability
- JSON Lines format for machine-readable logs
- Structured event details for all tool types
- Response summary logging (continue, reason, context_length)
- Debug mode with raw event and rule evaluation details

#### Performance
- Sub-10ms event processing (<3ms actual)
- Cold start under 5ms p95
- Minimal memory footprint (<50MB resident)

### Technical Details

- **Language**: Rust 2024 edition
- **Runtime**: Tokio async (current_thread flavor)
- **Zero unsafe code**: Memory safety guaranteed by compiler
- **Cross-platform**: Linux (x86_64, aarch64), macOS (x86_64, aarch64), Windows (x86_64)

### Testing

- 64 tests covering all user stories
- Unit tests for core logic
- Integration tests for CLI commands
- Performance tests for latency requirements

## Links

- [Documentation](docs/README.md)
- [User Guide - CLI](docs/USER_GUIDE_CLI.md)
- [User Guide - Skill](docs/USER_GUIDE_SKILL.md)

# Changelog

All notable changes to RuleZ (AI Policy Engine) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.5.0] - 2026-02-10

### Renamed

- **Project renamed from CCH (Claude Context Hooks) to RuleZ** — binary, configs, logs, and all references updated
- Binary: `cch` -> `rulez`
- Log file: `cch.log` -> `rulez.log`
- Release assets: `cch-*` -> `rulez-*`

### Added

#### v1.2 — Inline Content & Conditional Rules
- **`inject_inline`** — Embed context directly in YAML rules without external files
- **`inject_command`** — Generate dynamic context via shell commands at evaluation time
- **`enabled_when`** — Conditional rule activation with evalexpr expressions (e.g., `event_type == "PreToolUse"`)

#### v1.3 — Advanced Matching & Validation
- **`prompt_match`** — Regex intent routing against prompt text with case-insensitive, anchored, AND/OR logic
- **`require_fields` / `field_types`** — Fail-closed field existence and type validation with dot-notation paths
- **`validate_expr`** — Inline evalexpr expressions with `get_field()` / `has_field()` custom functions
- **`inline_script`** — Shell scripts embedded in YAML with configurable timeout protection

#### v1.4 — Stability & Polish
- **JSON Schema validation** — Fail-open schema validation for hook event payloads (<0.1ms overhead via LazyLock pre-compiled validators)
- **Debug CLI UserPromptSubmit support** — Debug command now handles prompt-submit events
- **LRU regex cache** (100 entries) — Replaces unbounded HashMap to prevent memory growth
- **Cross-platform E2E tests** — Path canonicalization for macOS symlinks, CI matrix (ubuntu, macOS, Windows)
- **Tauri CI build pipeline** — E2E gate before desktop builds, multi-platform Tauri packaging

#### RuleZ UI (Desktop App)
- Tauri 2.0 desktop app scaffold with React 18, TypeScript 5.7+, Tailwind CSS 4
- 18 React components, 3 Zustand stores, Monaco YAML editor with schema validation
- Dual-mode architecture (Tauri desktop + web browser fallback)
- Playwright E2E tests with Page Object Model (56 tests)
- `task run-app` command for launching the desktop app

### Fixed

- **Broken pipe in inline scripts** — `Stdio::null()` for stdout/stderr when only checking exit code (Linux CI fix)
- **Zombie process reaping** — Timeout path now calls `child.kill()` + `child.wait()`
- **Stale binary artifacts** — Cleaned up old `cch` binaries after rename
- **E2E test strict mode** — Added `data-testid` attributes for Playwright strict selector compliance
- **Merge conflict resolution** — Fixed 12 files with leftover conflict markers from concurrent PRs

### Changed

- 634 tests passing (up from 64 in v1.0.0)
- <3ms rule processing latency maintained across all new features
- Monorepo structure: `rulez/` (core), `mastering-hooks/` (skill), `rulez-ui/` (desktop app)
- Release workflow updated: asset names now use `rulez-*` prefix

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

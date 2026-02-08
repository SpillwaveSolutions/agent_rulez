# Architecture

**Analysis Date:** 2026-02-06

## Pattern Overview

**Overall:** Dual-layer Policy Engine Architecture

This project implements a high-performance policy engine (CCH) that enforces user-defined YAML rules on Claude Code tool invocations. The architecture separates concerns into:

1. **Rust CLI Layer** - Policy engine that processes events and executes rules
2. **React UI Layer** - Desktop application for visual configuration and testing

**Key Characteristics:**
- **Event-driven** - Responds to Claude Code hook events (stdin/stdout protocol)
- **Configuration-driven** - All behavior defined in user YAML (`.claude/hooks.yaml`)
- **Rule-based** - Matchers determine rule applicability; actions define behavior
- **Audit-trail** - Comprehensive JSON Lines logging for compliance and debugging
- **Async-first** - Tokio-based event processing with timeout protection
- **Zero built-in logic** - No hardcoded blocking/injection; all policies are user-configurable

## Layers

**Policy Engine (Rust CLI):**
- Purpose: Receive hook events from Claude Code, evaluate rules, return allow/block/inject decisions
- Location: `cch_cli/src/`
- Contains: CLI handlers, config loading, rule evaluation, logging infrastructure
- Depends on: Serde (JSON/YAML), Tokio (async), Regex (pattern matching), Tracing (logging)
- Used by: Claude Code hook system (stdin/stdout protocol)

**Configuration Management:**
- Purpose: Load and validate YAML configuration from project and global scopes
- Location: `cch_cli/src/config.rs`
- Contains: Config loading logic, fallback hierarchy (project → global → defaults)
- Depends on: Serde_yaml, file system operations
- Used by: Main event processing flow and CLI commands

**Rule Evaluation Engine:**
- Purpose: Match events against rules and determine outcomes (allow/block/inject/audit)
- Location: `cch_cli/src/hooks.rs` (primary) and `cch_cli/src/models.rs` (types)
- Contains: Event matching, action execution, context injection, validator scripts
- Depends on: Regex patterns, timeout management, logging
- Used by: Main event processor

**Logging & Audit Trail:**
- Purpose: Record all policy decisions in JSON Lines format for compliance
- Location: `cch_cli/src/logging.rs`
- Contains: Logger implementation, log query interface, filtering logic
- Depends on: Chrono (timestamps), Serde (JSON)
- Used by: Event processor and logs command

**UI Layer (React + Tauri):**
- Purpose: Provide visual interface for editing rules, simulating events, and debugging
- Location: `rulez_ui/src/`
- Contains: React components, state management, editor integration, Tauri IPC
- Depends on: React 18, Zustand, Monaco Editor, Tauri 2.0
- Used by: End users for rule configuration and testing

## Data Flow

**Hook Event Processing (Primary Flow):**

1. Claude Code detects tool invocation → sends JSON event to `cch` binary on stdin
2. `main.rs` parses event as `models::Event`
3. Config loaded from project root (traverses up to find `.claude/hooks.yaml`)
4. `hooks::process_event()` calls `evaluate_rules()`:
   - Iterates through enabled rules
   - For each rule: evaluates matchers (event type, tool name, patterns)
   - Collects all matching rules
5. First matching rule determines outcome:
   - If action is `block` → returns `Response { continue: false }`
   - If action is `inject` → loads context files, returns `Response { continue: true, context: ... }`
   - If action is `validate` → runs validator script, returns based on exit code
   - If policy mode is `warn` → injects warning context instead of blocking
   - If policy mode is `audit` → logs only, always allows
6. `LogEntry` created with event details, matched rules, and decision
7. Response written to stdout with timing information
8. If `continue: false`, exit with code 2 (signals blocking)

**Configuration Loading Hierarchy:**

1. Try project-specific: `.claude/hooks.yaml` (from event's cwd)
2. Try global: `~/.claude/hooks.yaml`
3. Use built-in defaults if no files found

**State Management (UI):**

- `configStore` - Holds loaded config files and open file state
- `editorStore` - Tracks cursor position, selection, validation errors/warnings
- `uiStore` - Manages theme preference

**Data Flow (UI):**

1. App loads config files via Tauri IPC (`config::list_config_files`)
2. User selects file → reads via IPC (`config::read_config`)
3. YAML content displayed in Monaco Editor with validation
4. User creates test event in simulator form
5. Simulator sends to CCH binary via Tauri (`debug::run_debug`)
6. Response shown with rule matches and evaluation trace

## Key Abstractions

**Event:**
- Purpose: Represents a Claude Code hook invocation (tool use, permission request, session start)
- Examples: `cch_cli/src/models.rs` lines 200+
- Pattern: Serde JSON serialization with typed fields for tool name, arguments, session context

**Rule:**
- Purpose: Configurable policy with matchers and actions
- Examples: `cch_cli/src/models.rs` - defines `Rule` struct with name, enabled flag, matchers, actions, governance metadata
- Pattern: YAML structure with type discriminator for action (block/inject/validate)

**Matcher:**
- Purpose: Determines if a rule applies to an event
- Examples: Event type matchers, tool name matchers, regex pattern matchers on command/path
- Pattern: Recursive matching logic in `hooks.rs` that short-circuits on first non-match

**Action:**
- Purpose: Defines what happens when rule matches
- Examples: `block`, `inject` (with file list), `validate` (with script path), `warn`, `audit`
- Pattern: Enum in models with associated data (file paths, scripts)

**Governance Metadata:**
- Purpose: Tracks rule authority, priority, and trust level (Phase 2 feature)
- Examples: `PolicyMode` (enforce/warn/audit), `Priority`, `TrustLevel`
- Pattern: Serde enum with display impls for logging

**LogEntry:**
- Purpose: Structured audit record for compliance and debugging
- Examples: `cch_cli/src/models.rs` lines 1000+
- Pattern: Flat JSON structure with optional nested fields (event_details, response_summary, rule_evaluations)

## Entry Points

**CLI - Hook Event Processing:**
- Location: `cch_cli/src/main.rs::main()` (no subcommand case)
- Triggers: Claude Code calls `cch < event.json`
- Responsibilities: Read stdin, parse event, load config, evaluate rules, return response

**CLI - Debug Command:**
- Location: `cch_cli/src/main.rs::main()` → `Commands::Debug`
- Triggers: User runs `cch debug PreToolUse --tool Bash --command "rm -rf"`
- Responsibilities: Simulate event with given parameters, show rule matches and evaluation trace

**CLI - Init Command:**
- Location: `cch_cli/src/cli/init.rs`
- Triggers: User runs `cch init` in new project
- Responsibilities: Create `.claude/hooks.yaml` with template, optionally add example files

**CLI - Install Command:**
- Location: `cch_cli/src/cli/install.rs`
- Triggers: User runs `cch install` to register hook with Claude Code
- Responsibilities: Add/update `.claude/settings.json` to call `cch` on hook events

**UI - App Entry:**
- Location: `rulez_ui/src/main.tsx` → `rulez_ui/src/App.tsx`
- Triggers: User launches desktop app or opens browser to dev server
- Responsibilities: Initialize React app, load initial config, mount AppShell component

**UI - AppShell:**
- Location: `rulez_ui/src/components/layout/AppShell.tsx`
- Triggers: App render
- Responsibilities: Set up main layout (header, sidebar, editor, right panel), coordinate component interactions

## Error Handling

**Strategy:** Layered with fail-open default

**Patterns:**

1. **Parsing Errors** - Invalid JSON/YAML → Logged, continue: false returned (blocks operation)
2. **Rule Evaluation Errors** - Regex fails, file not found → Logged, rule skipped if `fail_open: true`
3. **Validator Script Errors** - Script times out or crashes → Treated as failed validation, logged
4. **Config Loading Errors** - File not found → Fall back to next location or use defaults
5. **Logging Errors** - Log file write fails → Warning logged, doesn't affect decision
6. **Async Timeouts** - Script execution exceeds limit → Defaults to block (safe default), error logged

**Error Types:**
- `anyhow::Result` wrapper for propagation
- `thiserror` for CCH-specific errors (in models)
- Tracing/logging for observability
- Exit code 2 for decision blocking, exit code 1 for errors

## Cross-Cutting Concerns

**Logging:** Tracing framework with environment filter; structured JSON audit trail in `~/.claude/logs/cch.log`

**Validation:**
- Config validation on load (all rules, all matchers)
- Rule evaluation validates event structure
- Validator scripts provide user-defined validation

**Authentication:**
- File-based config in user home directory
- Trust levels track validator script provenance (local/verified/untrusted)
- No authentication between Claude Code and CCH (assumes local execution)

**Performance:**
- Regex compilation (cached at rule load)
- Async I/O with timeout protection (default 5s for scripts)
- Single-threaded Tokio runtime (minimizes overhead)
- JSON Lines logging (streaming, not buffered)

---

*Architecture analysis: 2026-02-06*

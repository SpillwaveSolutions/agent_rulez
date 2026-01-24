# Project Constitution

## Project Vision

### Strategic Positioning
CCH (Claude Context Hooks) is evolving from a "powerful local hook system" into a **first-class, auditable, local AI policy engine** suitable for real organizational governance.

The project encompasses:
1. **CCH Core** (v1.0.0 Released) - Rust-based policy engine binary
2. **Phase 2 Governance** (Planned) - Policy modes, metadata, priorities, enterprise features
3. **RuleZ UI** (Planned) - Tauri desktop application for visual configuration

### Design Philosophy
**LLMs do not enforce policy. LLMs are subject to policy.**

- CCH is the policy authority
- Skills are policy authors
- Claude is policy-constrained execution

This positions CCH as comparable to:
- OPA (but human-readable)
- Terraform Sentinel (but local)
- Kubernetes admission controllers (but for agents)

---

## Git Workflow Principles

### Feature Branch Requirement
- **NEVER commit directly to `main`** - This is a non-negotiable principle
- All feature work MUST be done in a dedicated feature branch
- Pull Requests are REQUIRED for all changes to `main`
- Code review via PR ensures quality and knowledge sharing

### Branch Naming Convention
- Features: `feature/<feature-name>` (e.g., `feature/add-debug-command`)
- Bugfixes: `fix/<bug-description>` (e.g., `fix/config-parsing-error`)
- Documentation: `docs/<doc-topic>` (e.g., `docs/update-readme`)
- Releases: `release/<version>` (e.g., `release/v1.0.0`)

### PR Workflow
1. Create feature branch from `main`
2. Implement changes with atomic, conventional commits
3. **Run all pre-commit checks locally** (see below)
4. Push branch and create Pull Request
5. Request review and address feedback
6. Merge via GitHub (squash or merge commit as appropriate)
7. Delete feature branch after merge

### Pre-Commit Checks (MANDATORY)

**For CCH Core (Rust):**
```bash
cd cch_cli
cargo fmt --check                                          # Formatting
cargo clippy --all-targets --all-features -- -D warnings   # Linting
cargo test                                                 # All tests
```

**For RuleZ UI (TypeScript/React):**
```bash
cd rulez_ui
bun run lint                                               # ESLint
bun run typecheck                                          # TypeScript
bun run test                                               # Bun tests
```

**NEVER commit if any check fails.** This is non-negotiable. CI will reject PRs that fail these checks, wasting time and creating noise.

Quick one-liner for CCH Core:
```bash
cd cch_cli && cargo fmt && cargo clippy --all-targets --all-features -- -D warnings && cargo test
```

### Rationale
Direct commits to `main` bypass code review, risk introducing bugs, and make it difficult to revert changes. Feature branches enable parallel development, clean history, and proper CI/CD validation before merge.

---

## Core Principles

### Safety First
- **Zero unsafe code blocks**: All Rust code must be memory-safe using Rust's ownership system
- **Fail-open design**: System continues operating even when individual components fail
- **Comprehensive error handling**: All error paths must be handled gracefully
- **No network access**: CCH operates purely locally for security
- **No telemetry**: User privacy is paramount; no analytics or data collection

### Performance Critical
- **Sub-10ms processing**: Hook events must be processed in under 10ms
- **Minimal dependencies**: Only essential crates to minimize binary size and startup time
- **Async efficiency**: Use tokio with minimal features for optimal performance
- **UI responsiveness**: RuleZ UI must maintain 60fps (< 16ms input latency)
- **Fast startup**: CCH cold start <5ms, RuleZ UI launch <2s

### Configuration-Driven Architecture
- **YAML-based rules**: All behavior defined by user configuration, not hardcoded logic
- **Flexible matching**: Support tools, extensions, directories, operations, and command patterns
- **Pluggable actions**: Support inject, run, block, and block_if_match actions
- **Backward compatible**: New features (metadata, modes, priority) are always optional

### Observability & Debugging
- **Complete audit trail**: All decisions logged in JSON Lines format
- **Debug configuration**: Optional detailed logging for troubleshooting
- **CLI tools**: Commands for log querying and rule explanation
- **Visual debugging**: RuleZ UI provides simulation and trace visualization

## Technology Choices

### CCH Core (Rust Binary)

**Language & Runtime:**
- **Rust 2024 edition**: Modern Rust with stable features
- **No unsafe code**: Memory safety guaranteed by compiler
- **Tokio async runtime**: For efficient async operations

**Core Dependencies:**
- **serde**: JSON/YAML serialization (no other serialization crates)
- **clap**: CLI argument parsing (derive API)
- **regex**: Pattern matching for rule conditions
- **tokio**: Async runtime (minimal features for performance)
- **tracing**: Structured logging (not println!)
- **chrono**: Time handling with serde support
- **dirs**: Cross-platform directory handling

**Project Structure:**
- **Workspace layout**: Separate binary crate for clean separation
- **Module organization**: Clear separation of concerns (cli, config, hooks, logging, models)
- **Test organization**: Unit, integration, and contract tests

### RuleZ UI (Desktop Application)

**Frontend Stack:**
- **Runtime**: Bun (all TypeScript/React operations)
- **Framework**: React 18 + TypeScript
- **Styling**: Tailwind CSS 4
- **Editor**: Monaco Editor + monaco-yaml
- **State**: Zustand + TanStack Query

**Desktop Framework:**
- **Tauri 2.0**: Rust backend with WebView frontend
- **IPC**: Type-safe command invocation between frontend and backend
- **File I/O**: Secure filesystem access via Tauri APIs

**Testing:**
- **Unit Tests**: Bun test (80%+ coverage for utilities)
- **E2E Tests**: Playwright (critical user flows)

## Coding Standards

### Rust (CCH Core)

**Error Handling:**
- Use `anyhow::Result` for application errors
- Use `thiserror` for library crate error types
- Log errors with context, don't panic

**Async Patterns:**
- Use `tokio::main` with current_thread flavor for minimal overhead
- Prefer async functions over blocking operations
- Use tokio::process for external command execution

**Configuration:**
- Load from `.claude/hooks.yaml` (project) with fallback to `~/.claude/hooks.yaml` (user)
- Validate configuration on startup
- Support environment variable overrides

**Logging:**
- Use tracing macros (info!, error!, warn!, debug!)
- Structure logs as JSON for machine readability
- Include session_id and event context in all log entries

### TypeScript (RuleZ UI)

**Type Safety:**
- Strict TypeScript configuration
- No `any` types without explicit justification
- Prefer interfaces over type aliases for objects

**React Patterns:**
- Functional components with hooks
- Zustand for global state management
- TanStack Query for async operations

**Styling:**
- Tailwind CSS utility classes
- Dark/light theme support via CSS variables
- System preference detection with manual override

## Architectural Decisions

### Event Processing Pipeline (CCH Core)
1. Parse JSON event from stdin
2. Load and validate configuration
3. Match rules against event
4. Execute matching rule actions
5. Log decision with full provenance
6. Output JSON response to stdout

### Rule Matching Logic
- AND conditions within matchers (all must match)
- OR across rules (first matching rule wins)
- Actions executed in rule definition order
- Block actions terminate processing immediately

### Phase 2 Governance Extensions

**Policy Modes** (enforce | warn | audit):
- `enforce` (default): Normal blocking behavior
- `warn`: Never blocks, injects warning context
- `audit`: No injection, no blocking, logs only

**Rule Priority:**
- Higher numbers run first (default = 0)
- Enables explicit control over policy ordering
- Prevents emergent policy bugs

**Rule Metadata (Provenance):**
- `author`, `created_by`, `reason`, `confidence`
- `last_reviewed`, `ticket`, `tags`
- Included in logs and debug output for auditability

**Conflict Resolution:**
- enforce + warn = enforce wins
- audit + enforce = enforce wins
- Multiple enforce = highest priority wins

### Security Model
- No network access (pure local processing)
- File system access limited to configuration and log directories
- External script execution with timeout and controlled environment
- Input validation on all event data
- RuleZ UI: No arbitrary code execution, respects filesystem permissions

## Quality Gates

### CCH Core Performance
- Cold start: <5ms p95, <10ms p99
- Rule matching: <1ms for 100 rules
- Memory usage: <50MB resident
- No memory leaks in 24-hour stress test

### RuleZ UI Performance
- App launch: <2 seconds
- File load (10KB YAML): <100ms
- Validation response: <200ms
- Editor input latency: <16ms (60fps)
- Memory usage (idle): <150MB

### Reliability
- Zero crashes in normal operation
- Graceful degradation on configuration errors
- Fail-open behavior for non-critical failures
- Comprehensive test coverage
- RuleZ UI: Graceful handling of missing configs and CCH binary

### Maintainability
- Clear module boundaries
- Comprehensive documentation
- Automated testing and linting
- Simple deployment (single binary for CCH, installers for RuleZ UI)

---

## Platform Support

### CCH Core
| Platform | Architecture | Status |
|----------|--------------|--------|
| Linux | x86_64, aarch64 | Supported |
| macOS | Intel, Apple Silicon | Supported |
| Windows | x86_64 | Supported |

### RuleZ UI
| Platform | Format | Status |
|----------|--------|--------|
| macOS | .dmg, .app | Planned |
| Windows | .msi, .exe | Planned |
| Linux | .deb, .AppImage | Planned |

---

## Roadmap Summary

### v1.0.0 (Released)
- Core policy engine with blocking, injection, validation
- CLI commands: init, install, uninstall, validate, logs, explain, debug, repl
- 64+ tests, comprehensive logging

### Phase 2 Governance (Planned)
- Policy modes (enforce/warn/audit)
- Rule priority and metadata
- Enhanced `cch explain rule` command
- Trust levels for validators (informational)
- Policy packs concept (future-proof)

### RuleZ UI (Planned)
- Visual YAML editor with Monaco
- Real-time schema validation
- Debug simulator for testing rules
- Multi-file support (global + project configs)
- Dark/light theme support
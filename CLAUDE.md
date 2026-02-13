# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**RuleZ** is a high-performance AI policy engine for development workflows. It intercepts Claude Code tool invocations via hooks and applies user-defined YAML rules.

**This is a monorepo with three components:**

| Component | Location | Purpose | Priority |
|-----------|----------|---------|----------|
| **RuleZ Core** | `rulez/` | Rust policy engine binary | P1 - Primary |
| **Mastering Hooks** | `mastering-hooks/` | Claude Code skill | P2 - Secondary |
| **RuleZ UI** | `rulez-ui/` | Tauri desktop app | P3 - Tertiary |

## Repository Structure

```
agent_rulez/
├── rulez/                 # Core binary (Rust)
│   ├── src/
│   │   ├── cli/          # CLI subcommands
│   │   ├── config.rs     # Config loading
│   │   ├── hooks.rs      # Rule evaluation engine
│   │   ├── logging.rs    # Audit trail
│   │   └── models.rs     # Type definitions
│   └── tests/            # Integration tests
├── mastering-hooks/       # Claude Code skill
│   ├── SKILL.md          # Skill definition
│   ├── references/       # Documentation
│   └── assets/           # Templates
├── rulez-ui/             # Desktop app (Tauri)
│   ├── src/              # React frontend
│   └── src-tauri/        # Rust backend
├── docs/                 # Documentation
├── .planning/            # GSD workflow artifacts
└── .speckit/             # SDD artifacts (reference)
```

## Git Workflow Requirements

**CRITICAL: Always use feature branches for all work.**

- **NEVER commit directly to `main`** - All feature work MUST be done in a feature branch
- Create a feature branch before starting any work: `git checkout -b feature/<feature-name>`
- Push the feature branch and create a Pull Request for review
- Only merge to `main` via PR after review

**Branch Naming Convention:**
- Features: `feature/<feature-name>` (e.g., `feature/add-debug-command`)
- Bugfixes: `fix/<bug-description>` (e.g., `fix/config-parsing-error`)
- Documentation: `docs/<doc-topic>` (e.g., `docs/update-readme`)

## Quick Start

```bash
# Build everything
task build

# Run tests
task test

# Build and run RuleZ CLI
task run -- --help

# Start RuleZ UI development
task run-app
```

## RuleZ Core Commands

```bash
rulez init              # Create default hooks.yaml
rulez install           # Register with Claude Code
rulez uninstall         # Remove from Claude Code
rulez debug <event>     # Simulate events to test rules
rulez validate          # Validate configuration
rulez logs              # Query audit logs
rulez explain           # Explain why rules fired
rulez repl              # Interactive debug mode
```

## Active Technologies

- **Core Binary:** Rust 2021, tokio (async), serde (JSON/YAML), clap (CLI), regex
- **Desktop App:** Tauri 2.0, React 18, TypeScript 5.7+, Tailwind CSS 4
- **Skill:** SKILL.md format, references/, assets/

## Configuration

- Global config: `~/.claude/hooks.yaml`
- Project config: `.claude/hooks.yaml`
- Logs: `~/.claude/logs/rulez.log`

## Pre-Push Checklist

**CRITICAL: Always run the FULL CI pipeline locally before pushing or creating PRs. ALL steps must pass.**

```bash
# 1. Format check
cargo fmt --all --check

# 2. Clippy (CI uses -D warnings — all warnings are errors)
cargo clippy --all-targets --all-features --workspace -- -D warnings

# 3. Full test suite (remove stale binaries first if binary was renamed)
cargo test --tests --all-features --workspace

# 4. Code coverage (runs ALL tests including e2e — catches pipe/process bugs)
cargo llvm-cov --all-features --workspace --no-report
```

**Why this matters:**
- Tests may pass locally due to stale build artifacts (e.g., old binary names in `target/`) that don't exist on CI.
- The code coverage step (`cargo llvm-cov`) runs tests with instrumentation that can surface pipe, process, and concurrency bugs that `cargo test` alone does not.
- If `cargo llvm-cov` is not installed: `cargo install cargo-llvm-cov`
- **Do NOT push if any step fails.**

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Configuration error |
| 2 | Validation error |
| 3 | Runtime error |

# RuleZ (AI Policy Engine)

## What This Is

**RuleZ** is a high-performance, auditable, local AI policy engine for Claude Code and other AI development tools. It intercepts tool invocations via hooks and applies user-defined YAML rules to block dangerous operations, inject helpful context, and maintain comprehensive audit trails.

## Monorepo Components

| Component | Location | Priority | Status |
|-----------|----------|----------|--------|
| **RuleZ Core** | `rulez/` | P1 - Primary | v1.1.0 Released |
| **Mastering Hooks** | `mastering-hooks/` | P2 - Secondary | Complete (skill) |
| **RuleZ UI** | `rulez-ui/` | P3 - Tertiary | M1 Complete |

## Core Value

**LLMs do not enforce policy. LLMs are subject to policy.**

RuleZ positions itself as comparable to:
- OPA (but human-readable)
- Terraform Sentinel (but local)
- Kubernetes admission controllers (but for AI agents)

## Design Philosophy

- **Zero unsafe Rust code** - Memory safety guaranteed
- **Sub-10ms processing** - Hook events processed in under 10ms
- **Configuration-driven** - All behavior defined by user YAML, not hardcoded logic
- **Complete audit trail** - All decisions logged in JSON Lines format
- **No network access** - Pure local processing for security
- **No telemetry** - User privacy is paramount

## Current State

### RuleZ Core (v1.2.0)
- Policy engine with blocking, injection, validation
- CLI: init, install, uninstall, validate, logs, explain, debug, repl
- 245 tests, <3ms latency, comprehensive logging
- Phase 2 Governance: modes (enforce/warn/audit), priority, metadata
- **v1.2 Features:**
  - `inject_inline` - Embed context directly in YAML
  - `inject_command` - Dynamic context via shell commands
  - `enabled_when` - Conditional rule activation with expressions

### Mastering Hooks (Complete)
- Claude Code skill for RuleZ mastery
- References: schema, CLI commands, patterns, troubleshooting
- Future: Convert to plugin format

### RuleZ UI (In Progress)
- Tauri 2.0 desktop app
- M1 (scaffold) complete, M2-M8 pending
- Lower priority than core binary enhancements

## Technology Stack

### RuleZ Core (Rust)
- Rust 2021 edition, tokio async runtime
- serde (JSON/YAML), clap (CLI), regex (patterns)
- tracing (structured logging), chrono (time)

### RuleZ UI (Desktop)
- Tauri 2.0 (Rust backend + WebView)
- React 18 + TypeScript 5.7+
- Monaco Editor + monaco-yaml
- Tailwind CSS 4, Zustand, Bun

## Quality Gates

- Cold start: <5ms p95
- Rule matching: <1ms for 100 rules
- Memory: <50MB resident
- RuleZ UI launch: <2 seconds
- Editor input latency: <16ms (60fps)

## Git Workflow

- `main` - Production-ready, fully validated
- `develop` - Integration branch, fast CI
- `feature/*` - Short-lived working branches
- Never commit directly to main or develop

---

*Last updated: 2026-02-07 after v1.2 milestone*
*Reorganized as monorepo on 2026-02-06*
*Renamed from CCH to RuleZ*

# RuleZ (AI Policy Engine)

## What This Is

**RuleZ** is a high-performance, auditable, local AI policy engine for Claude Code and other AI development tools. It intercepts tool invocations via hooks and applies user-defined YAML rules to block dangerous operations, inject helpful context, validate tool inputs, and maintain comprehensive audit trails.

## Monorepo Components

| Component | Location | Priority | Status |
|-----------|----------|----------|--------|
| **RuleZ Core** | `rulez/` | P1 - Primary | v1.3.0 Shipped |
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

### RuleZ Core (v1.3.0)
- Policy engine with blocking, injection, validation, inline scripting
- CLI: init, install, uninstall, validate, logs, explain, debug, repl
- 605 tests, <3ms latency, comprehensive logging
- 22,339 LOC Rust
- **v1.2 Features:**
  - `inject_inline` - Embed context directly in YAML
  - `inject_command` - Dynamic context via shell commands
  - `enabled_when` - Conditional rule activation with expressions
- **v1.3 Features:**
  - `prompt_match` - Regex intent routing with case-insensitive, anchored, AND/OR logic
  - `require_fields` / `field_types` - Fail-closed field validation with dot-notation paths
  - `validate_expr` - Inline evalexpr expressions with get_field() / has_field()
  - `inline_script` - Shell scripts in YAML with timeout protection

### Mastering Hooks (Complete)
- Claude Code skill for RuleZ mastery
- References: schema, CLI commands, patterns, troubleshooting
- Future: Convert to plugin format

### RuleZ UI (In Progress)
- Tauri 2.0 desktop app
- M1 (scaffold) complete, M2-M8 pending
- Lower priority than core binary enhancements

## Requirements

### Validated

- ✓ `inject_inline` — Embed context directly in YAML — v1.2
- ✓ `inject_command` — Dynamic context via shell commands — v1.2
- ✓ `enabled_when` — Conditional rule activation with expressions — v1.2
- ✓ PROMPT-01: Regex pattern matching against prompt text — v1.3
- ✓ PROMPT-02: Case-insensitive matching — v1.3
- ✓ PROMPT-03: Multiple patterns with any/all logic — v1.3
- ✓ PROMPT-04: Anchored pattern matching — v1.3
- ✓ PROMPT-05: Script-based prompt matching — v1.3
- ✓ FIELD-01: Required field existence validation — v1.3
- ✓ FIELD-02: Fail-closed blocking on missing fields — v1.3
- ✓ FIELD-03: Nested field paths with dot notation — v1.3
- ✓ FIELD-04: Field type validation — v1.3
- ✓ SCRIPT-01: Inline evalexpr expressions in YAML — v1.3
- ✓ SCRIPT-02: Custom functions (get_field, has_field) — v1.3
- ✓ SCRIPT-03: Boolean validation semantics — v1.3
- ✓ SCRIPT-04: Inline shell scripts — v1.3
- ✓ SCRIPT-05: Timeout protection for scripts — v1.3
- ✓ SCRIPT-06: Config-time script validation — v1.3

### Active

(None — ready for next milestone requirements)

### Out of Scope

| Feature | Reason |
|---------|--------|
| NLP/semantic prompt matching | Performance impact (50+ MB, 100ms+ latency) |
| External state in validators | Security risk, use inject_command instead |
| Async validators | Breaks sub-10ms guarantee |
| Full scripting language (Rhai/Lua) | 500 KB binary impact, complexity, 7+ deps |
| Mobile app | Web-first approach |

## Technology Stack

### RuleZ Core (Rust)
- Rust 2021 edition, tokio async runtime
- serde (JSON/YAML), clap (CLI), regex (patterns)
- evalexpr (expressions + custom functions)
- tracing (structured logging), chrono (time)

### RuleZ UI (Desktop)
- Tauri 2.0 (Rust backend + WebView)
- React 18 + TypeScript 5.7+
- Monaco Editor + monaco-yaml
- Tailwind CSS 4, Zustand, Bun

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| evalexpr 13.1 for expressions | Lightweight, proven, zero deps | ✓ Good |
| Fail-closed semantics throughout | Safety-first for policy engine | ✓ Good |
| serde untagged enum for PromptMatch | Flexible YAML syntax | ✓ Good |
| once_cell for regex caching | Zero-allocation repeat matches | ✓ Good |
| Dot notation for field paths | User-friendly, RFC 6901 conversion | ✓ Good |
| validate_expr / inline_script mutual exclusion | Simpler mental model | ✓ Good |
| Defer script sandboxing to v1.4 | Cross-platform complexity | ⚠️ Revisit |
| Unbounded regex cache | Short-lived CLI, low risk | ⚠️ Revisit |

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

*Last updated: 2026-02-10 after v1.3 milestone complete*
*Reorganized as monorepo on 2026-02-06*
*Renamed from CCH to RuleZ*

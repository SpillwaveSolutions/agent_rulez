# RuleZ (AI Policy Engine)

## What This Is

**RuleZ** is a high-performance, auditable, local AI policy engine for Claude Code and other AI development tools. It intercepts tool invocations via hooks and applies user-defined YAML rules to block dangerous operations, inject helpful context, validate tool inputs, and maintain comprehensive audit trails.

## Monorepo Components

| Component | Location | Priority | Status |
|-----------|----------|----------|--------|
| **RuleZ Core** | `rulez/` | P1 - Primary | v1.4 Shipped |
| **Mastering Hooks** | `mastering-hooks/` | P2 - Secondary | Complete (skill) |
| **RuleZ UI** | `rulez-ui/` | P1 - Primary (v1.5) | M1 Scaffold Complete, v1.5 Active |

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

### RuleZ Core (v2.2.2)
- Policy engine with blocking, injection, validation, inline scripting, schema validation
- CLI: init, install, uninstall, validate, logs, explain, debug, repl, test, lint, upgrade
- Multi-CLI support: Claude Code, Gemini, Copilot, OpenCode (install/doctor commands)
- Parallel rule evaluation, config caching, globset matching
- E2E test harness across 5 CLIs (Claude Code, Gemini, Copilot, OpenCode, Codex)

### Mastering Hooks (Complete)
- Claude Code skill for RuleZ mastery
- References: schema, 19+ CLI commands documented, patterns, troubleshooting
- Covers subagent hook patterns

### RuleZ UI (Complete)
- Tauri 2.0 desktop app with Monaco YAML editor, log viewer, config management
- ConfigDiffView with Monaco DiffEditor for config comparison
- Debug simulator, onboarding wizard, settings panel
- Playwright E2E tests, CI builds on ubuntu/macOS/Windows

### Release Skill (Complete)
- Renamed from `release-cch` to `release-rulez` (v2.2.1)
- Automated release workflow with preflight checks, changelog, and verification

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

- ✓ REQ-SCHEMA-01..06: JSON Schema validation for hook event payloads — v1.4
- ✓ REQ-DEBUG-01..05: Debug CLI enhancements (UserPromptSubmit, LRU cache, state isolation) — v1.4
- ✓ REQ-E2E-01..05: E2E test stabilization (canonical paths, symlink resolution, explicit cleanup) — v1.4
- ✓ REQ-TAURI-01..06: Tauri CI build pipeline with E2E gate — v1.4
- ✓ REQ-PERF-01..02: Performance quality gates (<0.1ms schema validation) — v1.4
- ✓ REQ-COMPAT-01..02: Cross-platform compatibility (CI matrix) — v1.4

- ✓ CLIDOC-01..03: CLI reference docs updated with all commands and flags — v2.2.2
- ✓ GUIDE-01..03: Per-CLI usage guides (Claude Code, Gemini, OpenCode) — v2.2.2
- ✓ FEAT-01..03: Feature documentation (external logging, lint, test) — v2.2.2
- ✓ AUDIT-01..02: Accuracy audit against source code and --help output — v2.2.2

### Active

- [ ] PROFILE-01: Runtime profiles define per-platform conventions (dirs, separators, tool style)
- [ ] PROFILE-02: Skill discovery scans .claude/skills/, .claude/commands/, and extra dirs
- [ ] PROFILE-03: Extra skill sources (mastering-hooks at repo root) discovered automatically
- [ ] XFORM-01: Tool names converted from PascalCase to runtime conventions
- [ ] XFORM-02: Path references rewritten (~/.claude/ -> runtime equivalents)
- [ ] XFORM-03: Command filenames flattened (dot to hyphen) with cross-reference rewriting
- [ ] XFORM-04: YAML frontmatter converted (allowed-tools -> tools, color hex, strip unsupported)
- [ ] XFORM-05: MCP tools excluded for Gemini, preserved for OpenCode
- [ ] CLI-01: `rulez skills install --runtime <rt>` installs to target runtime
- [ ] CLI-02: `rulez skills install --dry-run` previews without writing
- [ ] CLI-03: `rulez skills clean --runtime <rt>` removes generated files
- [ ] CONFIG-01: Auto-update GEMINI.md skill registry with marker sections
- [ ] CONFIG-02: Auto-generate AGENTS.md for Codex with skill registry
- [ ] CONFIG-03: Mastering-hooks platform references rewritten context-aware
- [ ] DX-01: `rulez skills status` shows human-readable dates and freshness
- [ ] DX-02: `rulez skills diff --runtime <rt>` shows colored diff of changes
- [ ] DX-03: `rulez skills sync` installs to all detected runtimes
- [ ] DX-04: Colorized output with progress indicators

## Current Milestone: v2.3.0 Multi-Runtime Skill Portability

**Goal:** Build an installer-based conversion pipeline that transforms canonical Claude Code skills into runtime-specific installations. Author once in `.claude/`, convert at install time, run everywhere.

**Target features:**
- Runtime profiles and skill discovery (Phase 34 — DONE)
- Content transformation engine with 6 transform types (Phase 35 — DONE)
- CLI integration with file writer (Phase 36 — DONE)
- Config file generation for GEMINI.md, AGENTS.md (Phase 37)
- DX polish: status, diff, sync, clean (Phase 38)

**Plan:** `docs/plans/multi-runtime-skill-portability.md`

## Shipped: v2.2.2 Documentation Audit & Multi-CLI Guides (2026-03-17)

All documentation audited against source code, per-CLI usage guides created for Claude Code/Gemini/OpenCode, feature docs for external logging/lint/test. 11/11 requirements satisfied, 4 phases, 8 plans.

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
| Defer script sandboxing to v1.5+ | Cross-platform complexity, not in v1.4 scope | ⚠️ Revisit |
| LRU regex cache (100 entries) | Replaced unbounded HashMap, prevents memory growth | ✓ Good |
| Fail-open schema validation | Log warnings but continue processing for robustness | ✓ Good |
| LazyLock pre-compiled validators | <0.1ms validation overhead at runtime | ✓ Good |
| ubuntu-22.04 for Tauri builds | webkit2gtk-4.1 requirement, ubuntu-latest may break | ✓ Good |
| E2E gate before Tauri builds | Fast feedback (2-3min) prevents expensive failed builds | ✓ Good |
| Hardcoded Rust transforms, not YAML-configurable | 4 well-known runtimes + Custom variant covers long tail | — Pending |
| Clean-install writer (rm + recreate) | Prevents orphan files across versions, proven in GSD | — Pending |
| `rulez skills` subcommand family, not extending `rulez install` | Hook registration and skill distribution are orthogonal | — Pending |

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

*Last updated: 2026-03-16 after v2.3.0 milestone start*
*Reorganized as monorepo on 2026-02-06*
*Renamed from CCH to RuleZ*

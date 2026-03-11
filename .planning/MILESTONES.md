# Project Milestones: RuleZ (AI Policy Engine)

## v1.2 P2 Features (Shipped: 2026-02-07)

**Delivered:** Advanced injection and conditional rule features for dynamic context generation and environment-aware rule activation.

**Phases completed:** 1-3 (6 plans total)

**Key accomplishments:**

- Added `inject_inline` for embedding markdown context directly in YAML rules
- Added `inject_command` for dynamic context via shell command execution
- Added `enabled_when` for conditional rule activation with expression evaluation
- Integrated evalexpr 13.1 for lightweight expression parsing
- Established execution precedence: inject_inline > inject_command > inject > run
- Implemented fail-closed semantics for invalid expressions

**Stats:**

- 42 files created/modified
- 6,098 lines of Rust (src/)
- 3 phases, 6 plans
- 245 tests passing (up from ~75)
- 17 days from start to ship

**Git range:** `feat(01-01)` → `docs(03-03)`

**What's next:** RuleZ v1.3 with prompt_match, require_fields, or inline script blocks

---

## v1.3 Advanced Matching & Validation (Shipped: 2026-02-10)

**Delivered:** Intent-based routing, required field validation, and inline validation logic for more powerful rule authoring.

**Phases completed:** 4-6 (10 plans total)

**Key accomplishments:**

- Added `prompt_match` for regex-based intent routing with case-insensitive, anchored, and AND/OR logic
- Added `require_fields` and `field_types` for fail-closed field validation with dot-notation paths
- Added `validate_expr` for inline evalexpr expressions with custom functions (get_field, has_field)
- Added `inline_script` for YAML-embedded shell scripts with timeout protection
- Zero new dependencies — extended evalexpr with custom functions, reused regex crate
- Comprehensive test coverage: 262 tests (247 unit + 15 integration) across all 15 requirements

**Stats:**

- 31 files created/modified
- 22,339 lines of Rust (src/)
- 3 phases, 10 plans
- 262 tests passing (up from 245)
- 2 days from start to ship (2026-02-09 → 2026-02-10)

**Git range:** `337f6a7` (feat 04-01) → `4fb39c7` (audit)

**Tech debt accepted:**
- Debug CLI can't simulate UserPromptSubmit (medium)
- Regex cache unbounded (low)
- Inline scripts unsandboxed (deferred to v1.4)

**What's next:** v1.4 with script sandboxing, JSON Schema validation, and debug CLI improvements

---


## v1.4 Stability & Polish (Shipped: 2026-02-10)

**Delivered:** Infrastructure hardening — JSON Schema validation, debug CLI parity, cross-platform E2E reliability, and Tauri 2.0 CI automation.

**Phases completed:** 7-10 (9 plans total)

**Key accomplishments:**

- Added JSON Schema validation with fail-open mode (<0.1ms overhead via LazyLock pre-compiled validators)
- Debug CLI now supports UserPromptSubmit events with LRU-cached regex (100 entry cap, state isolation)
- Cross-platform E2E path canonicalization fixing macOS /var → /private/var symlink issues
- CI matrix workflow for E2E tests across ubuntu, macOS, and Windows with binary artifact validation
- Tauri CI build pipeline with E2E gate (web mode) and multi-platform desktop builds (.dmg, .msi, .AppImage)
- Fixed e2e.yml directory mismatch (rulez_ui → rulez-ui) enabling Playwright E2E tests

**Stats:**

- 14 files created/modified
- +1,293 / -61 lines
- 4 phases, 9 plans
- 634 tests passing (up from 605)
- 1 day execution (2026-02-10)

**Git range:** `feat(07-01)` → `docs(10-02)`

**Tech debt resolved from v1.3:**
- Regex cache now bounded (LRU 100 entries) — was unbounded HashMap
- Debug CLI now supports all event types including UserPromptSubmit

**What's next:** `/gsd:new-milestone` for next milestone planning

---

## v1.6 RuleZ UI (Shipped: 2026-02-12)

**Delivered:** Production-ready desktop UI for RuleZ policy management with log viewer, config management, debug simulator, and onboarding.

**Phases completed:** 11-17 (19 plans total)

**Key accomplishments:**

- Tauri 2.0 desktop app with React 18, TypeScript 5.7+, Tailwind CSS 4
- Complete cch→rulez rename across all UI labels, Tauri commands, and settings
- Monaco YAML editor with schema validation, autocomplete, memory management
- High-performance log viewer with virtual scrolling (100K+ entries at 60fps)
- Multi-scope config management with import/export and file watching
- Debug simulator using real `rulez debug` binary with step-by-step traces
- First-run onboarding wizard with binary detection and sample config generation
- Comprehensive Playwright E2E tests (56 tests) across all features

**Stats:**
- 7 phases, 19 plans
- 18 React components, 3 Zustand stores
- Dual-mode architecture (Tauri desktop + web browser fallback)

---

## v1.7 Multi-Platform Hook Support (Shipped: 2026-02-13)

**Delivered:** RuleZ integration with OpenCode, Gemini CLI, and Copilot hook surfaces.

**Phases completed:** 18-21 (12 plans total, Phase 19 superseded by Phase 20)

**Key accomplishments:**

- OpenCode plugin integration with lifecycle event mapping and audit logging
- Gemini CLI adapter with dual-fire events, install command, and doctor diagnostics
- Copilot CLI adapter with hook runner, install tooling, and VS Code chat participant
- 16 canonical event types with serde aliases for backward compatibility
- Multi-platform adapter architecture supporting 4 CLIs

**Stats:**
- 4 phases, 12 plans (Phase 19 absorbed into Phase 20)

---

## v1.8 Tool Name Canonicalization (Shipped: 2026-02-22)

**Delivered:** Cross-platform tool name normalization so rules work identically across all CLIs.

**Phases completed:** 22 (2 plans)

**Key accomplishments:**

- Platform-specific tool names normalized to Claude Code PascalCase at adapter ingestion
- Original platform tool name preserved in `tool_input.platform_tool_name`
- TOOL-MAPPING.md cross-platform reference with all canonical names and aliases
- Map-first pattern refactor fixing Rust ownership bugs in Gemini and Copilot adapters

**Stats:**
- 1 phase, 2 plans

---

## v1.9 Multi-CLI E2E Testing — Partial (Shipped: 2026-03-05)

**Delivered:** E2E test harness framework and first two CLI scenario suites (Claude Code, Copilot).

**Phases completed:** 23, 25 (5 plans total)

**Key accomplishments:**

- Pure bash E2E harness with workspace isolation, assertion library, reporting (ASCII table + JUnit XML + Markdown)
- Claude Code adapter with 4 scenarios (install, hook-fire, deny, inject)
- Copilot adapter with 4 scenarios + auth check gap closure
- `task e2e` entry point with dynamic scenario discovery
- Audit log as deterministic proof for integration assertions

**Stats:**
- 2 phases, 5 plans (2 Claude Code + 3 Copilot)
- Note: Phases 24, 26, 27 moved to v2.1

---

## v2.0 RuleZ Cleanup and Hardening (Shipped: 2026-03-05)

**Delivered:** Critical bug fixes, engine performance improvements, skill docs corrections, and auto-upgrade capability.

**Phases completed:** 28 (8 plans across 4 waves)

**Key accomplishments:**

- Regex fail-closed: invalid regex returns non-match (not silent match-all)
- Config cache with mtime-based invalidation
- tool_input fields exposed in enabled_when eval context
- Debug trace now exercises run action scripts with script_output in JSON
- Naive matchers replaced with globset crate
- `rulez upgrade` command using self_update crate (rustls backend)
- UI log filter debounce tuned (300ms → 200ms)
- Parallel rule evaluation with join_all for rule sets >= 10
- mastering-hooks skill docs corrected (7 field name mismatches)

**Stats:**
- 1 phase, 8 plans
- 5 GitHub issues closed (#101-#105)

---

## v2.1 Multi-CLI E2E Testing — Continued (Shipped: 2026-03-09)

**Delivered:** Remaining CLI E2E testing for Gemini, OpenCode, and Codex — completing coverage for all 5 CLIs.

**Phases completed:** 24, 26, 27 (4 plans total)

**Key accomplishments:**

- Gemini CLI E2E adapter and 4 scenarios (install, hook-fire, deny, inject)
- OpenCode CLI E2E adapter with TypeScript plugin and 4 scenarios
- Codex CLI E2E adapter with 4 scenarios (1 install + 3 skip stubs — no hooks support)
- GSD tracking reconciliation for all phases and milestones
- All 5 CLIs now have complete E2E scenario coverage

**Stats:**
- 3 phases, 4 plans
- All 5 CLIs covered: Claude Code, Gemini, Copilot, OpenCode, Codex

---

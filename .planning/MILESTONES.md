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


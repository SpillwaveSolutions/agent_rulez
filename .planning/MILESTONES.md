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


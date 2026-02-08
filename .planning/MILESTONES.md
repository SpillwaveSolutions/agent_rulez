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

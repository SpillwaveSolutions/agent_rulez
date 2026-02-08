# Living Memory

**Last Updated:** 2026-02-07
**Current Phase:** Milestone Complete
**Current Plan:** Ready for next milestone

---

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-07)

**Core value:** LLMs do not enforce policy. LLMs are subject to policy.
**Current focus:** v1.2 complete, planning next milestone

---

## Current Position

Phase: v1.2 Complete
Plan: Ready to plan next milestone
Status: Milestone shipped
Last activity: 2026-02-07 - Completed v1.2 milestone

Progress: ██████████ 100% (v1.2)

---

## v1.2 Milestone Summary

| Phase | Feature | Plans | Status |
|-------|---------|-------|--------|
| 1 | `inject_inline` | 1/1 | ✓ Complete |
| 2 | `inject_command` | 2/2 | ✓ Complete |
| 3 | `enabled_when` | 3/3 | ✓ Complete |

**Total:** 3 phases, 6 plans, 245 tests

---

## Key Decisions (v1.2)

1. **Binary renamed to `rulez`** (was `cch`)
2. **RuleZ Core is P1** - UI is P3
3. **Execution precedence:** inject_inline > inject_command > inject > run
4. **Fail-open semantics:** Command failures log warning but don't block
5. **evalexpr 13.1** for expression evaluation (lightweight, no deps)
6. **Underscore syntax** for variable names (env_CI, not env.CI)
7. **Fail-closed for enabled_when:** Invalid expressions disable the rule
8. **build_operator_tree** for syntax validation (parse without evaluate)

---

## Technical Notes

Total test count: 245
LOC (src/): 6,098 Rust

Key files modified in v1.2:
- `rulez/Cargo.toml` - evalexpr dependency
- `rulez/src/models.rs` - inject_inline, inject_command, enabled_when fields
- `rulez/src/hooks.rs` - execute_inject_command, build_eval_context, is_rule_enabled
- `rulez/src/config.rs` - Expression validation in validate()
- `rulez/tests/oq_us2_injection.rs` - Injection integration tests
- `rulez/tests/oq_us3_enabled_when.rs` - enabled_when integration tests

---

## Context for Next Session

RuleZ v1.2 Milestone is COMPLETE!

Next steps:
- Run `/gsd:new-milestone` to start v1.3 planning
- Future phases: prompt_match, require_fields, inline script blocks

---

*State file for GSD workflow continuity*

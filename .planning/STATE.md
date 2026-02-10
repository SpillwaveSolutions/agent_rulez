# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-08)

**Core value:** LLMs do not enforce policy. LLMs are subject to policy.
**Current focus:** v1.3 Advanced Matching & Validation - Phase 6 IN PROGRESS

## Current Position

Phase: 6 of 6 (Inline Script Blocks)
Plan: 3 of 3 (COMPLETE)
Status: Phase 6 COMPLETE - v1.3 Advanced Matching & Validation COMPLETE
Last activity: 2026-02-10 - Completed 06-03-PLAN.md (Integration Tests & Verification)

Progress: █████████████████████████ 100% (6 phases complete)

## Performance Metrics

**Velocity:**
- Total plans completed: 16 (6 v1.2 + 10 v1.3)
- Average duration: ~6min (Phases 4-6)
- Total execution time: 100min (Phase 4: 61min, Phase 5: 21min, Phase 6: 18min)

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1. Inline Content Injection | 1 | - | - |
| 2. Command-Based Context | 2 | - | - |
| 3. Conditional Rule Activation | 3 | - | - |
| 4. Prompt Matching | 4/4 | 61min | 15min |
| 5. Field Validation | 3/3 | 21min | 7min |
| 6. Inline Script Blocks | 3/3 | 18min | 6min |

**Recent Trend:**
- v1.3 Phase 5 Plan 3 complete (8 min)
- v1.3 Phase 6 Plan 1 complete (5 min)
- v1.3 Phase 6 Plan 2 complete (3 min)
- v1.3 Phase 6 Plan 3 complete (10 min) - PHASE 6 COMPLETE

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting v1.3 work:

- [v1.2]: evalexpr 13.1 for expression evaluation (lightweight, proven)
- [v1.2]: Fail-closed for enabled_when (invalid expressions disable rule)
- [v1.2]: Underscore syntax for variable names (env_CI, not env.CI)
- [v1.2]: Execution precedence: inject_inline > inject_command > inject > run
- [v1.3-04-01]: serde untagged enum for PromptMatch flexible YAML syntax
- [v1.3-04-01]: MatchMode defaults to Any (OR logic)
- [v1.3-04-01]: case_insensitive defaults to false
- [v1.3-04-02]: once_cell for lazy static regex caching
- [v1.3-04-02]: Cache key format "pattern:case_insensitive"
- [v1.3-04-02]: Fail-closed on invalid regex (log warning, return false)
- [v1.3-04-02]: Missing prompt causes prompt_match rules to not match
- [v1.3-04-03]: Validate patterns after shorthand expansion and anchor application
- [v1.3-04-03]: Include original pattern and expanded pattern in error messages
- [v1.3-04-04]: Integration tests follow OQ (Operational Qualification) pattern
- [v1.3-05-01]: Dot notation for field paths (user-friendly) with RFC 6901 JSON Pointer conversion
- [v1.3-05-01]: Validate field paths at config load time (fail-fast pattern)
- [v1.3-05-01]: Type specifiers whitelist: string, number, boolean, array, object, any
- [v1.3-05-01]: field_types implicitly requires field existence
- [Phase 05-field-validation]: Field validation as final matcher check (fail-closed on validation failure)
- [Phase 05-field-validation]: Null values treated as missing in field validation
- [Phase 05-field-validation]: Error accumulation pattern (collect all errors before returning)
- [v1.3-06-01]: validate_expr and inline_script are mutually exclusive (enforced at config load)
- [v1.3-06-01]: Warn (not error) on missing shebang or large inline scripts (>10KB)
- [v1.3-06-01]: validate_expr uses build_operator_tree for syntax validation (same pattern as enabled_when)
- [v1.3-06-02]: Move closures with cloned tool_input for 'static lifetime in custom functions
- [v1.3-06-02]: Empty string for missing/null fields in get_field() (consistent fallback)
- [v1.3-06-02]: tokio::time::timeout for script timeout (avoids child ownership issues)
- [v1.3-06-02]: Validation runs BEFORE all other actions (fail-closed gate pattern)
- [v1.3-06-03]: Exit code 2 for validation failures (aligns with validation error exit code)
- [v1.3-06-03]: Float comparison for number tests (evalexpr converts JSON numbers to Float)

### v1.3 Research Findings

From research/SUMMARY.md (completed 2026-02-08):

**Stack decisions:**
- Extend evalexpr 13.1 with custom functions (zero new dependencies)
- Reuse existing regex crate (zero new dependencies)
- Add jsonschema 0.41 for field validation (single new dependency)

**Critical pitfalls to avoid:**
1. Catastrophic regex backtracking (Phase 4) - Validate patterns at config load, add timeout
2. Script execution without sandboxing (Phase 6) - Implement seccomp + Landlock OR defer to v1.4
3. Nested JSON validation overhead (Phase 5) - Limit field paths to 5 levels

**Phase order rationale:**
- Phase 4 (prompt_match): COMPLETE - Simplest, mirrors command_match pattern
- Phase 5 (require_fields): Tests action extensibility, no external scripts
- Phase 6 (inline scripts): Most complex, sandboxing required

### Pending Todos

None yet for v1.3.

### Blockers/Concerns

**Phase 6 decision point:** Inline shell scripts require cross-platform sandboxing.
- Option A: Implement seccomp + Landlock (Linux only)
- Option B: Defer to v1.4 with proper cross-platform sandboxing
- Option C: Ship evalexpr custom functions only (no shell scripts)

Decision needed before Phase 6 planning.

## Test Coverage Summary

Phase 4 added 26 tests, Phase 5 added 54 tests + 15 integration, Phase 6 added 31 tests + 15 integration:

| Requirement | Description | Covered By |
|-------------|-------------|------------|
| PROMPT-01 | Regex pattern matching | Unit + Integration tests |
| PROMPT-02 | Case-insensitive matching | Unit + Integration tests |
| PROMPT-03 | Multiple patterns with any/all | Unit + Integration tests |
| PROMPT-04 | Anchor positions | Unit + Integration tests |
| PROMPT-05 | Prompt in evalexpr context | Unit + Integration tests |
| FIELD-01 | Field existence validation | Unit (6 tests) + Integration (2 tests) |
| FIELD-02 | Fail-closed blocking | Unit (7 tests) + Integration (2 tests) |
| FIELD-03 | Nested field paths (dot notation) | Unit (11 tests) + Integration (3 tests) |
| FIELD-04 | Field type validation | Unit (20 tests) + Integration (4 tests) |
| SCRIPT-01 | validate_expr in YAML | Unit (11 tests) + Integration (3 tests) |
| SCRIPT-02 | get_field/has_field functions | Unit (10 tests) + Integration (2 tests) |
| SCRIPT-03 | Boolean semantics | Unit (5 tests) + Integration (2 tests) |
| SCRIPT-04 | Inline shell scripts | Integration (3 tests) |
| SCRIPT-05 | Timeout protection | Integration (1 test) |
| SCRIPT-06 | Config validation | Unit (11 tests) + Integration (2 tests) |

Total tests: 262 (247 unit tests + 15 integration tests)
- Baseline: 191 unit tests
- Phase 4: +26 unit tests (217 total)
- Phase 5: +54 unit tests + 15 integration tests (232 unit + 15 integration)
- Phase 6: +31 unit tests + 15 integration tests (247 unit + 15 integration = 262 total)

## Session Continuity

Last session: 2026-02-10
Stopped at: Completed 06-03-PLAN.md (Integration Tests & Verification) - PHASE 6 COMPLETE, v1.3 COMPLETE
Resume file: None

Next action: v1.3 Advanced Matching & Validation is COMPLETE. All 6 phases (Prompt Matching, Field Validation, Inline Script Blocks) implemented and tested. Ready for v1.4 planning or other development work.

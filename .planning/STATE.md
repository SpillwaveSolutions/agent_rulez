# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-08)

**Core value:** LLMs do not enforce policy. LLMs are subject to policy.
**Current focus:** v1.3 Advanced Matching & Validation - Phase 4 COMPLETE

## Current Position

Phase: 4 of 6 (Prompt Matching) - COMPLETE
Plan: 4 of 4 (all plans complete)
Status: Phase complete
Last activity: 2026-02-09 - Completed 04-04-PLAN.md (Comprehensive Tests)

Progress: ██████████████████░░ 75% (4 of 6 phases complete)

## Performance Metrics

**Velocity:**
- Total plans completed: 10 (6 v1.2 + 4 v1.3)
- Average duration: ~15min (Phase 4)
- Total execution time: 61min (Phase 4)

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1. Inline Content Injection | 1 | - | - |
| 2. Command-Based Context | 2 | - | - |
| 3. Conditional Rule Activation | 3 | - | - |
| 4. Prompt Matching | 4/4 | 61min | 15min |

**Recent Trend:**
- v1.3 Phase 4 Plan 1 complete (15 min)
- v1.3 Phase 4 Plan 2 complete (18 min)
- v1.3 Phase 4 Plan 3 complete (10 min)
- v1.3 Phase 4 Plan 4 complete (18 min)

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

## Test Coverage Summary (Phase 4)

Phase 4 added 70+ new tests covering all PROMPT requirements:

| Requirement | Description | Covered By |
|-------------|-------------|------------|
| PROMPT-01 | Regex pattern matching | Unit + Integration tests |
| PROMPT-02 | Case-insensitive matching | Unit + Integration tests |
| PROMPT-03 | Multiple patterns with any/all | Unit + Integration tests |
| PROMPT-04 | Anchor positions | Unit + Integration tests |
| PROMPT-05 | Prompt in evalexpr context | Unit + Integration tests |

Total tests: 407 (up from 285)

## Session Continuity

Last session: 2026-02-09
Stopped at: Completed 04-04-PLAN.md (Comprehensive Tests) - Phase 4 COMPLETE
Resume file: None

Next action: Begin Phase 5 (Require Fields) or Phase 6 (Inline Scripts)

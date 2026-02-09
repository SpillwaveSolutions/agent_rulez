# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-08)

**Core value:** LLMs do not enforce policy. LLMs are subject to policy.
**Current focus:** v1.3 Advanced Matching & Validation - Phase 4 (Prompt Matching)

## Current Position

Phase: 4 of 6 (Prompt Matching)
Plan: 3 of 4 (plans 01, 02, 03 complete)
Status: In progress
Last activity: 2026-02-09 - Completed 04-02-PLAN.md (Prompt Matching Logic)

Progress: ██████████████░░░░ 65% (3.75 of 6 phases - Phase 4 Plans 1, 2, 3 complete)

## Performance Metrics

**Velocity:**
- Total plans completed: 9 (6 v1.2 + 3 v1.3)
- Average duration: ~14min (Phase 4)
- Total execution time: Not fully tracked

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1. Inline Content Injection | 1 | - | - |
| 2. Command-Based Context | 2 | - | - |
| 3. Conditional Rule Activation | 3 | - | - |
| 4. Prompt Matching | 3/4 | 43min | 14min |

**Recent Trend:**
- v1.3 Phase 4 Plan 1 complete (15 min)
- v1.3 Phase 4 Plan 2 complete (18 min)
- v1.3 Phase 4 Plan 3 complete (10 min)

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
- Phase 4 (prompt_match): Simplest, mirrors command_match pattern
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

## Session Continuity

Last session: 2026-02-09
Stopped at: Completed 04-02-PLAN.md (Prompt Matching Logic)
Resume file: None

Next action: Execute next plan in Phase 4 (04-04-PLAN.md for integration tests)

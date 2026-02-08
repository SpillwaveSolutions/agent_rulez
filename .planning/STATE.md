# Living Memory

**Last Updated:** 2026-02-08
**Current Phase:** Not started (defining requirements)
**Current Plan:** —

---

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-08)

**Core value:** LLMs do not enforce policy. LLMs are subject to policy.
**Current focus:** v1.3 Advanced Matching & Validation

---

## Current Position

Phase: Not started (defining requirements)
Plan: —
Status: Defining requirements
Last activity: 2026-02-08 — Milestone v1.3 started

Progress: ░░░░░░░░░░ 0%

---

## v1.3 Target Features

| Feature | Description | Status |
|---------|-------------|--------|
| `prompt_match` | Match rules against user prompt text | Not started |
| `require_fields` | Validate required fields in tool input | Not started |
| Inline scripts | Write validator scripts in YAML | Not started |

---

## Key Decisions (Carried from v1.2)

1. **Binary renamed to `rulez`** (was `cch`)
2. **RuleZ Core is P1** - UI is P3
3. **Execution precedence:** inject_inline > inject_command > inject > run
4. **Fail-open semantics:** Command failures log warning but don't block
5. **evalexpr 13.1** for expression evaluation (lightweight, no deps)
6. **Underscore syntax** for variable names (env_CI, not env.CI)
7. **Fail-closed for enabled_when:** Invalid expressions disable the rule
8. **build_operator_tree** for syntax validation (parse without evaluate)

---

## Context for Next Session

Starting v1.3 milestone with three features:
- prompt_match matcher
- require_fields action
- Inline script blocks

Next: Define requirements, create roadmap, then plan phases.

---

*State file for GSD workflow continuity*

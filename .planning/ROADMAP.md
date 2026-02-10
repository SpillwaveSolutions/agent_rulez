# RuleZ Core Roadmap

**Current Focus:** v1.3 Advanced Matching & Validation

---

## Milestones

- ✅ **v1.2 P2 Features** — Phases 1-3 (shipped 2026-02-07) — [Archive](milestones/v1.2-ROADMAP.md)
- 🚧 **v1.3 Advanced Matching & Validation** — Phases 4-6 (planned)

---

## Completed: v1.2 P2 Features

<details>
<summary>✅ v1.2 P2 Features (Phases 1-3) — SHIPPED 2026-02-07</summary>

- [x] Phase 1: Inline Content Injection (1/1 plans) — inject_inline field
- [x] Phase 2: Command-Based Context Generation (2/2 plans) — inject_command field
- [x] Phase 3: Conditional Rule Activation (3/3 plans) — enabled_when field

See [v1.2-ROADMAP.md](milestones/v1.2-ROADMAP.md) for full details.

</details>

---

## v1.3 Advanced Matching & Validation (In Planning)

**Milestone Goal:** Enable intent-based routing, required field validation, and inline validation logic for more powerful rule authoring.

### Phase 4: Prompt Matching ✅

**Goal:** Users can route rules based on prompt text patterns, enabling intent-based policy enforcement.

**Depends on:** Phase 3 (v1.2 complete)

**Requirements:** PROMPT-01, PROMPT-02, PROMPT-03, PROMPT-04, PROMPT-05

**Success Criteria** (what must be TRUE):
1. ✅ User can write rules that match against prompt text using regex patterns
2. ✅ User can enable case-insensitive matching via configuration flag
3. ✅ User can combine multiple prompt patterns with AND/OR logic
4. ✅ User can anchor patterns to match at start, end, or anywhere in prompt
5. ✅ User can use evalexpr-based matching for complex prompt logic

**Plans:** 4 plans in 3 waves — COMPLETE

Plans:
- [x] 04-01-PLAN.md — Core types (PromptMatch enum, Matchers extension, Event prompt field)
- [x] 04-02-PLAN.md — Matching logic (regex caching, matches_prompt function)
- [x] 04-03-PLAN.md — Config validation for prompt_match patterns
- [x] 04-04-PLAN.md — Comprehensive unit and integration tests

### Phase 5: Field Validation ✅

**Goal:** Users can enforce required fields in tool inputs with fail-closed blocking, preventing incomplete or malformed tool invocations.

**Depends on:** Phase 4

**Requirements:** FIELD-01, FIELD-02, FIELD-03, FIELD-04

**Success Criteria** (what must be TRUE):
1. ✅ User can specify required fields that must exist in tool_input JSON
2. ✅ System blocks tool execution when required fields are missing (fail-closed)
3. ✅ User can validate nested field paths using dot notation (e.g., input.user.name)
4. ✅ User can validate field types match expected values (string, number, boolean, array, object)

**Plans:** 3 plans in 3 waves — COMPLETE

Plans:
- [x] 05-01-PLAN.md — Types, config validation (require_fields, field_types, dot_to_pointer)
- [x] 05-02-PLAN.md — Matching logic (validate_required_fields, hooks.rs integration)
- [x] 05-03-PLAN.md — Comprehensive unit and integration tests

### Phase 6: Inline Script Blocks

**Goal:** Users can write validation logic directly in YAML using evalexpr expressions and shell scripts, eliminating need for external script files.

**Depends on:** Phase 5

**Requirements:** SCRIPT-01, SCRIPT-02, SCRIPT-03, SCRIPT-04, SCRIPT-05, SCRIPT-06

**Success Criteria** (what must be TRUE):
1. User can write evalexpr expressions directly in YAML for inline validation
2. evalexpr expressions have access to custom functions (get_field, has_field) for field inspection
3. User can write inline shell scripts using YAML literal block syntax
4. Shell scripts execute with timeout protection, failing closed on timeout
5. System validates script syntax at configuration load time, rejecting invalid configs

**Plans:** 3 plans in 3 waves

Plans:
- [ ] 06-01-PLAN.md — Types and config validation (validate_expr, inline_script fields, syntax validation)
- [ ] 06-02-PLAN.md — Execution logic (custom functions get_field/has_field, inline script execution, pipeline integration)
- [ ] 06-03-PLAN.md — Comprehensive unit and integration tests (SCRIPT-01 through SCRIPT-06)

---

## Progress

**Execution Order:**
Phases execute in numeric order: 4 → 5 → 6

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Inline Content Injection | v1.2 | 1/1 | ✅ Complete | 2026-02-06 |
| 2. Command-Based Context | v1.2 | 2/2 | ✅ Complete | 2026-02-06 |
| 3. Conditional Rule Activation | v1.2 | 3/3 | ✅ Complete | 2026-02-07 |
| 4. Prompt Matching | v1.3 | 4/4 | ✅ Complete | 2026-02-09 |
| 5. Field Validation | v1.3 | 3/3 | ✅ Complete | 2026-02-09 |
| 6. Inline Script Blocks | v1.3 | 0/3 | Planned | - |

---

*Created 2026-02-06 - Updated 2026-02-09 Phase 5 complete*

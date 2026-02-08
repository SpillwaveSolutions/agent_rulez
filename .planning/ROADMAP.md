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

### Phase 4: Prompt Matching

**Goal:** Users can route rules based on prompt text patterns, enabling intent-based policy enforcement.

**Depends on:** Phase 3 (v1.2 complete)

**Requirements:** PROMPT-01, PROMPT-02, PROMPT-03, PROMPT-04, PROMPT-05

**Success Criteria** (what must be TRUE):
1. User can write rules that match against prompt text using regex patterns
2. User can enable case-insensitive matching via configuration flag
3. User can combine multiple prompt patterns with AND/OR logic
4. User can anchor patterns to match at start, end, or anywhere in prompt
5. User can use evalexpr-based matching for complex prompt logic

**Plans:** TBD

Plans:
- [ ] 04-01: TBD during planning

### Phase 5: Field Validation

**Goal:** Users can enforce required fields in tool inputs with fail-closed blocking, preventing incomplete or malformed tool invocations.

**Depends on:** Phase 4

**Requirements:** FIELD-01, FIELD-02, FIELD-03, FIELD-04

**Success Criteria** (what must be TRUE):
1. User can specify required fields that must exist in tool_input JSON
2. System blocks tool execution when required fields are missing (fail-closed)
3. User can validate nested field paths using dot notation (e.g., input.user.name)
4. User can validate field types match expected values (string, number, boolean, array, object)

**Plans:** TBD

Plans:
- [ ] 05-01: TBD during planning

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

**Plans:** TBD

Plans:
- [ ] 06-01: TBD during planning

---

## Progress

**Execution Order:**
Phases execute in numeric order: 4 → 5 → 6

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Inline Content Injection | v1.2 | 1/1 | ✅ Complete | 2026-02-06 |
| 2. Command-Based Context | v1.2 | 2/2 | ✅ Complete | 2026-02-06 |
| 3. Conditional Rule Activation | v1.2 | 3/3 | ✅ Complete | 2026-02-07 |
| 4. Prompt Matching | v1.3 | 0/? | Not started | - |
| 5. Field Validation | v1.3 | 0/? | Not started | - |
| 6. Inline Script Blocks | v1.3 | 0/? | Not started | - |

---

*Created 2026-02-06 - Updated 2026-02-08 with v1.3 roadmap*

# Requirements: RuleZ v1.3

**Defined:** 2026-02-08
**Core Value:** LLMs do not enforce policy. LLMs are subject to policy.

## v1.3 Requirements

Requirements for milestone v1.3: Advanced Matching & Validation.

### Prompt Matching

- [ ] **PROMPT-01**: User can match rules against user prompt text using regex patterns
- [ ] **PROMPT-02**: User can specify case-insensitive matching via `case_insensitive: true`
- [ ] **PROMPT-03**: User can specify multiple patterns with any/all matching logic
- [ ] **PROMPT-04**: User can anchor patterns to start, end, or contains
- [ ] **PROMPT-05**: User can use script-based matching that returns true/false

### Field Validation

- [ ] **FIELD-01**: User can require specific fields exist in tool input
- [ ] **FIELD-02**: System blocks action if required fields are missing (fail-closed)
- [ ] **FIELD-03**: User can specify nested field paths with dot notation (e.g., `input.user.name`)
- [ ] **FIELD-04**: User can validate field types (string, number, boolean, array, object)

### Inline Scripts

- [ ] **SCRIPT-01**: User can write evalexpr expressions directly in YAML for validation
- [ ] **SCRIPT-02**: evalexpr has access to `get_field()` and `has_field()` custom functions
- [ ] **SCRIPT-03**: evalexpr expressions return boolean (pass/fail) for validation
- [ ] **SCRIPT-04**: User can write inline shell scripts for complex validation
- [ ] **SCRIPT-05**: Shell scripts have timeout protection (fail-closed on timeout)
- [ ] **SCRIPT-06**: System validates script syntax at config load time

## Future Requirements

Deferred to v1.4+. Tracked but not in current roadmap.

### Advanced Validation

- **VALID-01**: JSON Schema validation for complex type checking
- **VALID-02**: Cross-field validation (field A depends on field B)

### Script Sandboxing

- **SAND-01**: Sandbox shell script execution on Linux (seccomp/Landlock)
- **SAND-02**: Sandbox shell script execution on macOS

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| NLP/semantic prompt matching | Performance impact (50+ MB, 100ms+ latency) |
| External state in validators | Security risk, use inject_command instead |
| Async validators | Breaks sub-10ms guarantee |
| Full scripting language (Rhai/Lua) | 500 KB binary impact, complexity, 7+ deps |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| PROMPT-01 | Phase 4 | Pending |
| PROMPT-02 | Phase 4 | Pending |
| PROMPT-03 | Phase 4 | Pending |
| PROMPT-04 | Phase 4 | Pending |
| PROMPT-05 | Phase 4 | Pending |
| FIELD-01 | Phase 5 | Pending |
| FIELD-02 | Phase 5 | Pending |
| FIELD-03 | Phase 5 | Pending |
| FIELD-04 | Phase 5 | Pending |
| SCRIPT-01 | Phase 6 | Pending |
| SCRIPT-02 | Phase 6 | Pending |
| SCRIPT-03 | Phase 6 | Pending |
| SCRIPT-04 | Phase 6 | Pending |
| SCRIPT-05 | Phase 6 | Pending |
| SCRIPT-06 | Phase 6 | Pending |

**Coverage:**
- v1.3 requirements: 15 total
- Mapped to phases: 15
- Unmapped: 0

✓ 100% requirement coverage achieved

---
*Requirements defined: 2026-02-08*
*Last updated: 2026-02-08 after roadmap creation - 100% coverage*

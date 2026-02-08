# RuleZ Core v1.2 Roadmap

**Milestone Goal:** Enhance the RuleZ policy engine with advanced injection and conditional rule features.

**Target:** RuleZ v1.2.0 with inline injection, command-based context, and conditional rules.

---

## Milestone: RuleZ v1.2 (P2 Features)

### Phase 1: Inline Content Injection ✓

**Status:** Complete (2026-02-06)

**Goal:** Allow injecting markdown content directly in rules without separate files.

**User Story:** US-ADV-04 from cch-advanced-rules spec

**Plans:** 1 plan (complete)

Plans:
- [x] 01-01-PLAN.md - Add inject_inline field and tests

**Implementation:**
- Added `inject_inline: Option<String>` to Actions struct
- Handles in both normal and warn mode
- inject_inline takes precedence over inject when both specified
- 5 unit tests + 2 integration tests

**Success Criteria:**
- [x] `inject_inline` parses from YAML
- [x] Content injected into response context
- [x] Works alongside existing `inject:` file action
- [x] Tests cover multiline content

---

### Phase 2: Command-Based Context Generation ✓

**Status:** Complete (2026-02-06)

**Goal:** Generate context dynamically by running a shell command.

**User Story:** US-ADV-05 from cch-advanced-rules spec

**Plans:** 2 plans (complete)

Plans:
- [x] 02-01-PLAN.md - Add inject_command field and execute_inject_command function
- [x] 02-02-PLAN.md - Add integration tests for inject_command

**Implementation:**
- Added `inject_command: Option<String>` to Actions struct
- Implemented `execute_inject_command` async function with timeout and error handling
- Execution order: inject_inline > inject_command > inject > run
- 3 unit tests + 3 integration tests

**Success Criteria:**
- [x] `inject_command` parses from YAML
- [x] Command executed with timeout
- [x] stdout injected as context
- [x] stderr/failure logged but doesn't block

---

### Phase 3: Conditional Rule Activation ✓

**Status:** Complete (2026-02-07)

**Goal:** Rules that only activate under certain conditions.

**User Story:** US-ADV-01 from cch-advanced-rules spec

**Plans:** 3 plans (complete)

Plans:
- [x] 03-01-PLAN.md - Add evalexpr dependency and enabled_when field
- [x] 03-02-PLAN.md - Implement build_eval_context and is_rule_enabled
- [x] 03-03-PLAN.md - Add validation and integration tests

**Implementation:**
- Added `enabled_when: Option<String>` to Rule struct
- Added evalexpr 13.1 for expression evaluation
- Implemented build_eval_context() with env_*, tool_name, event_type
- Implemented is_rule_enabled() with fail-closed semantics
- Added validation in Config.validate() using build_operator_tree
- 8 unit tests + 5 integration tests

**Example:**
```yaml
rules:
  - name: ci-only-strict
    enabled_when: 'env_CI == "true"'
    matchers:
      tools: [Bash]
      command_match: "git push"
    actions:
      block: true
```

**Success Criteria:**
- [x] `enabled_when` parses from YAML
- [x] Expression evaluation works for env vars
- [x] Rule skipped when condition is false
- [x] Syntax errors reported by `rulez validate`

---

## Future Phases (Post v1.2)

### Phase 4: prompt_match Matcher
- Match against user prompt text
- Enables prompt-based routing

### Phase 5: require_fields Action
- Validate required fields exist in tool input
- Block if fields missing

### Phase 6: Inline Script Blocks
- Write validator scripts directly in YAML
- No separate script files needed

### Phase 7: RuleZ UI (Lower Priority)
- M2-M8 from previous roadmap
- Only if there's user demand

---

## Technical Considerations

**Models Changes (`rulez/src/models.rs`):**
- Add `inject_inline: Option<String>` to Actions
- Add `inject_command: Option<String>` to Actions
- Add `enabled_when: Option<String>` to Rule

**Hooks Changes (`rulez/src/hooks.rs`):**
- Handle `inject_inline` in execute_rule_actions
- Handle `inject_command` with subprocess execution
- Evaluate `enabled_when` before rule matching

**Expression Evaluation:**
- Simple parser for `env.VAR == 'value'` expressions
- Start minimal, expand later if needed

---

*Created 2026-02-06 - Focus on RuleZ Core P2 features*

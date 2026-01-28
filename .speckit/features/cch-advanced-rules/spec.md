# Feature Specification: CCH Advanced Rules

**Feature Branch**: `feature/cch-advanced-rules`
**Created**: 2026-01-27
**Status**: Backlog
**Input**: Features removed from mastering-hooks skill docs (never implemented in CCH binary)

## Background

During the skill documentation fix (January 2026), several features documented in the mastering-hooks skill were found to not exist in the CCH binary. These were fabricated by the AI that generated the original skill docs. This spec captures them as future backlog items.

## User Scenarios & Testing

### User Story 1 - enabled_when Conditional Matcher (Priority: P2)

Users want rules that only activate under certain conditions (e.g., CI environment, specific branches, test files). Currently, all rules are always active when their matchers match.

**Why this priority**: Enables environment-aware rules without duplicating configs. High value for teams with different dev/CI workflows.

**Independent Test**: Can be tested by creating a rule with `enabled_when: "env.CI == 'true'"` and verifying it only fires when the CI env var is set.

**Acceptance Scenarios**:

1. **Given** a rule with `enabled_when: "env.CI == 'true'"`, **When** the rule is evaluated in a non-CI environment, **Then** the rule does not match
2. **Given** a rule with `enabled_when: "env.CI == 'true'"`, **When** the rule is evaluated with `CI=true`, **Then** the rule matches normally

---

### User Story 2 - prompt_match Matcher (Priority: P3)

Users want rules that match against user prompt text, enabling prompt-based routing (e.g., deploy requests, slash commands).

**Why this priority**: Useful but niche. Most rules match on tool usage, not prompt text.

**Independent Test**: Can be tested by creating a rule with `prompt_match: "(?i)deploy"` and simulating a UserPromptSubmit event.

**Acceptance Scenarios**:

1. **Given** a rule with `prompt_match: "(?i)deploy"`, **When** a user types "Deploy to production", **Then** the rule matches
2. **Given** a rule with `prompt_match: "^/fix"`, **When** a user types "Fix the bug", **Then** the rule does not match (no leading slash)

---

### User Story 3 - require_fields Action (Priority: P3)

Users want to validate that required fields exist in tool input before allowing execution.

**Why this priority**: Low priority — most validation can be done via `run:` scripts.

**Independent Test**: Can be tested by creating a rule with `require_fields: [path, content]` on the Write tool.

**Acceptance Scenarios**:

1. **Given** a rule requiring fields `[path, content]` on Write, **When** Write is called with both, **Then** the tool proceeds
2. **Given** a rule requiring fields `[path, content]` on Write, **When** Write is called without `content`, **Then** the tool is blocked

---

### User Story 4 - Inline Content Injection (Priority: P2)

Users want to inject short markdown content directly in the rule without creating a separate file.

**Why this priority**: Reduces file proliferation for simple warnings or reminders. Currently `inject:` only accepts file paths.

**Independent Test**: Can be tested by creating a rule with `inject_inline: "Warning: check before proceeding"` and verifying the content appears in context.

**Acceptance Scenarios**:

1. **Given** a rule with `inject_inline: "## Warning\nBe careful"`, **When** the rule matches, **Then** the inline content is injected into Claude's context

---

### User Story 5 - Command-Based Context Generation (Priority: P2)

Users want to generate context dynamically by running a shell command (e.g., `git branch --show-current`).

**Why this priority**: Enables dynamic context without full validator scripts. Currently requires a `run:` script that outputs JSON.

**Independent Test**: Can be tested by creating a rule with `inject_command: "git branch --show-current"` and verifying the output appears in context.

**Acceptance Scenarios**:

1. **Given** a rule with `inject_command: "echo '## Branch\nMain'"`, **When** the rule matches, **Then** the command output is injected as context

---

### User Story 6 - Inline Script Blocks in run: (Priority: P3)

Users want to write small validator scripts directly in hooks.yaml instead of creating separate script files.

**Why this priority**: Convenience for simple checks. Currently `run:` only accepts file paths.

**Independent Test**: Can be tested by creating a rule with multiline `run:` script and verifying execution.

**Acceptance Scenarios**:

1. **Given** a rule with `run: |` multiline script block, **When** the rule matches, **Then** the inline script executes and returns JSON

---

### User Story 7 - Context Variables in Expressions (Priority: P3)

Users want access to runtime variables (`tool.name`, `env.CI`, `session.id`) in `enabled_when` expressions.

**Why this priority**: Required by US-ADV-01 (enabled_when). Dependency for conditional matching.

**Independent Test**: Can be tested by creating rules referencing `tool.name`, `env.CI`, etc.

**Acceptance Scenarios**:

1. **Given** an expression `tool.name == 'Bash'`, **When** Bash tool is used, **Then** the expression evaluates to true
2. **Given** an expression `env.USER == 'ci-bot'`, **When** USER env var is "ci-bot", **Then** the expression evaluates to true

---

### Edge Cases

- What happens when `enabled_when` expression has a syntax error?
- How does `inject_command` handle script timeouts?
- What happens with `require_fields` on tools that have no input fields?

## Requirements

### Functional Requirements

- **FR-001**: System MUST support `enabled_when` conditional expressions on rules
- **FR-002**: System MUST support `prompt_match` regex matching on user prompts
- **FR-003**: System MUST support `require_fields` action type for input validation
- **FR-004**: System MUST support inline content injection (not just file paths)
- **FR-005**: System MUST support command-based context generation
- **FR-006**: System MUST support inline script blocks in `run:` action
- **FR-007**: System MUST provide context variables for expressions

### Key Entities

- **Expression**: A conditional expression evaluated at runtime (used by `enabled_when`)
- **ContextVariable**: A runtime variable providing event context (tool.name, env.CI, etc.)

## Success Criteria

### Measurable Outcomes

- **SC-001**: All 7 user stories have passing integration tests
- **SC-002**: Backward compatibility maintained — existing configs work without changes
- **SC-003**: Performance stays under 10ms for rule evaluation with new matchers
- **SC-004**: `cch validate` catches expression syntax errors

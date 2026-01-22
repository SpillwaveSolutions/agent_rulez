# Feature Specification: CCH Binary v1

**Feature Branch**: `feature/cch-binary-v1`
**Created**: 2025-01-21
**Status**: In Progress
**Input**: PRD documents from docs/prds/

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Block Dangerous Operations (Priority: P1)

As a developer, I want CCH to automatically block dangerous git operations (like force push) so that I cannot accidentally overwrite team members' work.

**Why this priority**: Safety is the primary value proposition. Users must trust that CCH protects them from destructive actions.

**Independent Test**: Can be fully tested by configuring a block rule for `git push --force` and verifying the operation is blocked with a clear message.

**Acceptance Scenarios**:

1. **Given** hooks.yaml contains a rule matching `git push.*(--force|-f)` with `action: block`, **When** Claude attempts to run `git push --force`, **Then** the operation is blocked and stderr contains the configured message
2. **Given** the same configuration, **When** Claude runs `git push` (without force), **Then** the operation continues normally
3. **Given** a blocking rule, **When** blocked, **Then** the log file records the event with `"outcome": "block"`

---

### User Story 2 - Inject Context for Skill Triggers (Priority: P1)

As a developer, I want relevant skill documentation automatically injected when I edit files in specific directories, so that Claude always has the right context.

**Why this priority**: Context injection is the core feature that makes Claude more effective by providing relevant documentation automatically.

**Independent Test**: Can be tested by configuring an `inject` rule for CDK files and verifying the SKILL.md content appears in Claude's context.

**Acceptance Scenarios**:

1. **Given** hooks.yaml contains a PreToolUse rule with `tools: [Edit, Write]`, `directories: [cdk/**]`, and `inject: .claude/skills/aws-cdk/SKILL.md`, **When** Claude edits a file in `cdk/`, **Then** the response includes `"context": <contents of SKILL.md>`
2. **Given** the same configuration, **When** Claude edits a file in `src/`, **Then** no context is injected
3. **Given** multiple matching rules, **When** triggered, **Then** all contexts are aggregated in the response

---

### User Story 3 - Run Custom Validators (Priority: P2)

As a developer, I want to run custom Python scripts to enforce complex rules (like no console.log), so that I can create project-specific checks.

**Why this priority**: Custom validators unlock unlimited enforcement possibilities beyond pattern matching.

**Independent Test**: Can be tested by creating a validator script that checks for `console.log` and verifying it blocks when the pattern is found.

**Acceptance Scenarios**:

1. **Given** hooks.yaml contains a rule with `run: .claude/validators/no-console.py`, **When** the script exits 0, **Then** the operation continues
2. **Given** the same rule, **When** the script exits 1 with stderr "console.log found", **Then** the operation is blocked with that message
3. **Given** a script that outputs to stdout, **When** it exits 0, **Then** stdout content is injected as context
4. **Given** a script that times out (>5s default), **When** timeout occurs, **Then** the operation continues (fail-open) and warning is logged

---

### User Story 4 - Explain Commands Before Permission (Priority: P2)

As a developer, I want Claude to provide structured explanations before asking permission for commands, so that I understand what's happening.

**Why this priority**: Transparency in AI operations builds trust and helps users make informed decisions.

**Independent Test**: Can be tested by configuring a PermissionRequest rule with `inject: explain-command.md` template.

**Acceptance Scenarios**:

1. **Given** hooks.yaml contains a PermissionRequest rule with `inject: .claude/context/explain-command.md`, **When** Claude requests permission for a Bash command, **Then** the explanation template is included in context
2. **Given** a rule with `require_fields: [{name: "what", description: "Plain English explanation"}]`, **When** permission requested, **Then** Claude must fill in the required fields

---

### User Story 5 - Query Logs for Troubleshooting (Priority: P3)

As a developer, I want to query CCH logs to understand why rules did or didn't fire, so that I can debug my configuration.

**Why this priority**: Observability is essential for configuration debugging but not needed for basic operation.

**Independent Test**: Can be tested by running `cch logs` after hook events and verifying events are recorded.

**Acceptance Scenarios**:

1. **Given** CCH has processed events, **When** running `cch logs`, **Then** JSON Lines output shows all decisions
2. **Given** a log entry, **When** examining it, **Then** it contains `ts`, `event`, `tool`, `rules_matched`, `outcome`, and `timing`
3. **Given** `cch explain rule <name>`, **When** the rule exists, **Then** output shows matchers, actions, metadata, and recent trigger stats

---

### Edge Cases

- What happens when hooks.yaml is missing? → Fail gracefully, log warning, return `{"continue": true}`
- What happens when referenced inject file doesn't exist? → Log error, skip injection, continue operation
- What happens when validator script doesn't exist? → Log error, continue operation (fail-open)
- What happens when YAML is malformed? → Return error response, don't crash
- What happens with very large context files? → Truncate with warning in logs

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Binary MUST parse JSON hook events from stdin
- **FR-002**: Binary MUST output JSON responses to stdout
- **FR-003**: Binary MUST load configuration from `.claude/hooks.yaml` (project) with fallback to `~/.claude/hooks.yaml` (user)
- **FR-004**: Binary MUST support matchers: `tools`, `extensions`, `directories`, `operations`, `command_match`
- **FR-005**: Binary MUST support actions: `inject`, `run`, `block`, `block_if_match`
- **FR-006**: Binary MUST log all decisions to `~/.claude/logs/cch.log` in JSON Lines format
- **FR-007**: Binary MUST complete processing in <10ms (without validator scripts)
- **FR-008**: Binary MUST execute validator scripts with event JSON on stdin and environment variables
- **FR-009**: Binary MUST interpret validator exit codes: 0=continue, non-zero=block
- **FR-010**: Binary MUST support all Claude Code hook events: PreToolUse, PostToolUse, PermissionRequest, UserPromptSubmit, SessionStart, SessionEnd, PreCompact

### Key Entities

- **Rule**: A configuration entry with name, matchers, and actions
- **Event**: A Claude Code hook event with tool_name, tool_input, session_id
- **Response**: JSON output with continue (bool), context (string), reason (string if blocked)
- **LogEntry**: JSON record of each decision with timestamp, event type, rules matched, outcome, timing

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Cold start completes in <5ms (p95), <10ms (p99)
- **SC-002**: Rule matching completes in <1ms for up to 100 rules
- **SC-003**: All hook events logged with full provenance data
- **SC-004**: `cch validate` catches 100% of configuration errors before runtime
- **SC-005**: Zero memory leaks or crashes in 24-hour stress test

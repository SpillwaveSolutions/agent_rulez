# Multi-CLI E2E Test Harness - Context

**Gathered:** 2026-02-22
**Status:** Ready for phase creation

<domain>
## Phase Boundary

Build a headless multi-CLI E2E/UAT test harness for RuleZ that validates real integration behavior across 5 agent CLIs: Claude Code, Gemini CLI, GitHub Copilot CLI, OpenCode CLI, and Codex CLI. Each CLI gets its own phase with full E2E testing. RuleZ-only scope for now — sister products (Agent Memory, Agent Cron, Agent CLOD) add their own scenarios later.

**Phase structure:** One phase per CLI. The first phase (Claude Code) also establishes the harness framework. Subsequent CLI phases add adapters + scenarios to the existing harness.

**Phase order:**
1. Claude Code CLI (establishes harness + Claude Code scenarios)
2. Gemini CLI
3. Copilot CLI
4. OpenCode CLI
5. Codex CLI (hooks NOT supported — limited scenario set)

</domain>

<decisions>
## Implementation Decisions

### Product boundaries
- RuleZ-only scope — sister products (Memory, Cron, CLOD) are out of scope for this harness
- Sister products are developed by the same team, all work across the same 5 CLIs
- The harness framework should be extensible enough that sister products could add scenarios later
- Codex CLI does NOT support hooks — hook-based scenarios must be skipped (not failed) for Codex

### CLI detection & headless strategy
- All 5 CLIs are installed locally and will be required in CI (missing CLI = CI failure)
- Research phase must determine headless invocation flags for each CLI (use Perplexity MCP)
- Real API calls, not mocks — accept cost and non-determinism for true E2E validation
- Tests use manual hook config for workspace isolation (don't pollute user's real config), even though `rulez <cli> install` commands exist

### Harness language & entry point
- Shell scripts (bash) as primary implementation; TypeScript if needed for complex logic
- Lives at `e2e/` at repo root, completely separate from cargo unit/integration tests
- These are UAT tests, not unit tests — keep them independent
- Entry point: `task e2e` via Taskfile integration
- Isolated workspaces: `e2e/.runs/<run-id>/<cli>/<test-name>/`

### Scenario scope & proof artifacts
- **Core 4 scenarios per CLI (must-pass):**
  1. `rulez <cli> install` succeeds in clean workspace
  2. PreToolUse hook fires and is logged (audit log proof)
  3. Deny rule blocks a tool call (exit code + stderr assertion)
  4. Inject rule adds context (marker file via inject_command)
- Proof methods: ALL of — audit log parsing, inject_command marker files, exit code + stderr, and structural assertions
- Assertion strategies vary per scenario: structural assertions, regex pattern matching, constrained prompts for predictable output

### Reporting
- Console ASCII table (CLI × scenario matrix)
- JUnit XML for GitHub Actions CI integration
- Markdown summary for PR comments / Actions summaries
- Non-zero exit code if any scenario fails

### CI integration
- All 5 CLIs required in CI
- Unit tests run first, then E2E
- Upload `e2e/.runs/**` artifacts on failure
- Separate from existing cargo test pipeline

### Claude's Discretion
- Exact headless invocation flags per CLI (determined by research)
- Shell script structure and helper functions
- Fixture project contents
- Workspace cleanup strategy (keep on failure, clean on success)
- Timeout values per scenario
- Golden file format and comparison approach

</decisions>

<specifics>
## Specific Ideas

- Use the existing RuleZ codebase (`rulez/src/adapters/`, `rulez/src/cli/`) as reference for how hooks are registered and managed per CLI
- See `rulez <cli> install` and `rulez <cli> doctor` commands for each platform's hook wiring
- Codex CLI has no hooks support — its scenarios should only test non-hook capabilities
- Research must use Perplexity MCP to confirm headless flags, hook config locations, and env vars for each CLI

</specifics>

<deferred>
## Deferred Ideas

- Phase 22.1: Expose tool_input fields in enabled_when eval context (already captured as todo)
- Sister product E2E scenarios (Agent Memory, Agent Cron, Agent CLOD)
- Performance/load testing (many rules, large configs)
- Cross-CLI rule compatibility matrix testing
- Automated golden file update workflow

</deferred>

---

*Context gathered: 2026-02-22*

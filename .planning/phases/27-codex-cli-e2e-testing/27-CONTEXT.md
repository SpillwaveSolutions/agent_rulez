# Phase 27: Codex CLI E2E Testing - Context

**Gathered:** 2026-03-06
**Status:** Ready for planning

<domain>
## Phase Boundary

Add E2E test scenarios for OpenAI's Codex CLI to the existing multi-CLI E2E harness. Follows the established pattern from Phases 23-26 (Claude Code, Gemini, Copilot, OpenCode). Codex CLI does NOT currently support hooks, so hook-dependent scenarios will skip gracefully.

</domain>

<decisions>
## Implementation Decisions

### CI requirements
- Codex CLI is **optional** in CI — not required on runners
- If Codex CLI is missing, all scenarios exit 77 (skip) — no CI failure
- This avoids CI complexity for a CLI whose scenarios mostly skip

### Adapter pattern
- Full adapter with CLI detection (`codex_adapter_check()` looks for binary, reports version)
- Consistent with other adapters (claude, gemini, copilot, opencode)
- `require_codex_cli()` returns 77 if unavailable
- `codex_adapter.sh` in `e2e/lib/`

### Scenario scope
- Create all 4 standard scenarios matching the harness pattern:
  1. `01-install.sh` — runs if CLI found (tests adapter detection, workspace setup)
  2. `02-hook-fire.sh` — skips with clear "no hook support" message
  3. `03-deny.sh` — skips with clear "no hook support" message
  4. `04-inject.sh` — skips with clear "no hook support" message
- Scenarios ready to enable when/if Codex adds hooks support
- Fixtures in `e2e/fixtures/codex/`

### RuleZ source changes
- **No Rust changes in this phase** — no `rulez codex install` or `rulez codex hook` subcommand
- This phase is E2E testing only; CLI subcommands are a separate concern
- The adapter handles workspace setup directly without RuleZ CLI integration

### run.sh integration
- Add codex availability check block in `run.sh` (matching pattern for other CLIs)
- `CODEX_CLI_AVAILABLE` environment variable exported for scenarios

### Claude's Discretion
- Exact Codex CLI binary name and detection method (research needed — `codex` vs `openai-codex` vs other)
- Headless invocation flags for Codex CLI
- Skip message wording for hook-dependent scenarios
- Fixture file contents

</decisions>

<code_context>
## Existing Code Insights

### Reusable Assets
- `e2e/lib/harness.sh`: Full harness with workspace management, assertions, scenario runner
- `e2e/lib/reporting.sh`: JUnit XML, markdown summary, ASCII table output
- `e2e/lib/opencode_adapter.sh`: Most recent adapter — good template for codex adapter
- `e2e/run.sh`: Auto-discovers `e2e/scenarios/<cli>/` directories, CLI availability checks

### Established Patterns
- Each adapter exports: `<cli>_adapter_check()`, `require_<cli>_cli()`, `setup_<cli>_hooks()`, `invoke_<cli>_headless()`
- Scenarios define `scenario_<name>()` functions, called by `run_scenario()`
- Exit code 77 = skip (autotools convention), used throughout
- Fixtures per CLI in `e2e/fixtures/<cli>/` (hooks-deny.yaml, hooks-hookfire.yaml, hooks-inject.yaml.template)

### Integration Points
- `e2e/run.sh` needs: source codex_adapter.sh + add codex availability check block
- `e2e/scenarios/codex/` directory with 4 scenario scripts
- `e2e/fixtures/codex/` directory with fixture YAML files
- `e2e/lib/codex_adapter.sh` — new adapter file

</code_context>

<specifics>
## Specific Ideas

- Follow `opencode_adapter.sh` as the closest template since it was the most recently built adapter
- Hook-dependent scenarios should print a clear, informative skip message explaining Codex doesn't support hooks yet
- The install scenario should still validate workspace setup and adapter detection even without hook support

</specifics>

<deferred>
## Deferred Ideas

- `rulez codex install` / `rulez codex hook` CLI subcommands — separate phase when Codex adds hook support
- Enabling hook-fire, deny, inject scenarios — when Codex CLI adds hooks
- Non-hook Codex invocation testing (direct tool use without policy enforcement)

</deferred>

---

*Phase: 27-codex-cli-e2e-testing*
*Context gathered: 2026-03-06*

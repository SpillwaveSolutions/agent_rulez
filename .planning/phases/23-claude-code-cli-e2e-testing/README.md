# Phase 23: Claude Code CLI E2E Testing

**CLI:** Claude Code
**Milestone:** v1.8

This phase establishes the multi-CLI E2E test harness framework and implements the first set of scenarios targeting the Claude Code CLI. The harness is designed to be extended by subsequent phases for additional CLIs.

**Scenarios:**
1. `rulez claude install` succeeds in clean workspace
2. PreToolUse hook fires and is logged (audit log proof)
3. Deny rule blocks a tool call (exit code + stderr assertion)
4. Inject rule adds context (marker file via inject_command)

See [E2E-CONTEXT.md](E2E-CONTEXT.md) for shared decisions and harness architecture.

# Phase 25: Copilot CLI E2E Testing

**CLI:** GitHub Copilot CLI
**Milestone:** v1.8

This phase adds the GitHub Copilot CLI adapter and scenarios to the E2E test harness established in Phase 23.

**Scenarios:**
1. `rulez copilot install` succeeds in clean workspace
2. PreToolUse hook fires and is logged (audit log proof)
3. Deny rule blocks a tool call (exit code + stderr assertion)
4. Inject rule adds context (marker file via inject_command)

See [E2E-CONTEXT.md](E2E-CONTEXT.md) for shared decisions and harness architecture.

# Phase 27: Codex CLI E2E Testing

**CLI:** Codex CLI
**Milestone:** v1.8

This phase adds the Codex CLI adapter and limited scenarios to the E2E test harness established in Phase 23. Codex CLI does NOT support hooks, so hook-based scenarios are skipped (not failed).

**Scenarios (limited -- no hooks support):**
- Non-hook capabilities only
- Hook-based scenarios (install, hook-fire, deny, inject) are skipped with clear skip reasons

See [E2E-CONTEXT.md](E2E-CONTEXT.md) for shared decisions and harness architecture.

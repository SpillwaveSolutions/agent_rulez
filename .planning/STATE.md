---
gsd_state_version: 1.0
milestone: v1.6
milestone_name: RuleZ UI
status: completed
stopped_at: Phase 27 context gathered
last_updated: "2026-03-06T22:45:29.579Z"
last_activity: "2026-03-05 — Phase 28 complete: regex fix, docs fix, upgrade cmd, debounce, tool_input eval, debug trace, globset, parallel eval"
progress:
  total_phases: 18
  completed_phases: 9
  total_plans: 51
  completed_plans: 33
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-12)

**Core value:** LLMs do not enforce policy. LLMs are subject to policy.
**Current focus:** v2.1 — Multi-CLI E2E Testing (Phases 24, 26, 27)
**v2.0:** RuleZ Cleanup and Hardening — COMPLETE (Phase 28, shipped 2026-03-05)
**v1.9:** Multi-CLI E2E Testing (partial) — COMPLETE (Phases 23, 25, shipped 2026-03-05)
**v1.8:** Tool Name Canonicalization — COMPLETE (Phase 22, shipped 2026-02-22)
**v1.7:** Multi-Platform Hook Support — COMPLETE (all phases 18-21 done)
**v1.6:** RuleZ UI — COMPLETE (all phases 11-17 done)

## Current Position

Milestone: v2.0 — RuleZ Cleanup and Hardening
Phase: 28 (COMPLETE)
Plan: All 8 plans complete (01-08)
Status: Phase 28 complete. All 8 plans done across 4 waves.
Last activity: 2026-03-05 — Phase 28 complete: regex fix, docs fix, upgrade cmd, debounce, tool_input eval, debug trace, globset, parallel eval

Progress: [████████████████████████] 28/28 phases complete (v2.0 milestone done)

## Performance Metrics

**Velocity (all milestones):**
- Total plans completed: 58 (6 v1.2 + 10 v1.3 + 9 v1.4 + 19 v1.6 + 12 v1.7 + 2 v1.8)
- Average duration: ~5min per plan (Phases 4-10)
- v1.5.0 released 2026-02-11 (first successful cross-platform binary release)

**By Milestone:**

| Milestone | Phases | Plans | Status |
|-----------|--------|-------|--------|
| v1.2 | 3 | 6 | Complete |
| v1.3 | 3 | 10 | Complete |
| v1.4 | 4 | 9 | Complete |
| v1.6 | 7 | 19/19 | Complete |
| v1.7 | 4 | 12/12 | Complete |

**Recent Trend:**
- v1.4 shipped in 1 day (9 plans)
- Trend: Stable execution velocity

**Recent Executions:**

| Plan | Duration | Tasks | Files |
|------|----------|-------|-------|
| Phase 24 P02 | 2 min | 2 tasks | 4 files |
| Phase 24 P01 | 2 min | 2 tasks | 5 files |
| Phase 23 P02 | 5 min | 2 tasks | 9 files |
| Phase 23 P01 | 5 min | 2 tasks | 5 files |
| Phase 16 P01 | 1 min | 4 tasks | 7 files |
| Phase 16 P02 | 1 min | 1 task | 1 file |
| Phase 15 P01 | 1 min | 4 tasks | 2 files |
| Phase 15 P02 | 1 min | 4 tasks | 5 files |
| Phase 15 P03 | 1 min | 3 tasks | 2 files |
| Phase 14 P01 | 1 min | 3 tasks | 3 files |
| Phase 14 P02 | 1 min | 3 tasks | 3 files |
| Phase 14 P03 | 1 min | 3 tasks | 3 files |
| Phase 13 P01 | 1 min | 3 tasks | 8 files |
| Phase 13 P02 | 1 min | 4 tasks | 8 files |
| Phase 13 P03 | 1 min | 3 tasks | 3 files |
| Phase 21 P04 | 0 min | 2 tasks | 7 files |
| Phase 12 P01-03 | 1 min | 7 tasks | 5 files |
| Phase 11 P01 | 3 min | 3 tasks | 10 files |
| Phase 11 P02 | 0 min | 2 tasks | 7 files |
| Phase 11 P03 | 0 min | 2 tasks | 7 files |
| Phase 17 P01 | 1 min | 8 tasks | 20 files |
| Phase 17 P02 | 1 min | 7 tasks | 4 files |
| Phase 21-copilot-cli-support-and-copilot-hooks-support P01 | 4 min | 2 tasks | 4 files |
| Phase 21-copilot-cli-support-and-copilot-hooks-support P04 | 0 min | 2 tasks | 7 files |
| Phase 25-copilot-cli-e2e-testing P01 | 2 | 2 tasks | 5 files |
| Phase 25 P02 | 2 | 2 tasks | 4 files |
| Phase 25 P03 | 1 | 1 task | 1 file |
| Phase 28 P06 | 5 min | 2 tasks | 4 files |
| Phase 28 P07 | 5 min | 3 tasks | 2 files |
| Phase 28 P04 | 5 min | 2 tasks | 1 file |
| Phase 28 P03 | 5 min | 3 tasks | 2 files |
| Phase 28 P02 | 6 min | 3 tasks | 2 files |
| Phase 28 P05 | 5 min | 2 tasks | 3 files |
| Phase 28 P08 | 5 min | 3 tasks | 2 files |

## Accumulated Context

### Roadmap Evolution

- Phase 19 added: Gemini hooks support
- Phase 20 added: Gemini CLI support and Gemini hooks support
- Phase 21 added: Copilot CLI support and Copilot hooks support
- Phase 22 added: Tool Name Canonicalization Across Platforms
- Phases 23-27 added: Multi-CLI E2E Testing (Claude Code, Gemini, Copilot, OpenCode, Codex)
- Phase 28 added: RuleZ Cleanup and Hardening (all 9 pending todos: regex bug, debug bug, tool_input eval, globset, caching, parallel eval, log worker, skill docs, auto-upgrade)

### Decisions

Phase 24 decisions:
- Timeout exit (124) maps to skip (77) in invoke_gemini_headless — Gemini --yolo flag has known intermittent behavior
- Gemini fixture files identical to claude-code fixtures — canonical tool names (Bash) work for both CLIs via RuleZ canonicalization
- GEMINI_API_KEY check is part of gemini_adapter_check (unlike Claude) because Gemini CLI requires API key at launch
- Gemini BeforeTool hook uses regex matcher ".*" (not glob "*" like Claude Code)
- Hook command format: "${abs_rulez} gemini hook"
- Install scenario (01) uses --scope project --binary flags: scopes to workspace and locates rulez binary
- Scenarios 02-04 all call mkdir -p .claude before cp hooks.yaml: ensures dir exists in fresh workspaces

Phase 23 decisions:
- Pure bash harness with no Node/Python dependencies (locked from CONTEXT.md)
- Workspace isolation via project-level .claude/settings.json in isolated run dir (CLAUDE_CONFIG_DIR does not exist)
- Log assertion uses WORKSPACE_LOG_SNAPSHOT (wc -l before scenario) + tail -n +<snapshot+1> after (avoids global log contamination)
- Dynamic scenario discovery: run.sh discovers e2e/scenarios/<cli>/*.sh, no hardcoded CLI list
- Scenario function naming convention: scenario_<name> with dashes->underscores
- task e2e depends on build-cli to ensure fresh binary before tests
- claude_adapter_check called per-CLI in run.sh; all claude-code scenarios skipped with SKIP if claude not found
- Audit log as deterministic proof: all 3 claude-invoking scenarios verify via assert_log_contains (not claude stdout)
- install scenario (01) does not invoke claude CLI — structural assertion only — runs regardless of claude availability
- Log snapshot refreshed inside each claude scenario after setup_claude_hooks to avoid counting hook-setup writes

All v1.4 decisions archived to PROJECT.md Key Decisions table and milestones/v1.4-ROADMAP.md.

v1.6 roadmap decisions:
- Use java-junit parsing for Playwright JUnit output (reliable check publishing)
- Gate full OS matrix to main while keeping develop PRs on Ubuntu only
- Phase 11 first: Fix rename + settings foundation (foundation for all other features)
- Phase 12: Monaco editor enhancements (memory management must be correct before building on top)
- Phase 13: Log viewer (high user value, streaming patterns inform other features)
- Phase 14: Config management (enables workflows, file watching patterns reused)
- Phase 15: Debug simulator (needs settings panel from Phase 11 for binary path)
- Phase 16: Onboarding (polish layer after core features working)
- Phase 17: E2E testing (validate all features before release)

Phase 11 decisions:
- Persist settings under a single settings key with localStorage fallback to keep defaults consistent across Tauri and web modes.

Phase 14 decisions:
- No config merging — matches CLI first-found-wins behavior (project completely overrides global)
- File watching uses Tauri watchImmediate() wrapping OS-native watchers (FSEvents/inotify) with 500ms debounce
- Export writes raw YAML strings to preserve comments (never parse-then-reserialize)

Phase 20 decisions:
- Map Gemini BeforeTool/AfterTool to RuleZ PreToolUse/PostToolUse while preserving the original hook_event_name in tool_input.
- Translate RuleZ context to Gemini systemMessage by default, with JSON tool_input override for tool hooks.
- Ensure gemini_hook_event_name is included in tool_input overrides for Gemini tool events

Phase 21 decisions:
- Copilot hook format uses `permissionDecision` (allow/deny) + optional `permissionDecisionReason` + optional `tool_input` override
- Copilot hook files stored in `.github/hooks/*.json` with version 1 format
- `cch copilot install` generates wrapper scripts (bash + PowerShell) and `.github/hooks/rulez.json`
- `cch copilot doctor` scans `.github/hooks/*.json` for installed/missing/misconfigured/outdated hooks

Phase 28 P06 decisions:
- GitHub owner/repo values are placeholders (SpillwaveSolutions/agent_rulez) — must be updated when repo is made public with releases
- Using self_update crate (industry standard for Rust binary self-upgrade) over manual reqwest approach
- --check flag prints version info and exits 0/1 without installing — safe for use in CI/automation

Phase 28 P07 decisions:
- Log filter debounce was already implemented in LogFilterBar.tsx at 300ms (component-level, Option A) — only updated timer from 300ms to 200ms
- Added debounce comment to logStore.ts setTextFilter documenting that debounce is applied at the call site

Phase 25 decisions:
- copilot_adapter_check checks PATH only — no API key (Copilot uses OAuth login, unlike Gemini)
- Copilot hook format: .github/hooks/rulez.json with preToolUse/bash/powershell/timeoutSec (vs Gemini BeforeTool/command/timeout ms)
- invoke_copilot_headless uses --allow-all-tools (not --yolo --output-format json like Gemini)
- Fixture YAML files identical to Gemini fixtures — canonical tool names work for Copilot via RuleZ canonicalization
- [Phase 25]: 01-install.sh uses no --scope flag (copilot install has no --scope, unlike gemini which uses --scope project)
- [Phase 25]: Assertion for hook entry uses unquoted 'copilot hook' substring — JSON bash/powershell fields have path prefix
- [Phase 25 P03]: Auth gap closed — gh auth status (Stage 1) / copilot probe timeout (Stage 2) added to copilot_adapter_check; unauthenticated => return 1 => COPILOT_CLI_AVAILABLE=0 => scenarios 02-04 skip (exit 77)
- [Phase 28]: build_glob_set() auto-appends /** for bare dir names; invalid patterns warn+skip; GlobSet compiled per eval call
- [Phase 28 P08]: Parallel rule matching uses join_all for >= 10 rules; action execution stays sequential to preserve merge semantics; futures 0.3 added

### Pending Todos

- [x] Replace Naive Matchers with globset (tooling) — DONE in 28-05: build_glob_set() replaces contains() hack
- [x] Implement Regex and Config Caching (tooling) — DONE in 28-03: mtime-based CONFIG_CACHE in Config::from_file()
- [ ] Offload Log Filtering to Web Worker or Rust (ui)
- [x] Parallel Rule Evaluation (tooling) — DONE in 28-08: join_all parallel matching for rule sets >= 10 rules
- [x] Expose tool_input fields in enabled_when eval context (tooling, Phase 22.1) — DONE in 28-03: tool_input_ prefixed vars in build_eval_context()
- [x] Auto-check and upgrade RuleZ binary to latest release (tooling, [#102](https://github.com/SpillwaveSolutions/agent_rulez/issues/102)) — DONE in 28-06
- [x] Fix mastering-hooks skill schema mismatches with RuleZ binary (docs, [#103](https://github.com/SpillwaveSolutions/agent_rulez/issues/103), [#104](https://github.com/SpillwaveSolutions/agent_rulez/issues/104), [#105](https://github.com/SpillwaveSolutions/agent_rulez/issues/105)) — DONE in 28-02: rules:/matchers:/actions:/version:1.0 corrected in hooks-yaml-schema.md and rule-patterns.md
- [x] Fix invalid regex silently matching all commands and stale config cache (tooling, [#101](https://github.com/SpillwaveSolutions/agent_rulez/issues/101)) — DONE in 28-01: fail-closed regex at all 5 call sites, Config::validate() catches bad command_match regex
- [x] rulez debug does not exercise run action scripts (tooling, [#104](https://github.com/SpillwaveSolutions/agent_rulez/issues/104)) — DONE in 28-04: script_output field added to JSON trace, run scripts exercised via process_event()

Phase 28 P03 decisions:
- Numbers from tool_input JSON stored as evalexpr Float (f64) -- comparison expressions must use 30.0 not 30
- Cache placed in from_file() not load() so both project and global config paths benefit from caching
- Complex JSON types (arrays, objects, null) silently skipped in tool_input injection -- only string, bool, number supported by evalexpr

Phase 28 P02 decisions:
- inject: takes a file path; inject_inline: takes inline markdown; inject_command: takes a shell command (corrected in skill docs)
- priority: higher number = higher priority (original docs said "lower = higher" which was wrong)
- event: per-rule flat field does not exist; use matchers.operations: [EventType] instead

Phase 28 P01 decisions:
- Use if let Ok(regex) = get_or_compile_regex(...) / else { warn; return false } — clippy prefers if-let for two-arm match (single_match_else lint)
- get_or_compile_regex promoted to pub(crate) so debug.rs can call crate::hooks::get_or_compile_regex without duplicating logic
- Fail-closed on invalid block_if_match: log warning and continue (no error Response), since no content matched

### Blockers/Concerns

None active.

## Session Continuity

Last session: 2026-03-06T22:45:29.575Z
Stopped at: Phase 27 context gathered
Resume file: .planning/phases/27-codex-cli-e2e-testing/27-CONTEXT.md

Next action: Release v2.0 with release skill. Phases 24, 26, 27 moved to v2.1 milestone.

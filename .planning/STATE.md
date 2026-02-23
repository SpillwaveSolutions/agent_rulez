# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-12)

**Core value:** LLMs do not enforce policy. LLMs are subject to policy.
**Current focus:** v1.9 — Multi-CLI E2E Testing (Phases 23-27)
**v1.8:** Tool Name Canonicalization — COMPLETE (Phase 22, shipped 2026-02-22)
**v1.7:** Multi-Platform Hook Support — COMPLETE (all phases 18-21 done)
**v1.6:** RuleZ UI — COMPLETE (all phases 11-17 done)

## Current Position

Milestone: v1.9
Phase: 24 of 27
Plan: 02 complete (all 4 Gemini E2E scenarios created)
Status: In progress — Phase 24 complete (both plans done), Phase 25 (Copilot E2E) next
Last activity: 2026-02-23 — Phase 24 Plan 02 complete: all 4 Gemini E2E scenario scripts created

Progress: [██████████████████░░░░░] 22/27 phases complete (81%)

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

## Accumulated Context

### Roadmap Evolution

- Phase 19 added: Gemini hooks support
- Phase 20 added: Gemini CLI support and Gemini hooks support
- Phase 21 added: Copilot CLI support and Copilot hooks support
- Phase 22 added: Tool Name Canonicalization Across Platforms
- Phases 23-27 added: Multi-CLI E2E Testing (Claude Code, Gemini, Copilot, OpenCode, Codex)

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

### Pending Todos

- [ ] Replace Naive Matchers with globset (tooling)
- [ ] Implement Regex and Config Caching (tooling)
- [ ] Offload Log Filtering to Web Worker or Rust (ui)
- [ ] Parallel Rule Evaluation (tooling)
- [ ] Expose tool_input fields in enabled_when eval context (tooling, Phase 22.1)

### Blockers/Concerns

None active.

## Session Continuity

Last session: 2026-02-23
Stopped at: Completed 24-02-PLAN.md (all 4 Gemini E2E scenario scripts)
Resume file: None

Next action: Execute Phase 25 (Copilot E2E testing)

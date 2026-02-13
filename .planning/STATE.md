# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-12)

**Core value:** LLMs do not enforce policy. LLMs are subject to policy.
**Current focus:** v1.7 Multi-Platform Hook Support — COMPLETE (all phases 18-21 done)
**v1.6:** RuleZ UI — COMPLETE (all phases 11-17 done)

## Current Position

Milestone: v1.7 Multi-Platform Hook Support — COMPLETE
Phase: 21 of 21
Plan: All plans complete
Status: All phases complete (18 OpenCode, 19 superseded, 20 Gemini CLI, 21 Copilot CLI)
Last activity: 2026-02-13 — Completed Phase 18 (OpenCode doctor, tests, docs)

Progress: [█████████████████] 21/21 phases complete (100%)

## Performance Metrics

**Velocity (all milestones):**
- Total plans completed: 56 (6 v1.2 + 10 v1.3 + 9 v1.4 + 19 v1.6 + 12 v1.7)
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

### Decisions

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

### Blockers/Concerns

None active.

## Session Continuity

Last session: 2026-02-13
Stopped at: v1.7 COMPLETE — all phases 18-21 done (228 tests passing)
Resume file: None

Next action: Plan v1.8 milestone or release v1.7

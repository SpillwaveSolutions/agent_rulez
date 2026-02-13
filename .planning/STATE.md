# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-12)

**Core value:** LLMs do not enforce policy. LLMs are subject to policy.
**Current focus:** v1.6 RuleZ UI — Phase 16 (Onboarding) next
**v1.7 planned:** Multi-Platform Hook Support (OpenCode, Gemini CLI, GitHub Copilot) — Phases 18-21

## Current Position

Milestone: v1.6 RuleZ UI
Phase: 15 of 17 (Debug Simulator) — COMPLETE
Plan: 3 of 3
Status: Phase 15 complete, ready for Phase 16
Last activity: 2026-02-12 — Completed Phase 15 Plan 03 (Integration wiring + E2E test fixes)

Progress: [██████████████░░░] 16/21 phases complete (76%)

## Performance Metrics

**Velocity (all milestones):**
- Total plans completed: 42 (6 v1.2 + 10 v1.3 + 9 v1.4 + 17 v1.6)
- Average duration: ~5min per plan (Phases 4-10)
- v1.5.0 released 2026-02-11 (first successful cross-platform binary release)

**By Milestone:**

| Milestone | Phases | Plans | Status |
|-----------|--------|-------|--------|
| v1.2 | 3 | 6 | Complete |
| v1.3 | 3 | 10 | Complete |
| v1.4 | 4 | 9 | Complete |
| v1.6 | 7 | 17/TBD | In progress |
| v1.7 | 4 | TBD | Planned |

**Recent Trend:**
- v1.4 shipped in 1 day (9 plans)
- Trend: Stable execution velocity

**Recent Executions:**

| Plan | Duration | Tasks | Files |
|------|----------|-------|-------|
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

### Pending Todos

0 pending

### Blockers/Concerns

None active.

## Session Continuity

Last session: 2026-02-12
Stopped at: Completed Phase 15 (Debug Simulator) — all 3 plans done
Resume file: None

Next action: Plan Phase 16 (Onboarding)

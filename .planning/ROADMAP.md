# RuleZ Roadmap

**Current Focus:** v2.1 Multi-CLI E2E Testing — Phases 24, 26, 27

---

## Milestones

- ✅ **v1.2 P2 Features** — Phases 1-3 (shipped 2026-02-07) — [Archive](milestones/v1.2-ROADMAP.md)
- ✅ **v1.3 Advanced Matching & Validation** — Phases 4-6 (shipped 2026-02-10) — [Archive](milestones/v1.3-ROADMAP.md)
- ✅ **v1.4 Stability & Polish** — Phases 7-10 (shipped 2026-02-10) — [Archive](milestones/v1.4-ROADMAP.md)
- ✅ **v1.6 RuleZ UI** — Phases 11-17 (shipped 2026-02-12)
- ✅ **v1.7 Multi-Platform Hook Support** — Phases 18-21 (shipped 2026-02-13)
- ✅ **v1.8 Tool Name Canonicalization** — Phase 22 (shipped 2026-02-22)
- ✅ **v1.9 Multi-CLI E2E Testing** — Phases 23, 25 (shipped 2026-03-05)
- ✅ **v2.0 RuleZ Cleanup and Hardening** — Phase 28 (shipped 2026-03-05)
- 🔄 **v2.1 Multi-CLI E2E Testing (continued)** — Phases 24, 26, 27

---

<details>
<summary>✅ v1.2 P2 Features (Phases 1-3) — SHIPPED 2026-02-07</summary>

- [x] Phase 1: Inline Content Injection (1/1 plans) — inject_inline field
- [x] Phase 2: Command-Based Context Generation (2/2 plans) — inject_command field
- [x] Phase 3: Conditional Rule Activation (3/3 plans) — enabled_when field

See [v1.2-ROADMAP.md](milestones/v1.2-ROADMAP.md) for full details.

</details>

<details>
<summary>✅ v1.3 Advanced Matching & Validation (Phases 4-6) — SHIPPED 2026-02-10</summary>

- [x] Phase 4: Prompt Matching (4/4 plans) — regex intent routing with AND/OR logic
- [x] Phase 5: Field Validation (3/3 plans) — fail-closed field existence and type checks
- [x] Phase 6: Inline Script Blocks (3/3 plans) — evalexpr expressions and shell scripts in YAML

See [v1.3-ROADMAP.md](milestones/v1.3-ROADMAP.md) for full details.

</details>

<details>
<summary>✅ v1.4 Stability & Polish (Phases 7-10) — SHIPPED 2026-02-10</summary>

- [x] Phase 7: JSON Schema Validation (2/2 plans) — fail-open schema validation with <0.1ms overhead
- [x] Phase 8: Debug CLI Enhancements (2/2 plans) — UserPromptSubmit support, LRU regex cache
- [x] Phase 9: E2E Test Stabilization (3/3 plans) — canonical paths, symlink tests, CI matrix
- [x] Phase 10: Tauri CI Integration (2/2 plans) — E2E gate + multi-platform desktop builds

See [v1.4-ROADMAP.md](milestones/v1.4-ROADMAP.md) for full details.

</details>

---

## ✅ v1.6 RuleZ UI (Complete)

**Milestone Goal:** Production-ready desktop UI for RuleZ policy management with log viewer, config management, debug simulator, and onboarding.

### Phase 11: Rename Fix + Settings Foundation
**Goal**: Fix cch→rulez binary references and establish settings infrastructure for all features
**Depends on**: Phase 10
**Requirements**: RENAME-01, RENAME-02, RENAME-03, SET-01, SET-02, SET-03, SET-04, DBG-05
**Success Criteria** (what must be TRUE):
  1. User sees "rulez" (not "cch") in all UI labels, button text, and window titles
  2. Tauri backend invokes `rulez debug` and `rulez validate` successfully
  3. User can configure theme, editor font size, and binary path from settings panel
  4. Settings persist across app restarts (theme, editor preferences, binary path)
  5. Binary path is auto-detected from PATH with fallback to manual configuration
**Plans**: 3 plans

Plans:
- [x] 11-01: Rename sweep (UI labels, shell scope, log path)
- [x] 11-02: Settings store + binary path resolution
- [x] 11-03: Settings panel UI + live preferences

### Phase 12: YAML Editor Enhancements
**Goal**: Production-quality Monaco integration with autocomplete, error markers, and memory management
**Depends on**: Phase 11
**Requirements**: EDIT-01, EDIT-02, EDIT-03, EDIT-04, EDIT-05, EDIT-06
**Success Criteria** (what must be TRUE):
  1. User gets schema-driven autocomplete suggestions when typing rule field names
  2. User sees inline error markers (red squiggles) for YAML syntax violations and schema mismatches
  3. User can click errors in error panel to jump directly to the corresponding line
  4. User can format/indent YAML on save or via keyboard shortcut
  5. Editor properly disposes Monaco models and workers when switching between files (no memory leaks after 10+ file switches)
**Plans**: 3 plans

Plans:
- [x] 12-01: Schema hardening + YAML formatting provider
- [x] 12-02: Memory management & disposal patterns
- [x] 12-03: Format-on-save + integration verification

### Phase 13: Log Viewer
**Goal**: High-performance audit log visualization with virtual scrolling and filtering
**Depends on**: Phase 12
**Requirements**: LOG-01, LOG-02, LOG-03, LOG-04, LOG-05, LOG-06, LOG-07
**Success Criteria** (what must be TRUE):
  1. User can view audit log entries from `~/.claude/logs/rulez.log` in a scrollable list
  2. User can filter log entries by text content, severity level, and time range
  3. Log viewer renders 100K+ entries at 60fps with virtual scrolling
  4. User can export filtered log results to JSON or CSV files
  5. User can copy individual log entries to clipboard
**Plans**: 3 plans

Plans:
- [x] 13-01: Rust log parsing command + TypeScript types + Tauri wiring
- [x] 13-02: Log viewer UI with virtual scrolling, filtering, and Zustand store
- [x] 13-03: Export (JSON/CSV) + clipboard copy + integration verification

### Phase 14: Config Management
**Goal**: Multi-scope config handling with import/export and live reload
**Depends on**: Phase 13
**Requirements**: CFG-01, CFG-02, CFG-03, CFG-04, CFG-05, CFG-06
**Success Criteria** (what must be TRUE):
  1. User can switch between global and project configs with visual indicator of active scope
  2. User can import config files from disk with YAML validation before applying
  3. User can export current config to a file (preserving comments and formatting)
  4. User sees config precedence (project overrides global) clearly indicated in the UI
  5. Config changes auto-reload when the file is modified externally (debounced file watching)
**Plans**: 3 plans

Plans:
- [x] 14-01: Scope indicators + config precedence UI
- [x] 14-02: Import/export with validation
- [x] 14-03: File watching + external change detection

### Phase 15: Debug Simulator
**Goal**: Real binary integration with step-by-step rule evaluation traces
**Depends on**: Phase 14
**Requirements**: DBG-01, DBG-02, DBG-03, DBG-04
**Success Criteria** (what must be TRUE):
  1. User can run debug simulation using the real `rulez debug` binary (not mock data)
  2. User sees step-by-step rule evaluation trace showing which rules matched and why
  3. User can save debug test cases (event + expected result) for reuse
  4. User can load and replay saved test cases from previous sessions
**Plans**: 3 plans

Plans:
- [x] 15-01: CLI `--json` flag + full event type support
- [x] 15-02: Save/load test cases
- [x] 15-03: Integration wiring + E2E test fixes

### Phase 16: Onboarding
**Goal**: First-run wizard to guide new users through setup
**Depends on**: Phase 15
**Requirements**: OB-01, OB-02, OB-03, OB-04, OB-05
**Success Criteria** (what must be TRUE):
  1. First-time users see a setup wizard on initial app launch
  2. Wizard detects whether `rulez` binary is installed and accessible via PATH
  3. Wizard generates a sample `hooks.yaml` config with documented example rules
  4. Wizard guides user through a test simulation to verify setup works
  5. User can re-run onboarding wizard from settings panel
**Plans**: 2 plans

Plans:
- [x] 16-01: Onboarding wizard foundation + UI
- [x] 16-02: Settings panel integration + verification

### Phase 17: E2E Testing
**Goal**: Comprehensive Playwright E2E test coverage for all UI features
**Depends on**: Phase 16
**Requirements**: E2E-01, E2E-02, E2E-03
**Success Criteria** (what must be TRUE):
  1. All new UI features have Playwright E2E tests in web mode
  2. E2E tests cover editor, log viewer, config management, simulator, settings, and onboarding
  3. E2E test suite passes in CI (GitHub Actions) on ubuntu, macOS, and Windows
**Plans**: TBD

Plans:
- [x] 17-01: Comprehensive Feature Test Coverage
- [x] 17-02: CI Integration & Cross-Platform Matrix

---

## ✅ v1.7 Multi-Platform Hook Support (Complete)

**Milestone Goal:** Integrate RuleZ with OpenCode, Gemini CLI, and Copilot hook surfaces.

### Phase 18: OpenCode Plugin Integration
**Goal**: Integrate RuleZ with OpenCode plugin lifecycle for policy enforcement and audit logging
**Depends on**: Phase 17
**Requirements**: OPENCODE-01, OPENCODE-02, OPENCODE-03, OPENCODE-04, OPENCODE-05, OPENCODE-06
**Success Criteria** (what must be TRUE):
  1. OpenCode lifecycle events emit RuleZ hook events with mapped context
  2. RuleZ allow/deny/inject decisions are enforced in the OpenCode flow
  3. OpenCode exposes RuleZ tools for on-demand policy checks
  4. Plugin config loads from `~/.config/opencode/plugins/rulez-plugin/`
  5. All OpenCode-RuleZ interactions are logged to an audit trail with plugin metadata
**Plans**: 3 plans

Plans:
- [x] 18-01: OpenCode Event Capture + RuleZ Payload Mapping
- [x] 18-02: Policy Enforcement + Tool Registration
- [x] 18-03: Plugin Config + Audit Logging

---

## Progress

**Execution Order:**
Phases execute in numeric order: 11 → 12 → 13 → 14 → 15 → 16 → 17 → 18 → 19 → 20 → 21

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Inline Content Injection | v1.2 | 1/1 | ✅ Complete | 2026-02-06 |
| 2. Command-Based Context | v1.2 | 2/2 | ✅ Complete | 2026-02-06 |
| 3. Conditional Rule Activation | v1.2 | 3/3 | ✅ Complete | 2026-02-07 |
| 4. Prompt Matching | v1.3 | 4/4 | ✅ Complete | 2026-02-09 |
| 5. Field Validation | v1.3 | 3/3 | ✅ Complete | 2026-02-09 |
| 6. Inline Script Blocks | v1.3 | 3/3 | ✅ Complete | 2026-02-09 |
| 7. JSON Schema Validation | v1.4 | 2/2 | ✅ Complete | 2026-02-10 |
| 8. Debug CLI Enhancements | v1.4 | 2/2 | ✅ Complete | 2026-02-10 |
| 9. E2E Test Stabilization | v1.4 | 3/3 | ✅ Complete | 2026-02-10 |
| 10. Tauri CI Integration | v1.4 | 2/2 | ✅ Complete | 2026-02-10 |
| 11. Rename Fix + Settings Foundation | v1.6 | 3/3 | ✅ Complete | 2026-02-12 |
| 12. YAML Editor Enhancements | v1.6 | 3/3 | ✅ Complete | 2026-02-12 |
| 13. Log Viewer | v1.6 | 3/3 | ✅ Complete | 2026-02-12 |
| 14. Config Management | v1.6 | 3/3 | ✅ Complete | 2026-02-12 |
| 15. Debug Simulator | v1.6 | 3/3 | Complete | 2026-02-12 |
| 16. Onboarding | v1.6 | 2/2 | Complete | 2026-02-12 |
| 17. E2E Testing | v1.6 | 2/2 | ✅ Complete | 2026-02-11 |
| 18. OpenCode Plugin Integration | v1.7 | 3/3 | ✅ Complete | 2026-02-13 |
| 19. Gemini Hook Support | v1.7 | 0/3 | Superseded by Phase 20 | - |
| 20. Gemini CLI Support | v1.7 | 5/5 | ✅ Complete | 2026-02-12 |
| 21. Copilot CLI Support | v1.7 | 4/4 | ✅ Complete | 2026-02-13 |
| 22. Tool Name Canonicalization | v1.8 | 2/2 | ✅ Complete | 2026-02-20 |

### Phase 19: Gemini hooks support (Superseded)

**Goal:** Translate Gemini CLI hook events into RuleZ policy evaluation with install tooling and documentation.
**Depends on:** Phase 18
**Status:** Superseded — all work absorbed into Phase 20 which expanded to 5 plans covering the full scope.

Plans:
- [x] 19-01 — Absorbed into 20-01 (Gemini hook adapter + runner)
- [x] 19-02 — Absorbed into 20-03 (Gemini install + settings integration)
- [x] 19-03 — Absorbed into 20-05 (Gemini integration docs)

### Phase 20: Gemini CLI support and Gemini hooks support

**Goal:** Finalize Gemini CLI integration with full event coverage, install tooling, diagnostics, and documentation.
**Depends on:** Phase 18
**Plans:** 5 plans — all complete

Plans:
- [x] 20-01-PLAN.md — Gemini hook adapter + event mapping
- [x] 20-02-PLAN.md — Gemini hook runner subcommand
- [x] 20-03-PLAN.md — Gemini install command + settings integration
- [x] 20-04-PLAN.md — Gemini doctor diagnostics command
- [x] 20-05-PLAN.md — Gemini integration docs + examples

### Phase 21: Copilot CLI support and Copilot hooks support

**Goal:** Integrate RuleZ with Copilot CLI hooks and VS Code chat participant for policy enforcement and diagnostics.
**Depends on:** Phase 20
**Plans:** 4 plans — all complete

Plans:
- [x] 21-01-PLAN.md — Copilot hook adapter + response translation
- [x] 21-02-PLAN.md — Copilot hook runner subcommand
- [x] 21-03-PLAN.md — Copilot hook install + doctor + wrapper scripts + docs
- [x] 21-04-PLAN.md — VS Code Copilot chat participant + LM summary

### Phase 22: Tool Name Canonicalization Across Platforms

**Goal:** Normalize platform-specific tool names to Claude Code's PascalCase canonical names at adapter ingestion time, so rules with `tools:` matchers work identically across all platforms
**Depends on:** Phase 21
**Plans:** 2 plans

Plans:
- [x] 22-01-PLAN.md — Fix adapter compile errors, correct Gemini mappings, update all adapter tests
- [x] 22-02-PLAN.md — Create TOOL-MAPPING.md cross-platform reference documentation

---

## ✅ v1.9 Multi-CLI E2E Testing (Partial)

**Milestone Goal:** Headless multi-CLI E2E/UAT test harness validating real integration behavior across 5 agent CLIs. Each CLI gets its own phase with full E2E testing. RuleZ-only scope.

**Shared context:** [E2E-CONTEXT.md](phases/e2e-multi-cli-harness/E2E-CONTEXT.md)

**Note:** Phases 24, 26, 27 moved to v2.1 milestone for continued E2E work.

### Phase 23: Claude Code CLI E2E Testing
**Goal**: Establish the E2E test harness framework + Claude Code scenarios (install, hook-fire, deny, inject)
**Depends on**: Phase 22
**Success Criteria** (what must be TRUE):
  1. E2E harness framework exists at `e2e/` with isolated workspace management
  2. `task e2e` entry point runs all scenarios and produces reports
  3. Claude Code passes all 4 core scenarios (install, hook-fire, deny, inject)
  4. Console ASCII table, JUnit XML, and Markdown summary reports generated
**Plans**: 2 plans

Plans:
- [x] 23-01-PLAN.md — E2E harness framework (workspace isolation, assertions, reporting, Taskfile integration)
- [x] 23-02-PLAN.md — Claude Code adapter, fixtures, and 4 E2E scenarios (install, hook-fire, deny, inject)

### Phase 24: Gemini CLI E2E Testing
**Goal**: Add Gemini CLI adapter + scenarios to the existing E2E harness
**Depends on**: Phase 23
**Success Criteria** (what must be TRUE):
  1. Gemini CLI passes all 4 core scenarios (install, hook-fire, deny, inject)
  2. Headless invocation works reliably in CI
  3. Reports include Gemini row in CLI x scenario matrix
**Plans**: 2 plans

Plans:
- [ ] 24-01-PLAN.md — Gemini adapter library, fixtures, and run.sh integration
- [ ] 24-02-PLAN.md — 4 Gemini E2E scenarios (install, hook-fire, deny, inject)

### Phase 25: Copilot CLI E2E Testing
**Goal**: Add Copilot CLI adapter + scenarios to the existing E2E harness
**Depends on**: Phase 23
**Success Criteria** (what must be TRUE):
  1. Copilot CLI passes all 4 core scenarios (install, hook-fire, deny, inject)
  2. Headless invocation works reliably in CI
  3. Reports include Copilot row in CLI x scenario matrix
**Plans**: 3 plans (2 original + 1 gap closure)

Plans:
- [x] 25-01-PLAN.md — Copilot adapter library, fixtures, and run.sh integration
- [x] 25-02-PLAN.md — 4 Copilot E2E scenarios (install, hook-fire, deny, inject)
- [x] 25-03-PLAN.md — Auth check gap closure: add authentication verification to copilot_adapter_check

### Phase 26: OpenCode CLI E2E Testing
**Goal**: Add OpenCode CLI adapter + scenarios to the existing E2E harness
**Depends on**: Phase 23
**Success Criteria** (what must be TRUE):
  1. OpenCode CLI passes all 4 core scenarios (install, hook-fire, deny, inject)
  2. Headless invocation works reliably in CI
  3. Reports include OpenCode row in CLI x scenario matrix
**Plans**: TBD

### Phase 27: Codex CLI E2E Testing
**Goal**: Add Codex CLI adapter + scenarios (NO hooks support — limited scenario set)
**Depends on**: Phase 23
**Success Criteria** (what must be TRUE):
  1. Codex CLI passes available scenarios (hooks NOT supported — hook scenarios skipped, not failed)
  2. Headless invocation works reliably in CI
  3. Reports include Codex row with skip markers for unsupported scenarios
**Plans**: 1 plan

Plans:
- [ ] 27-01-PLAN.md — Codex adapter, fixtures, 4 scenarios (install + 3 skip stubs), and run.sh integration

---

| 23. Claude Code CLI E2E Testing | v1.9 | 2/2 | ✅ Complete | 2026-02-23 |
| 25. Copilot CLI E2E Testing | v1.9 | 3/3 | ✅ Complete | 2026-02-23 |
| 24. Gemini CLI E2E Testing | v2.1 | 0/2 | Pending | - |
| 26. OpenCode CLI E2E Testing | v2.1 | 0/TBD | Pending | - |
| 27. Codex CLI E2E Testing | v2.1 | 0/1 | Pending | - |

| 28. RuleZ Cleanup and Hardening | v2.0 | 8/8 | ✅ Complete | 2026-03-05 |

### Phase 28: RuleZ Cleanup and Hardening
**Goal**: Fix critical bugs, improve engine performance, update skill docs, and add auto-upgrade capability — addresses all 9 pending todos
**Depends on**: Phase 22 (no dependency on E2E phases 23-27)
**Success Criteria** (what must be TRUE):
  1. Invalid regex in command_match returns non-match (not silent match-all) and validates at startup
  2. Config cache invalidated when hooks.yaml changes (timestamp or CRC check)
  3. `rulez debug` exercises run action scripts identically to live hook path
  4. tool_input fields exposed in enabled_when eval context (source, command, path, etc.)
  5. Naive matchers replaced with globset crate for correct glob patterns
  6. Regex compilation cached with LRU + config cached with file-change invalidation
  7. Parallel rule evaluation available for large rule sets
  8. Log filtering offloaded to Web Worker or Rust command
  9. mastering-hooks skill docs use correct field names matching RuleZ binary schema
  10. `rulez upgrade` or equivalent auto-checks and upgrades binary to latest release
**Plans**: 8 plans

Plans:
- [x] 28-01-PLAN.md — Regex fail-closed fix: 5 call sites in hooks.rs + debug.rs + command_match validation in config.rs
- [x] 28-02-PLAN.md — Fix mastering-hooks skill docs: 7 field name mismatches in hooks-yaml-schema.md + rule-patterns.md
- [x] 28-03-PLAN.md — tool_input fields in enabled_when eval context + mtime-based config cache
- [x] 28-04-PLAN.md — Fix debug run script trace: enrich JSON evaluations with action results
- [x] 28-05-PLAN.md — Replace naive contains() directory matching with globset crate
- [x] 28-06-PLAN.md — rulez upgrade subcommand using self_update crate
- [x] 28-07-PLAN.md — UI log filter debounce (200ms) in rulez-ui logStore + LogViewer
- [x] 28-08-PLAN.md — Parallel rule evaluation using tokio join_all for large rule sets

---

## ✅ v2.0 RuleZ Cleanup and Hardening (Complete)

**Milestone Goal:** Fix critical bugs, improve engine performance, update skill docs, and add auto-upgrade capability — addresses all 9 pending todos.

Phase 28 complete — all 8 plans executed across 4 waves. See Phase 28 section above.

---

## 🔄 v2.1 Multi-CLI E2E Testing (Continued)

**Milestone Goal:** Complete remaining CLI E2E testing for Gemini, OpenCode, and Codex.

Phases 24, 26, 27 moved from v1.9 — see phase details above.

---

*Created 2026-02-06 — Updated 2026-03-06 Phase 27 planned: 1 plan for Codex CLI E2E testing*

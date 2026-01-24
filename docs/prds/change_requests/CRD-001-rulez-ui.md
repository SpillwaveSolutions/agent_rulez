# Change Request Document: RuleZ UI

**CRD ID:** CRD-001
**Title:** RuleZ UI - Native Desktop Application for CCH Configuration
**Status:** Approved for Implementation
**Created:** 2026-01-24
**Author:** Claude Code Assistant
**Priority:** P1 (High)

---

## 1. Executive Summary

### 1.1 Change Description
Introduce RuleZ UI, a native desktop application built with Tauri 2.0 + React for visualizing, editing, validating, and debugging Claude Context Hooks (CCH) configurations.

### 1.2 Business Justification
- **Problem:** CCH users must manually edit YAML files without visual feedback, validation, or debugging tools
- **Solution:** Native desktop app providing visual editing, real-time validation, and debug simulation
- **Value:** Faster configuration, fewer errors, safer testing without running Claude Code

### 1.3 Scope
- **In Scope:** Phase 1 MVP (editor, validation, files, simulator, tree view, theming, tests)
- **Out of Scope:** Log viewer (Phase 2), rule templates (Phase 3), auto-update (Phase 4)

---

## 2. Technical Specification

### 2.1 Technology Stack

| Layer | Technology | Version |
|-------|------------|---------|
| Runtime | Bun | ^1.1 |
| Frontend | React + TypeScript | 18.x / 5.4+ |
| Styling | Tailwind CSS | 4.x |
| Editor | Monaco Editor + monaco-yaml | Latest |
| Desktop | Tauri | 2.0 |
| State | Zustand + TanStack Query | 5.x |
| Linting | Biome | Latest |
| Testing | Bun test + Playwright | Latest |

### 2.2 Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         RuleZ UI                                 │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                    React Frontend (Bun)                      ││
│  │  ┌───────────┐ ┌───────────┐ ┌───────────┐ ┌─────────────┐  ││
│  │  │  Monaco   │ │  Zustand  │ │ TanStack  │ │  Tailwind   │  ││
│  │  │  Editor   │ │  Stores   │ │   Query   │ │    CSS 4    │  ││
│  │  └───────────┘ └───────────┘ └───────────┘ └─────────────┘  ││
│  └─────────────────────────────────────────────────────────────┘│
│                              │ Tauri IPC                         │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                    Rust Backend (Tauri 2.0)                  ││
│  │  ┌───────────┐ ┌───────────┐ ┌───────────┐ ┌─────────────┐  ││
│  │  │  Config   │ │   Debug   │ │ Validate  │ │    File     │  ││
│  │  │ Commands  │ │ Commands  │ │ Commands  │ │   Watcher   │  ││
│  │  └───────────┘ └───────────┘ └───────────┘ └─────────────┘  ││
│  └─────────────────────────────────────────────────────────────┘│
│                              │                                   │
│  ┌───────────────────┐ ┌───────────────────┐                    │
│  │   File System     │ │    CCH Binary     │                    │
│  │ ~/.claude/        │ │   cch debug       │                    │
│  │ .claude/          │ │   cch validate    │                    │
│  └───────────────────┘ └───────────────────┘                    │
└─────────────────────────────────────────────────────────────────┘
```

### 2.3 Directory Structure

```
rulez_ui/
├── src/                          # React frontend
│   ├── components/
│   │   ├── editor/               # YamlEditor, RuleTreeView, ValidationPanel
│   │   ├── simulator/            # DebugSimulator, EventForm, ResultView
│   │   ├── files/                # FileSidebar, FileTab, FileTabBar
│   │   ├── layout/               # AppShell, Header, Sidebar, StatusBar
│   │   └── ui/                   # Button, Input, Select, ThemeToggle
│   ├── hooks/                    # useTauri, useConfig, useValidation
│   ├── lib/                      # tauri.ts, schema.ts, yaml-utils.ts
│   ├── stores/                   # configStore, editorStore, uiStore
│   ├── types/                    # TypeScript types
│   └── styles/                   # globals.css, monaco-theme.ts
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── commands/             # config.rs, debug.rs, validate.rs
│   │   └── watchers/             # file_watcher.rs
│   └── Cargo.toml
├── tests/                        # Playwright E2E tests
├── public/schema/                # JSON Schema for hooks.yaml
└── package.json                  # Bun manifest
```

### 2.4 Tauri IPC Commands

| Command | Purpose | Parameters | Returns |
|---------|---------|------------|---------|
| `list_config_files` | List configs | `project_dir?: string` | `Vec<ConfigFile>` |
| `read_config` | Read file | `path: string` | `string` |
| `write_config` | Write file | `path, content: string` | `()` |
| `run_debug` | Execute CCH debug | `event_type, tool?, command?, path?` | `DebugResult` |
| `validate_config` | Validate config | `path: string` | `ValidationResult` |

---

## 3. User Stories

### US-RUI-01: YAML Editor with Syntax Highlighting
**As a** CCH user
**I want to** edit my hooks.yaml file with syntax highlighting and autocomplete
**So that** I can write configurations faster and with fewer errors

**Acceptance Criteria:**
- Monaco editor loads with YAML syntax highlighting
- Autocomplete suggests valid rule fields
- Tab completion works for nested structures
- Undo/redo with Cmd/Ctrl+Z
- Line numbers displayed
- Code folding for rules/sections

---

### US-RUI-02: Real-time Schema Validation
**As a** CCH user
**I want to** see validation errors inline as I type
**So that** I can fix problems immediately without running external commands

**Acceptance Criteria:**
- Invalid YAML syntax shows red squiggly underlines
- Schema violations show with descriptive error messages
- Errors panel lists all issues with line numbers
- Click error to jump to line in editor
- Validation runs within 200ms of typing

---

### US-RUI-03: Multi-file Configuration Management
**As a** developer
**I want to** view and edit both my global and project-level hooks.yaml files
**So that** I can manage configurations in one place

**Acceptance Criteria:**
- File sidebar shows global (~/.claude/hooks.yaml) config
- File sidebar shows project (.claude/hooks.yaml) config
- Tab bar allows multiple files open simultaneously
- Modified files show indicator (asterisk or dot)
- Save with Cmd/Ctrl+S
- Create new hooks.yaml if not exists

---

### US-RUI-04: Debug Simulation
**As a** CCH user
**I want to** simulate events and see which rules match without running Claude Code
**So that** I can test my configuration safely

**Acceptance Criteria:**
- Select event type (PreToolUse, PostToolUse, etc.) from dropdown
- Enter tool name, command, and path
- Click "Simulate" to run debug via CCH binary
- See matched rules with evaluation trace
- See final outcome (Allow/Block/Inject)
- Display execution time

---

### US-RUI-05: Rule Tree Visualization
**As a** team lead
**I want to** see a visual tree of all configured rules
**So that** I can quickly audit what hooks are active

**Acceptance Criteria:**
- Tree view shows rules grouped by category
- Each rule shows: name, tools, action type, enabled status
- Click rule to jump to its location in editor
- Toggle switch to enable/disable rules (updates YAML)
- Collapsible sections for settings and rules

---

### US-RUI-06: Theme Support
**As a** user
**I want** the application to respect my system's dark/light mode preference
**So that** it's comfortable to use in any lighting condition

**Acceptance Criteria:**
- Detects system preference on launch
- Manual toggle in header (sun/moon icon)
- Monaco editor theme changes with app theme
- Preference persisted across sessions

---

## 4. Implementation Plan

### 4.1 Milestone Overview

| # | Milestone | Days | Description |
|---|-----------|------|-------------|
| M1 | Project Setup | 1 | Tauri + React + Bun scaffold, dual-mode, CI |
| M2 | Monaco Editor | 1 | YAML editor with syntax highlighting |
| M3 | Schema Validation | 2 | JSON Schema, inline errors, autocomplete |
| M4 | File Operations | 1 | Read/write, global + project configs |
| M5 | Rule Tree View | 1 | Visual tree, navigation |
| M6 | Debug Simulator | 2 | Event form, CCH integration, trace |
| M7 | Theming | 0.5 | Dark/light, system preference |
| M8 | Playwright Tests | 1 | E2E suite, CI integration |

**Total: 9.5 days**

### 4.2 Milestone Dependency Graph

```
M1 (Project Setup)
 │
 ├──────────────────────────────────────────────────────┐
 │                                                      │
 ▼                                                      ▼
M2 (Monaco Editor) ───────────► M3 (Schema Validation)  M7 (Theming)
 │                                    │                  │
 │                                    ▼                  │
 │                              M4 (File Ops)            │
 │                                    │                  │
 │                                    ▼                  │
 ├──────────────────────────────► M5 (Tree View) ◄──────┤
 │                                    │                  │
 │                                    ▼                  │
 └──────────────────────────────► M6 (Simulator) ◄──────┘
                                      │
                                      ▼
                                 M8 (Tests)
```

### 4.3 Task Breakdown

#### M1: Project Setup (1 day)

| Task ID | Description | Hours |
|---------|-------------|-------|
| M1-T01 | Initialize Tauri + React project with Bun | 3 |
| M1-T02 | Configure dual-mode architecture | 2 |
| M1-T03 | Set up CI workflow | 3 |

#### M2: Monaco Editor (1 day)

| Task ID | Description | Hours |
|---------|-------------|-------|
| M2-T01 | Integrate Monaco Editor | 3 |
| M2-T02 | Add editor features (save, undo/redo) | 2 |
| M2-T03 | Create editor toolbar | 3 |

#### M3: Schema Validation (2 days)

| Task ID | Description | Hours |
|---------|-------------|-------|
| M3-T01 | Create JSON Schema for hooks.yaml | 4 |
| M3-T02 | Integrate monaco-yaml | 4 |
| M3-T03 | Implement autocomplete | 4 |
| M3-T04 | Create ValidationPanel | 4 |

#### M4: File Operations (1 day)

| Task ID | Description | Hours |
|---------|-------------|-------|
| M4-T01 | Implement Tauri file commands | 3 |
| M4-T02 | Create config store with Zustand | 2 |
| M4-T03 | Create FileSidebar | 2 |
| M4-T04 | Create FileTabBar | 1 |

#### M5: Rule Tree View (1 day)

| Task ID | Description | Hours |
|---------|-------------|-------|
| M5-T01 | Create RuleTreeView component | 3 |
| M5-T02 | Implement rule cards | 2 |
| M5-T03 | Implement navigation | 3 |

#### M6: Debug Simulator (2 days)

| Task ID | Description | Hours |
|---------|-------------|-------|
| M6-T01 | Implement CCH debug Tauri command | 4 |
| M6-T02 | Create EventForm | 3 |
| M6-T03 | Create ResultView | 2 |
| M6-T04 | Create EvaluationTrace | 3 |
| M6-T05 | Integrate simulator | 4 |

#### M7: Theming (0.5 day)

| Task ID | Description | Hours |
|---------|-------------|-------|
| M7-T01 | Implement theme system | 1 |
| M7-T02 | Create ThemeToggle | 1 |
| M7-T03 | Configure Monaco themes | 1 |
| M7-T04 | Style Tailwind for themes | 1 |

#### M8: Playwright Tests (1 day)

| Task ID | Description | Hours |
|---------|-------------|-------|
| M8-T01 | Set up Playwright | 2 |
| M8-T02 | Write editor tests | 2 |
| M8-T03 | Write simulator tests | 2 |
| M8-T04 | Write file operation tests | 1 |
| M8-T05 | Configure CI | 1 |

---

## 5. Performance Requirements

| Metric | Target |
|--------|--------|
| App launch (cold start) | < 2 seconds |
| File load (10KB YAML) | < 100ms |
| Validation response | < 200ms |
| Debug simulation | < 500ms (excl. CCH) |
| Editor input latency | < 16ms (60fps) |
| Memory usage (idle) | < 150MB |

---

## 6. Quality Gates

### 6.1 Pre-Commit Checks
```bash
cd rulez_ui
bun run lint        # Biome linting
bun run typecheck   # TypeScript type check
bun run test        # Bun unit tests
```

### 6.2 CI Pipeline
- All unit tests pass (Bun test)
- All E2E tests pass (Playwright)
- TypeScript compiles without errors
- Biome reports no errors
- Build succeeds for all platforms

### 6.3 Definition of Done (per task)
- Code complete and compiles
- Unit tests written (if applicable)
- Component renders correctly
- Works in both Tauri and web mode
- Linting passes
- Type checking passes
- Reviewed in PR

---

## 7. Platform Support

| Platform | Architecture | Format |
|----------|--------------|--------|
| macOS | Intel (x86_64) | .dmg, .app |
| macOS | Apple Silicon (aarch64) | .dmg, .app |
| Linux | x86_64 | .deb, .AppImage |
| Linux | ARM64 | .deb, .AppImage |
| Windows | x86_64 | .msi, .exe |

---

## 8. Dependencies

### 8.1 CCH Binary Requirements
- `cch debug` command with `--json` flag
- `cch validate` command with `--json` flag
- `cch --version` for version check

### 8.2 Runtime Dependencies
- CCH binary in PATH (for debug simulator and validation)
- No other runtime dependencies (Tauri bundles everything)

---

## 9. Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Monaco bundle size too large | Medium | Medium | Use worker, code split |
| Tauri IPC latency | Low | Medium | Async commands, caching |
| CCH binary not in PATH | High | High | Clear error message, help |
| Cross-platform path issues | Medium | Medium | Path normalization |

---

## 10. Success Metrics

| Metric | Target |
|--------|--------|
| Validation errors caught before save | 95%+ |
| Debug simulations match actual CCH | 100% |
| Crash-free sessions | > 99% |
| E2E test pass rate | 100% |

---

## 11. Future Roadmap

### Phase 2: Log Viewer (5-7 days)
- Real-time log streaming via file watcher
- Virtual scrolling for large files
- Filter by level, rule, timestamp

### Phase 3: Advanced Features (7-10 days)
- Rule templates library
- Import/export configurations
- Regex pattern tester
- Context file preview

### Phase 4: Distribution (3-5 days)
- Platform installers (DMG, MSI, DEB)
- Auto-update with Tauri updater
- Onboarding tutorial

---

## 12. Approvals

| Role | Name | Date | Status |
|------|------|------|--------|
| Product Owner | | | Pending |
| Tech Lead | | | Pending |
| QA Lead | | | Pending |

---

## 13. Related Documents

| Document | Location |
|----------|----------|
| PRD | `docs/prds/rulez_ui_prd.md` |
| Plan | `docs/plans/rulez_ui_plan.md` |
| Spec | `.speckit/features/rulez-ui/spec.md` |
| Tasks | `.speckit/features/rulez-ui/tasks.md` |
| Implementation Plan | `.speckit/features/rulez-ui/plan.md` |

---

## 14. GitHub Issues

All 31 tasks have been created as GitHub issues:
- Issues #7-#9: M1 (Project Setup)
- Issues #10-#12: M2 (Monaco Editor)
- Issues #13-#16: M3 (Schema Validation)
- Issues #17-#20: M4 (File Operations)
- Issues #21-#23: M5 (Rule Tree View)
- Issues #24-#28: M6 (Debug Simulator)
- Issues #29-#32: M7 (Theming)
- Issues #33-#37: M8 (Playwright Tests)

View all issues: `gh issue list --label "feature:rulez-ui"`

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-01-24 | Claude Code | Initial CRD |

# RuleZ UI Feature Specification

**Feature ID:** rulez-ui
**Status:** Specified
**Created:** 2026-01-24
**Sources:** docs/plans/rulez_ui_plan.md, docs/prds/rulez_ui_prd.md

---

## Overview

RuleZ UI is a native desktop application built with Tauri 2.0 + React for visualizing, editing, validating, and debugging Claude Context Hooks (CCH) configurations. It provides a visual alternative to manual YAML editing with real-time validation and debug simulation.

### Design Philosophy
- **Native Desktop First**: The primary deliverable is a cross-platform desktop app
- **Web Mode for Testing**: Browser mode exists solely for Playwright E2E testing
- **Bun-Powered**: All TypeScript/React operations use Bun for performance
- **CCH Integration**: Uses actual CCH binary for debug simulation (not mocks in production)

---

## User Stories

### US-RUI-01: YAML Editor with Syntax Highlighting
**As a** CCH user
**I want to** edit my hooks.yaml file with syntax highlighting and autocomplete
**So that** I can write configurations faster and with fewer errors

**Acceptance Criteria:**
- [ ] Monaco editor loads with YAML syntax highlighting
- [ ] Autocomplete suggests valid rule fields (name, matchers, actions, etc.)
- [ ] Tab completion works for nested structures
- [ ] Undo/redo with Cmd/Ctrl+Z
- [ ] Line numbers displayed
- [ ] Code folding for rules/sections

---

### US-RUI-02: Real-time Schema Validation
**As a** CCH user
**I want to** see validation errors inline as I type
**So that** I can fix problems immediately without running external commands

**Acceptance Criteria:**
- [ ] Invalid YAML syntax shows red squiggly underlines
- [ ] Schema violations show with descriptive error messages
- [ ] Errors panel lists all issues with line numbers
- [ ] Click error to jump to line in editor
- [ ] Validation runs within 200ms of typing

---

### US-RUI-03: Multi-file Configuration Management
**As a** developer
**I want to** view and edit both my global and project-level hooks.yaml files
**So that** I can manage configurations in one place

**Acceptance Criteria:**
- [ ] File sidebar shows global (~/.claude/hooks.yaml) config
- [ ] File sidebar shows project (.claude/hooks.yaml) config
- [ ] Tab bar allows multiple files open simultaneously
- [ ] Modified files show indicator (asterisk or dot)
- [ ] Save with Cmd/Ctrl+S
- [ ] Create new hooks.yaml if not exists

---

### US-RUI-04: Debug Simulation
**As a** CCH user
**I want to** simulate events and see which rules match without running Claude Code
**So that** I can test my configuration safely

**Acceptance Criteria:**
- [ ] Select event type (PreToolUse, PostToolUse, etc.) from dropdown
- [ ] Enter tool name, command, and path
- [ ] Click "Simulate" to run debug via CCH binary
- [ ] See matched rules with evaluation trace
- [ ] See final outcome (Allow/Block/Inject)
- [ ] Display execution time

---

### US-RUI-05: Rule Tree Visualization
**As a** team lead
**I want to** see a visual tree of all configured rules
**So that** I can quickly audit what hooks are active

**Acceptance Criteria:**
- [ ] Tree view shows rules grouped by category
- [ ] Each rule shows: name, tools, action type, enabled status
- [ ] Click rule to jump to its location in editor
- [ ] Toggle switch to enable/disable rules (updates YAML)
- [ ] Collapsible sections for settings and rules

---

### US-RUI-06: Theme Support
**As a** user
**I want** the application to respect my system's dark/light mode preference
**So that** it's comfortable to use in any lighting condition

**Acceptance Criteria:**
- [ ] Detects system preference on launch
- [ ] Manual toggle in header (sun/moon icon)
- [ ] Monaco editor theme changes with app theme
- [ ] Preference persisted across sessions

---

## Technical Architecture

### Technology Stack

| Layer | Technology |
|-------|------------|
| Runtime | Bun (all TypeScript/React operations) |
| Frontend | React 18 + TypeScript + Tailwind CSS 4 |
| Editor | Monaco Editor + monaco-yaml |
| Desktop | Tauri 2.0 (Rust backend) |
| State | Zustand + TanStack Query |
| Linting | Biome (replaces ESLint + Prettier) |
| Testing | Bun test (unit) + Playwright (E2E) |

### Directory Structure

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

### Tauri IPC Commands

| Command | Purpose | Parameters |
|---------|---------|------------|
| `list_config_files` | List global and project configs | `project_dir?: string` |
| `read_config` | Read config file content | `path: string` |
| `write_config` | Write config file content | `path: string, content: string` |
| `run_debug` | Execute CCH debug command | `hook_event_name, tool?, command?, path?` |
| `validate_config` | Validate config via CCH | `path: string` |

---

## Performance Requirements

| Metric | Target |
|--------|--------|
| App launch (cold start) | < 2 seconds |
| File load (10KB YAML) | < 100ms |
| Validation response | < 200ms |
| Debug simulation | < 500ms (excl. CCH) |
| Editor input latency | < 16ms (60fps) |
| Memory usage (idle) | < 150MB |

---

## Quality Gates

### Pre-Commit Checks
```bash
cd rulez_ui
bun run lint        # Biome linting
bun run typecheck   # TypeScript type check
bun run test        # Bun unit tests
```

### CI Pipeline
- All unit tests pass (Bun test)
- All E2E tests pass (Playwright)
- TypeScript compiles without errors
- Biome reports no errors
- Build succeeds for all platforms

---

## Implementation Phases

### Phase 1: MVP (9.5 days estimated)

| Milestone | Description | Days |
|-----------|-------------|------|
| M1 | Project Setup (Tauri + React + Bun scaffold) | 1 |
| M2 | Monaco Editor (YAML syntax highlighting) | 1 |
| M3 | Schema Validation (JSON Schema, inline errors) | 2 |
| M4 | File Operations (read/write, global + project) | 1 |
| M5 | Rule Tree View (visual tree, navigation) | 1 |
| M6 | Debug Simulator (event form, CCH integration) | 2 |
| M7 | Theming (dark/light, system preference) | 0.5 |
| M8 | Playwright Tests (E2E suite, CI) | 1 |

### Phase 2: Log Viewer (5-7 days)
- Real-time log streaming
- Virtual scrolling
- Filtering and search

### Phase 3: Advanced Features (7-10 days)
- Rule templates library
- Import/export configurations
- Regex tester

### Phase 4: Distribution (3-5 days)
- Installers (DMG, MSI, DEB)
- Auto-update mechanism

---

## Platform Support

| Platform | Architecture | Format |
|----------|--------------|--------|
| macOS | Intel (x86_64) | .dmg, .app |
| macOS | Apple Silicon (aarch64) | .dmg, .app |
| Linux | x86_64 | .deb, .AppImage |
| Linux | ARM64 | .deb, .AppImage |
| Windows | x86_64 | .msi, .exe |

---

## Dependencies

### CCH Binary Requirements
- `cch debug` command with `--json` flag for structured output
- `cch validate` command with `--json` flag for structured output
- `cch --version` for version check

### Runtime Dependencies
- CCH binary in PATH (for debug simulator and validation)
- No other runtime dependencies (Tauri bundles everything)

---

## Open Questions

| Question | Status | Decision |
|----------|--------|----------|
| Support multiple config file formats? | Resolved | YAML only for Phase 1 |
| Mock CCH or call actual binary? | Resolved | Call actual CCH binary |
| Include rule creation wizard? | Deferred | Phase 3 |
| Log viewer in Phase 1 or 2? | Resolved | Phase 2 |

---

## Success Metrics

| Metric | Target |
|--------|--------|
| Validation errors caught before save | 95%+ |
| Debug simulations match actual CCH | 100% |
| Crash-free sessions | > 99% |
| E2E test pass rate | 100% |

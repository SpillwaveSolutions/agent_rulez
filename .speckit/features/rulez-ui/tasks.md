# RuleZ UI Implementation Tasks

**Feature ID:** rulez-ui
**Status:** M1 Complete, M2-M8 Pending
**Total Estimated Days:** 9.5 (Phase 1 MVP)
**PR:** #72 (merged to develop)
**Last Updated:** 2026-01-25

---

## Milestone 1: Project Setup (1 day) - COMPLETE

### M1-T01: Initialize Tauri + React project with Bun
- [x] Create `rulez_ui` directory at project root
- [x] Initialize Bun project: `bun init`
- [x] Add Tauri 2.0: `bunx create-tauri-app`
- [x] Configure TypeScript with strict mode
- [x] Set up Tailwind CSS 4
- [x] Configure Biome for linting/formatting
- [x] Create basic directory structure

### M1-T02: Configure dual-mode architecture
- [x] Create `src/lib/tauri.ts` with `isTauri()` detection
- [x] Implement web fallback pattern for all Tauri commands
- [x] Add mock data module for browser testing
- [x] Verify HMR works in both modes

### M1-T03: Set up CI workflow
- [x] Create `.github/workflows/rulez-ui.yml`
- [x] Configure Bun installation
- [x] Add lint, typecheck, test stages
- [x] Configure Tauri build for release artifacts

---

## Milestone 2: Monaco Editor (1 day)

### M2-T01: Integrate Monaco Editor
- [ ] Install `@monaco-editor/react`
- [ ] Create `YamlEditor.tsx` component
- [ ] Configure YAML language mode
- [ ] Add syntax highlighting
- [ ] Implement line numbers

### M2-T02: Add editor features
- [ ] Implement Cmd/Ctrl+S save shortcut
- [ ] Add undo/redo support
- [ ] Enable code folding
- [ ] Configure editor options (font, theme)

### M2-T03: Create editor toolbar
- [ ] Create `EditorToolbar.tsx`
- [ ] Add Save button
- [ ] Add Format button
- [ ] Add Undo/Redo buttons
- [ ] Show file modified indicator

---

## Milestone 3: Schema Validation (2 days)

### M3-T01: Create JSON Schema
- [ ] Create `public/schema/hooks-schema.json`
- [ ] Define version, settings, rules structure
- [ ] Add matchers and actions definitions
- [ ] Include all CCH field types
- [ ] Add descriptions for autocomplete

### M3-T02: Integrate monaco-yaml
- [ ] Install `monaco-yaml`
- [ ] Configure schema validation
- [ ] Map schema to hooks.yaml files
- [ ] Enable inline error markers (red squiggles)

### M3-T03: Implement autocomplete
- [ ] Configure JSON Schema autocomplete
- [ ] Add custom completion providers if needed
- [ ] Test completion for all rule fields
- [ ] Verify nested structure completion

### M3-T04: Create ValidationPanel
- [ ] Create `ValidationPanel.tsx`
- [ ] Display errors with line numbers
- [ ] Display warnings
- [ ] Implement click-to-jump-to-line
- [ ] Show error count in status bar

---

## Milestone 4: File Operations (1 day)

### M4-T01: Implement Tauri file commands
- [ ] Create `src-tauri/src/commands/config.rs`
- [ ] Implement `list_config_files` command
- [ ] Implement `read_config` command
- [ ] Implement `write_config` command
- [ ] Register commands in Tauri app

### M4-T02: Create config store
- [ ] Create `src/stores/configStore.ts` with Zustand
- [ ] Implement `loadConfig` action
- [ ] Implement `saveConfig` action
- [ ] Track file modified state
- [ ] Handle multiple open files

### M4-T03: Create FileSidebar
- [ ] Create `FileSidebar.tsx`
- [ ] Display global config path
- [ ] Display project config path
- [ ] Show file exists/not exists indicator
- [ ] Handle file selection

### M4-T04: Create FileTabBar
- [ ] Create `FileTabBar.tsx` and `FileTab.tsx`
- [ ] Support multiple open tabs
- [ ] Show modified indicator
- [ ] Implement tab close with save prompt

---

## Milestone 5: Rule Tree View (1 day)

### M5-T01: Create RuleTreeView component
- [ ] Create `RuleTreeView.tsx`
- [ ] Parse YAML to tree structure
- [ ] Display settings section
- [ ] Display rules section
- [ ] Add collapsible sections

### M5-T02: Implement rule cards
- [ ] Show rule name prominently
- [ ] Display tools list
- [ ] Show action type badge (Block/Inject/Run)
- [ ] Add enabled/disabled toggle

### M5-T03: Implement navigation
- [ ] Click rule to jump to editor line
- [ ] Sync tree selection with cursor position
- [ ] Highlight currently selected rule

---

## Milestone 6: Debug Simulator (2 days)

### M6-T01: Implement CCH debug command
- [ ] Create `src-tauri/src/commands/debug.rs`
- [ ] Execute `cch debug` with parameters
- [ ] Parse JSON output
- [ ] Return structured result
- [ ] Handle errors gracefully

### M6-T02: Create EventForm
- [ ] Create `EventForm.tsx`
- [ ] Add event type dropdown (all 7 types)
- [ ] Add tool name input with suggestions
- [ ] Add command input
- [ ] Add path input (optional)

### M6-T03: Create ResultView
- [ ] Create `ResultView.tsx`
- [ ] Display outcome (Allow/Block/Inject)
- [ ] Display reason if blocked
- [ ] Show execution time

### M6-T04: Create EvaluationTrace
- [ ] Create `EvaluationTrace.tsx`
- [ ] List all rules evaluated
- [ ] Show match status per rule
- [ ] Display evaluation time per rule
- [ ] Show match details (pattern, input)

### M6-T05: Integrate simulator
- [ ] Create `DebugSimulator.tsx` container
- [ ] Wire up Tauri command invocation
- [ ] Implement web fallback with mock data
- [ ] Add loading state

---

## Milestone 7: Theming (0.5 day)

### M7-T01: Implement theme system
- [ ] Create `src/stores/uiStore.ts`
- [ ] Detect system preference on load
- [ ] Implement manual toggle
- [ ] Persist preference to localStorage

### M7-T02: Create theme toggle
- [ ] Create `ThemeToggle.tsx`
- [ ] Add sun/moon icons
- [ ] Wire up to uiStore

### M7-T03: Configure Monaco themes
- [ ] Create `src/styles/monaco-theme.ts`
- [ ] Define light theme
- [ ] Define dark theme
- [ ] Switch theme with app theme

### M7-T04: Style Tailwind for themes
- [ ] Configure CSS variables for colors
- [ ] Implement dark mode classes
- [ ] Test all components in both themes

---

## Milestone 8: Playwright Tests (1 day)

### M8-T01: Set up Playwright
- [ ] Install Playwright: `bun add -d @playwright/test`
- [ ] Create `playwright.config.ts`
- [ ] Configure web server for testing
- [ ] Add test fixtures

### M8-T02: Write editor tests
- [ ] Test editor loads with content
- [ ] Test validation error display
- [ ] Test autocomplete functionality
- [ ] Test save keyboard shortcut

### M8-T03: Write simulator tests
- [ ] Test event form inputs
- [ ] Test simulation execution
- [ ] Test result display
- [ ] Test evaluation trace

### M8-T04: Write file operation tests
- [ ] Test file sidebar display
- [ ] Test tab management
- [ ] Test save confirmation

### M8-T05: Configure CI
- [ ] Add Playwright to CI workflow
- [ ] Configure test artifacts
- [ ] Generate test reports

---

## Definition of Done (per task)

- [ ] Code complete and compiles
- [ ] Unit tests written (if applicable)
- [ ] Component renders correctly
- [ ] Works in both Tauri and web mode
- [ ] Linting passes (`bun run lint`)
- [ ] Type checking passes (`bun run typecheck`)
- [ ] Reviewed in PR

---

## Notes

### Development Commands
```bash
cd rulez_ui
bun install           # Install dependencies
bun run dev           # Start dev server (browser)
bun run dev:tauri     # Start dev server (Tauri desktop)
bun run lint          # Run Biome linter
bun run typecheck     # Run TypeScript check
bun run test          # Run Bun unit tests
bun run test:e2e      # Run Playwright E2E tests
bun run build:tauri   # Build desktop app
```

### Key Integration Points
- CCH binary must be in PATH for debug simulator
- Schema must match CCH's actual configuration format
- File paths must work cross-platform (Windows backslash handling)

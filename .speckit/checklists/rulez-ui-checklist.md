# RuleZ UI Quality Checklist

**Feature ID:** rulez-ui
**Generated:** 2026-01-24
**Status:** M1 Complete, M2-M8 In Progress
**PR:** #72 (merged to develop)
**Last Updated:** 2026-01-25

---

## Pre-Implementation Checklist

### Project Setup Readiness
- [x] Bun installed and working (`bun --version`)
- [x] Rust toolchain installed (`rustc --version`)
- [x] Tauri CLI installed (`cargo install tauri-cli`)
- [x] Node.js available as fallback (for some tooling)
- [ ] CCH binary built and in PATH (`cch --version`) - needed for M6

### Development Environment
- [x] VS Code or preferred IDE configured
- [x] TypeScript extension installed
- [x] Rust analyzer extension installed
- [x] Tailwind CSS IntelliSense configured
- [x] Biome extension for linting

---

## Milestone 1: Project Setup ✅

### M1 Deliverables
- [x] Tauri 2.0 + React 18 scaffold
- [x] TypeScript strict mode configured
- [x] Tailwind CSS 4 configured
- [x] Biome linting configured
- [x] Dual-mode architecture (Tauri/web)
- [x] `isTauri()` detection function
- [x] Mock data module for browser testing
- [x] Basic Zustand stores (config, editor, ui)
- [x] Component skeleton structure
- [x] CLAUDE.md and README.md documentation

---

## User Story Acceptance Checklists

### US-RUI-01: YAML Editor with Syntax Highlighting

#### Functional Requirements
- [ ] Monaco editor loads successfully
- [ ] YAML syntax highlighting applied
- [ ] Autocomplete suggests valid rule fields (name, matchers, actions)
- [ ] Tab completion works for nested structures
- [ ] Undo/redo with Cmd/Ctrl+Z works
- [ ] Line numbers displayed correctly
- [ ] Code folding for rules/sections works

#### Edge Cases
- [ ] Empty file loads without error
- [ ] Very large file (>1000 lines) loads within 500ms
- [ ] Invalid YAML doesn't crash editor
- [ ] Unicode characters render correctly
- [ ] Copy/paste preserves formatting

#### Performance
- [ ] Editor input latency < 16ms (60fps)
- [ ] Initial load < 500ms

---

### US-RUI-02: Real-time Schema Validation

#### Functional Requirements
- [ ] Invalid YAML syntax shows red squiggly underlines
- [ ] Schema violations show with descriptive error messages
- [ ] Errors panel lists all issues with line numbers
- [ ] Click error to jump to line in editor
- [ ] Validation runs within 200ms of typing

#### Edge Cases
- [ ] Validation handles partial YAML (typing in progress)
- [ ] Multiple errors displayed simultaneously
- [ ] Deeply nested validation errors show correct line
- [ ] Unknown fields flagged appropriately
- [ ] Empty rules array handled

#### Performance
- [ ] Debounced validation (not on every keystroke)
- [ ] Validation completes < 200ms for typical files

---

### US-RUI-03: Multi-file Configuration Management

#### Functional Requirements
- [ ] File sidebar shows global (~/.claude/hooks.yaml) config
- [ ] File sidebar shows project (.claude/hooks.yaml) config
- [ ] Tab bar allows multiple files open simultaneously
- [ ] Modified files show indicator (asterisk or dot)
- [ ] Save with Cmd/Ctrl+S works
- [ ] Create new hooks.yaml if not exists

#### Edge Cases
- [ ] Handle missing global config gracefully
- [ ] Handle missing project config gracefully
- [ ] Handle read-only files appropriately
- [ ] Prompt to save unsaved changes on close
- [ ] Handle file deleted externally while open

#### Permissions
- [ ] Request file system access correctly (Tauri)
- [ ] Handle permission denied errors

---

### US-RUI-04: Debug Simulation

#### Functional Requirements
- [ ] Select event type (PreToolUse, PostToolUse, etc.) from dropdown
- [ ] Enter tool name, command, and path
- [ ] Click "Simulate" to run debug via CCH binary
- [ ] See matched rules with evaluation trace
- [ ] See final outcome (Allow/Block/Inject)
- [ ] Display execution time

#### Edge Cases
- [ ] Handle CCH binary not found
- [ ] Handle CCH binary crash
- [ ] Handle timeout (>5s)
- [ ] Handle empty event form submission
- [ ] Handle special characters in command input

#### Integration
- [ ] CCH debug command invoked correctly
- [ ] JSON output parsed correctly
- [ ] Error messages from CCH displayed

---

### US-RUI-05: Rule Tree Visualization

#### Functional Requirements
- [ ] Tree view shows rules grouped by category
- [ ] Each rule shows: name, tools, action type, enabled status
- [ ] Click rule to jump to its location in editor
- [ ] Toggle switch to enable/disable rules (updates YAML)
- [ ] Collapsible sections for settings and rules

#### Edge Cases
- [ ] Handle rules without names
- [ ] Handle malformed rules gracefully
- [ ] Sync tree when YAML edited manually
- [ ] Handle very long rule names (truncation)

---

### US-RUI-06: Theme Support

#### Functional Requirements
- [ ] Detects system preference on launch
- [ ] Manual toggle in header (sun/moon icon)
- [ ] Monaco editor theme changes with app theme
- [ ] Preference persisted across sessions

#### Edge Cases
- [ ] Handle system preference change while app running
- [ ] Handle corrupted localStorage preference
- [ ] All UI components respect theme (not just editor)

---

## Technical Quality Checklists

### Code Quality
- [ ] All TypeScript files compile without errors
- [ ] No `any` types without justification
- [ ] Biome reports no linting errors
- [ ] All components have prop types defined
- [ ] Zustand stores properly typed

### Testing
- [ ] Unit tests for utility functions (80%+ coverage)
- [ ] Component tests for key UI elements
- [ ] E2E tests for all user stories
- [ ] All tests pass in CI

### Performance
- [ ] App launch < 2 seconds
- [ ] File load (10KB YAML) < 100ms
- [ ] Validation response < 200ms
- [ ] Editor input latency < 16ms
- [ ] Memory usage (idle) < 150MB

### Accessibility
- [ ] Keyboard navigation works
- [ ] Focus states visible
- [ ] Screen reader friendly labels
- [ ] Color contrast meets WCAG AA

### Security
- [ ] No arbitrary code execution
- [ ] File system access scoped to config directories
- [ ] No sensitive data logged
- [ ] XSS protection in webview

---

## Pre-Merge Checklist (Per PR)

### Code Review
- [ ] Self-review completed
- [ ] Code follows project conventions
- [ ] No console.log statements (use proper logging)
- [ ] No commented-out code

### Testing
- [ ] All existing tests pass
- [ ] New tests added for new functionality
- [ ] Manual testing completed

### Documentation
- [ ] README updated if needed
- [ ] Code comments for complex logic
- [ ] Type definitions documented

### CI/CD
- [ ] `bun run lint` passes
- [ ] `bun run typecheck` passes
- [ ] `bun run test` passes
- [ ] Build succeeds for all platforms

---

## Pre-Release Checklist (MVP)

### Functionality
- [ ] All 6 user stories acceptance criteria met
- [ ] All E2E tests pass
- [ ] Manual QA on each platform (macOS, Linux, Windows)

### Performance
- [ ] All performance targets met
- [ ] No memory leaks in 1-hour session
- [ ] Startup time verified on each platform

### Distribution
- [ ] macOS .dmg builds and installs correctly
- [ ] Linux .AppImage runs correctly
- [ ] Windows .msi installs correctly
- [ ] Code signing configured (if applicable)

### Documentation
- [ ] User guide written
- [ ] Installation instructions verified
- [ ] Known issues documented
- [ ] Changelog updated

---

## Regression Test Suite

### Critical Paths
1. [ ] Open app → Load file → Edit → Save → Close
2. [ ] Open app → Run simulation → View results
3. [ ] Open app → Toggle theme → Verify persistence
4. [ ] Open multiple files → Switch between tabs → Save all
5. [ ] Edit YAML with errors → See validation → Fix → Validate

### Error Scenarios
1. [ ] CCH binary missing → Graceful error message
2. [ ] Corrupted YAML → Load with error indicator
3. [ ] Network offline → App still works (local only)
4. [ ] File permission denied → Clear error message
5. [ ] Disk full → Save fails gracefully

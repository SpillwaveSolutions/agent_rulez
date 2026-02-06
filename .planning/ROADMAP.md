# RuleZ UI v1.0 Roadmap

**Milestone Goal:** Deliver a production-ready Tauri desktop application for visual CCH configuration management.

**Target:** RuleZ UI v1.0.0 release with full editor, validation, debugging, and theming capabilities.

---

## Milestone: RuleZ UI v1.0

### Completed Work (Prior to GSD)

**M1: Project Setup** - COMPLETE
- Tauri 2.0 + React 18 + Bun scaffold
- Basic AppShell layout with sidebar
- Tailwind CSS 4 integration
- PR #72 merged to develop

---

### Phase 1: Monaco Editor (M2)

**Goal:** Integrate Monaco Editor with YAML language support for hooks.yaml editing.

**Requirements:**
- Monaco Editor integration with @monaco-editor/react
- YAML syntax highlighting via monaco-yaml
- Custom CCH theme (dark/light variants)
- Editor toolbar (save, undo, redo)
- Keyboard shortcuts (Cmd+S save, Cmd+Z undo)

**Success Criteria:**
- [ ] Can open and edit .yaml files
- [ ] Syntax highlighting works correctly
- [ ] Save persists changes to filesystem

---

### Phase 2: Schema Validation (M3)

**Goal:** Real-time validation of hooks.yaml against CCH schema.

**Requirements:**
- JSON Schema for hooks.yaml structure
- Real-time validation as user types
- Error/warning markers in editor gutter
- ValidationPanel component showing all issues
- Hover tooltips for error details

**Success Criteria:**
- [ ] Invalid YAML shows syntax errors
- [ ] Invalid schema shows semantic errors
- [ ] Errors highlighted in editor with line markers

---

### Phase 3: File Operations (M4)

**Goal:** Multi-file configuration management (global + project configs).

**Requirements:**
- File sidebar showing available configs
- Tab-based multi-file editing
- Unsaved changes indicator (dot on tab)
- Save confirmation dialog
- Create new config file
- Tauri IPC for filesystem access

**Success Criteria:**
- [ ] Can open ~/.claude/hooks.yaml (global)
- [ ] Can open .claude/hooks.yaml (project)
- [ ] Can switch between files via tabs
- [ ] Unsaved changes tracked per file

---

### Phase 4: Rule Tree View (M5)

**Goal:** Hierarchical visualization of rules in the current config.

**Requirements:**
- Tree component showing rules by name
- Expand/collapse rule details
- Click rule to navigate to editor location
- Show rule status (enabled/disabled)
- Show rule mode (enforce/warn/audit)

**Success Criteria:**
- [ ] Rules displayed in tree structure
- [ ] Click navigates to rule in editor
- [ ] Mode indicators visible

---

### Phase 5: Debug Simulator (M6)

**Goal:** Test rules by simulating events without Claude Code.

**Requirements:**
- Event form (type, tool, command, path)
- Run simulation via `cch debug` command
- Display result (Allow/Block/Inject)
- EvaluationTrace showing which rules matched
- Timing information display

**Success Criteria:**
- [ ] Can simulate PreToolUse events
- [ ] Shows which rules matched
- [ ] Shows injected context if any

---

### Phase 6: Theming (M7)

**Goal:** Dark/light theme support with system preference detection.

**Requirements:**
- Theme toggle in header
- System preference detection
- CSS variables for theme colors
- Monaco editor theme sync
- Persist preference to localStorage

**Success Criteria:**
- [ ] Can toggle dark/light mode
- [ ] Respects system preference on first load
- [ ] Preference persisted across sessions

---

### Phase 7: E2E Tests (M8)

**Goal:** Comprehensive Playwright test coverage for critical user flows.

**Requirements:**
- Page Object Model for test organization
- Test fixtures for mock configs
- Core flow tests: open, edit, save, validate
- Simulator tests: run debug, view results
- Theme tests: toggle, persist

**Success Criteria:**
- [ ] All critical paths covered
- [ ] Tests pass on CI (Linux)
- [ ] No flaky tests

---

## Future Milestones (Post v1.0)

### Phase 8+: Log Viewer
- View CCH audit logs in UI
- Filter by date, rule, outcome
- Search log entries

### Phase 9+: Advanced Features
- Rule templates
- Regex pattern tester
- Policy packs browser

### Phase 10+: Distribution
- macOS .dmg installer
- Windows .msi installer
- Linux .deb/.AppImage
- Auto-update support

---

*Converted from .speckit/features/rulez-ui/ on 2026-02-06*

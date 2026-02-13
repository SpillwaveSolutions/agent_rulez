# Requirements: RuleZ UI v1.6

**Defined:** 2026-02-11
**Core Value:** LLMs do not enforce policy. LLMs are subject to policy.

## v1.6 Requirements

Requirements for the RuleZ UI milestone. Each maps to roadmap phases.

### Rename Fix

- [ ] **RENAME-01**: User sees `rulez` (not `cch`) in all UI labels, commands, and config paths
- [ ] **RENAME-02**: Tauri backend invokes `rulez` binary (not `cch`) for debug and validation commands
- [ ] **RENAME-03**: Log file path defaults to `~/.claude/logs/rulez.log` (not `cch.log`)

### YAML Editor

- [ ] **EDIT-01**: User gets schema-driven autocomplete suggestions when typing YAML rule fields
- [ ] **EDIT-02**: User sees inline error markers (red squiggles) for invalid YAML syntax and schema violations
- [ ] **EDIT-03**: User can click errors in an error panel to jump to the corresponding line
- [ ] **EDIT-04**: User can format/indent YAML on save or via keyboard shortcut
- [ ] **EDIT-05**: Editor properly disposes Monaco models and workers when switching files (no memory leaks)
- [ ] **EDIT-06**: User sees a live preview panel showing parsed rules with matched event types and actions

### Log Viewer

- [ ] **LOG-01**: User can view audit log entries from `~/.claude/logs/rulez.log` in a scrollable list
- [ ] **LOG-02**: User can search/filter log entries by text content
- [ ] **LOG-03**: User can filter log entries by severity level (error, warn, info, debug)
- [ ] **LOG-04**: User can filter log entries by time range (date picker)
- [ ] **LOG-05**: Log viewer handles large files (100K+ entries) with virtual scrolling at 60fps
- [ ] **LOG-06**: User can export filtered log results to a file (JSON/CSV)
- [ ] **LOG-07**: User can copy individual log entries to clipboard

### Config Management

- [ ] **CFG-01**: User can switch between global (`~/.claude/hooks.yaml`) and project (`.claude/hooks.yaml`) configs
- [ ] **CFG-02**: User sees visual indicator of which config scope is active (global vs project)
- [ ] **CFG-03**: User can import a config file from disk (file picker, YAML validated before applying)
- [ ] **CFG-04**: User can export current config to a file
- [ ] **CFG-05**: User sees config precedence (project overrides global) clearly indicated in the UI
- [ ] **CFG-06**: Config changes auto-reload when the file is modified externally (file watching with debounce)

### Debug Simulator

- [ ] **DBG-01**: User can run debug simulation using the real `rulez debug` binary (not mock data)
- [ ] **DBG-02**: User sees step-by-step rule evaluation trace showing which rules matched and why
- [ ] **DBG-03**: User can save debug test cases (event + expected result) for reuse
- [ ] **DBG-04**: User can load and replay saved test cases
- [ ] **DBG-05**: Binary path is auto-detected from PATH with fallback to manual configuration

### Settings

- [ ] **SET-01**: User can toggle theme (light/dark/system) from a settings panel
- [ ] **SET-02**: User can configure editor font size and tab size
- [ ] **SET-03**: User can configure the path to the `rulez` binary
- [ ] **SET-04**: Settings persist across app restarts (Tauri store or equivalent)

### Onboarding

- [ ] **OB-01**: First-time users see a setup wizard on initial app launch
- [ ] **OB-02**: Wizard detects whether `rulez` binary is installed and accessible
- [ ] **OB-03**: Wizard generates a sample `hooks.yaml` config with documented example rules
- [ ] **OB-04**: Wizard guides user through a test simulation to verify setup works
- [ ] **OB-05**: User can re-run onboarding from settings panel

### E2E Testing

- [ ] **E2E-01**: All new UI features have Playwright E2E tests in web mode
- [ ] **E2E-02**: E2E tests cover editor, log viewer, config management, simulator, settings, and onboarding
- [ ] **E2E-03**: E2E test suite passes in CI (GitHub Actions) on ubuntu, macOS, and Windows

## Future Requirements (v1.7+)

### OpenCode Plugin Integration

- [ ] **OPENCODE-01**: Emit RuleZ hook events from OpenCode plugin lifecycle (file.edited, tool.execute.before/after, session.updated)
- [ ] **OPENCODE-02**: Map OpenCode event context (project, directory, worktree, client SDK, shell $) to RuleZ event payload
- [ ] **OPENCODE-03**: Inject RuleZ responses (block/allow, inject context) back into OpenCode plugin flow
- [ ] **OPENCODE-04**: Register custom OpenCode tools that invoke RuleZ binary for policy checks
- [ ] **OPENCODE-05**: Support OpenCode plugin config (`~/.config/opencode/plugins/rulez-plugin/`)
- [ ] **OPENCODE-06**: Log all OpenCode-RuleZ interactions to audit trail with plugin metadata

### Gemini CLI Hook Integration

- [ ] **GEMINI-01**: Detect Gemini CLI hook events (write_file, replace, afterAgent) and translate to RuleZ format
- [ ] **GEMINI-02**: Map Gemini CLI hook matchers to RuleZ event types (PreToolUse, PostToolUse equivalents)
- [ ] **GEMINI-03**: Return structured JSON responses matching Gemini CLI hook response schema (deny/allow with reason)
- [ ] **GEMINI-04**: Support Gemini CLI extensions that bundle RuleZ hooks (`~/.gemini/hooks/`)
- [ ] **GEMINI-05**: Enable secret scanning and iterative loop patterns via RuleZ rules (e.g., "Ralph loop")
- [ ] **GEMINI-06**: Document Gemini CLI→RuleZ translation layer with installation guide

### GitHub Copilot Extension Integration

- [ ] **COPILOT-01**: Create VS Code Copilot Chat participant that queries RuleZ for policy decisions
- [ ] **COPILOT-02**: Integrate RuleZ with Copilot Language Model API for inline chat policy checks
- [ ] **COPILOT-03**: Support slash commands in Copilot Chat that trigger RuleZ debug/explain/validate
- [ ] **COPILOT-04**: Inject RuleZ context into Copilot prompts via Chat API message attachments
- [ ] **COPILOT-05**: Block/warn on Copilot code suggestions that violate RuleZ policies (pre-acceptance hook)
- [ ] **COPILOT-06**: Log all Copilot-RuleZ interactions to audit trail with extension metadata
- [ ] **COPILOT-07**: Publish VS Code extension to marketplace with RuleZ binary bundled or auto-downloaded

## Future Requirements (v2+)

### Deferred Differentiators

- **DIFF-01**: Rule-to-log correlation (click log entry to see which rule fired) — requires log format changes
- **DIFF-02**: Replay saved events from logs (time-travel debugging)
- **DIFF-03**: Multi-config comparison with Monaco diff editor
- **DIFF-04**: Real-time log streaming (live tail in UI)

### Out of Scope

| Feature | Reason |
|---------|--------|
| Visual rule builder (drag-drop) | Conflicts with YAML-first design philosophy |
| AI-powered rule suggestions | Requires LLM integration, out of scope for desktop tool |
| Cloud sync for configs | Adds auth/server complexity, use git instead |
| Mobile app | Web-first approach, desktop is primary |
| Script sandboxing UI | Cross-platform complexity, defer to v2+ |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| RENAME-01 | Phase 11 | Pending |
| RENAME-02 | Phase 11 | Pending |
| RENAME-03 | Phase 11 | Pending |
| SET-01 | Phase 11 | Pending |
| SET-02 | Phase 11 | Pending |
| SET-03 | Phase 11 | Pending |
| SET-04 | Phase 11 | Pending |
| DBG-05 | Phase 11 | Pending |
| EDIT-01 | Phase 12 | Pending |
| EDIT-02 | Phase 12 | Pending |
| EDIT-03 | Phase 12 | Pending |
| EDIT-04 | Phase 12 | Pending |
| EDIT-05 | Phase 12 | Pending |
| EDIT-06 | Phase 12 | Pending |
| LOG-01 | Phase 13 | Pending |
| LOG-02 | Phase 13 | Pending |
| LOG-03 | Phase 13 | Pending |
| LOG-04 | Phase 13 | Pending |
| LOG-05 | Phase 13 | Pending |
| LOG-06 | Phase 13 | Pending |
| LOG-07 | Phase 13 | Pending |
| CFG-01 | Phase 14 | Pending |
| CFG-02 | Phase 14 | Pending |
| CFG-03 | Phase 14 | Pending |
| CFG-04 | Phase 14 | Pending |
| CFG-05 | Phase 14 | Pending |
| CFG-06 | Phase 14 | Pending |
| DBG-01 | Phase 15 | Pending |
| DBG-02 | Phase 15 | Pending |
| DBG-03 | Phase 15 | Pending |
| DBG-04 | Phase 15 | Pending |
| OB-01 | Phase 16 | Pending |
| OB-02 | Phase 16 | Pending |
| OB-03 | Phase 16 | Pending |
| OB-04 | Phase 16 | Pending |
| OB-05 | Phase 16 | Pending |
| E2E-01 | Phase 17 | Pending |
| E2E-02 | Phase 17 | Pending |
| E2E-03 | Phase 17 | Pending |
| OPENCODE-01 | Phase 18 | Pending |
| OPENCODE-02 | Phase 18 | Pending |
| OPENCODE-03 | Phase 18 | Pending |
| OPENCODE-04 | Phase 18 | Pending |
| OPENCODE-05 | Phase 18 | Pending |
| OPENCODE-06 | Phase 18 | Pending |
| GEMINI-01 | Phase 19 | Pending |
| GEMINI-02 | Phase 19 | Pending |
| GEMINI-03 | Phase 19 | Pending |
| GEMINI-04 | Phase 19 | Pending |
| GEMINI-05 | Phase 19 | Pending |
| GEMINI-06 | Phase 19 | Pending |
| COPILOT-01 | Phase 20 | Pending |
| COPILOT-02 | Phase 20 | Pending |
| COPILOT-03 | Phase 20 | Pending |
| COPILOT-04 | Phase 20 | Pending |
| COPILOT-05 | Phase 20 | Pending |
| COPILOT-06 | Phase 20 | Pending |
| COPILOT-07 | Phase 20 | Pending |

**Coverage:**
- v1.6 requirements: 38 total
- v1.7 requirements: 19 total (6 OpenCode + 6 Gemini + 7 Copilot)
- Mapped to phases: 57/57 (100%)
- Unmapped: 0

---
*Requirements defined: 2026-02-11*
*Last updated: 2026-02-11 after v1.7 phases added*

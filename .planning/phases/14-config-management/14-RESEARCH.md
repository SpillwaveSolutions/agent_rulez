# Phase 14: Config Management - Research

**Researched:** 2026-02-12
**Domain:** Multi-scope config handling, import/export, file watching, YAML comment preservation
**Confidence:** HIGH

## Summary

Phase 14 adds multi-scope config management to the RuleZ UI. The existing infrastructure already handles basic config operations (list/read/write/validate) with a two-scope model (global at `~/.claude/hooks.yaml`, project at `.claude/hooks.yaml`). The CLI uses **first-found-wins** resolution (project config completely overrides global — no merging). The UI needs to clearly surface this precedence, add import/export with comment preservation, and implement file watching for external changes.

**Key gap:** No file watcher exists. External edits (e.g., user editing `hooks.yaml` in VS Code) are invisible until the app is restarted or the file is manually re-opened.

**Primary recommendation:** Use Tauri's existing `plugin-fs` `watchImmediate()` API for file watching (no new dependencies needed). Add scope indicator badges and import/export buttons to the sidebar. Preserve YAML comments during all operations by working with raw YAML strings rather than parsed/re-serialized objects.

## Standard Stack

### Core (Already in Project)
| Library | Version | Purpose | Status |
|---------|---------|---------|--------|
| `@tauri-apps/plugin-fs` | ^2.0.0 | File watching via `watchImmediate()` | Already installed |
| `@tauri-apps/plugin-dialog` | ^2.0.0 | Open/save file dialogs for import/export | Already installed |
| `zustand` | ^5.0.3 | Config store state management | Already in use |
| `monaco-yaml` | - | YAML editing with schema validation | Already configured |

### Not Needed
| Problem | Don't Add | Why |
|---------|-----------|-----|
| YAML merging | `yaml-merge` or custom deep merge | CLI uses first-found-wins, not merge. UI should match. |
| File diff | `diff` or `jsdiff` | No requirement for showing config diffs |
| Config linting | `yamllint` | Already have `validate_config` Tauri command |

## Architecture Patterns

### Pattern 1: Scope Indicator in Sidebar
**What:** Visual badge showing which config scope is active (project overrides global).
**When to use:** Always visible in sidebar when both configs exist.
**Why:** SC-1 requires "visual indicator of active scope" and SC-4 requires "config precedence clearly indicated."

The sidebar already separates files into "Global" and "Project" sections. Add:
- An "Active" badge on the project config when it exists (since project takes precedence)
- A "Fallback" or "Overridden" indicator on the global config when project config also exists
- Tooltip explaining precedence: "Project config takes priority over global"

### Pattern 2: Import Config with Validation
**What:** Button to import a `.yaml` file from disk, validate it, then apply.
**When to use:** SC-2 requires import with validation before applying.

Flow:
1. User clicks "Import" in sidebar
2. `open()` from `@tauri-apps/plugin-dialog` shows file picker (filter: `*.yaml`, `*.yml`)
3. `readTextFile()` from `@tauri-apps/plugin-fs` reads the selected file
4. `validateConfig(tempPath)` validates the YAML (or validate inline using the existing YAML schema)
5. If valid: show preview, user confirms, write to target config path
6. If invalid: show errors, don't apply

### Pattern 3: Export Config with Comment Preservation
**What:** Export the current config to a file, preserving all comments and formatting.
**When to use:** SC-3 requires export preserving comments.

Key insight: The config store already holds raw YAML strings (not parsed objects). `readConfig()` returns the raw file content and `writeConfig()` writes it directly. Comments are preserved because we never parse-and-reserialize — the editor works on the raw string.

Export flow:
1. User clicks "Export"
2. `save()` from `@tauri-apps/plugin-dialog` shows save dialog
3. Write the raw content from `configStore.openFiles.get(path).content` to the chosen path
4. Comments are preserved because we export the editor's raw string

### Pattern 4: File Watching with Tauri plugin-fs
**What:** Watch config files for external changes and prompt the user to reload.
**When to use:** SC-5 requires auto-reload on external changes.

Tauri v2 `plugin-fs` provides `watchImmediate()`:
```typescript
import { watchImmediate } from '@tauri-apps/plugin-fs';

const stopWatching = await watchImmediate(
  '/path/to/hooks.yaml',
  (event) => {
    // event.type: 'create' | 'modify' | 'remove' | ...
    if (event.type.modify) {
      // File was changed externally
    }
  },
  { recursive: false }
);

// Clean up
stopWatching();
```

Design:
- Watch both global and project config file paths
- On external modify: if file is open and unmodified in editor → auto-reload silently
- On external modify: if file is open and has unsaved changes → show "File changed on disk. Reload? [Reload] [Keep mine]" notification
- Debounce events (500ms) to avoid rapid-fire reloads from editors that do save-then-rename

### Anti-Patterns to Avoid
- **Don't parse and re-serialize YAML for export**: This strips comments. Always work with raw strings.
- **Don't deep-merge configs**: The CLI uses first-found-wins, not merge. The UI should reflect the same behavior.
- **Don't watch with polling**: Use Tauri's event-based `watchImmediate()` which uses OS-native file watchers (inotify on Linux, FSEvents on macOS, ReadDirectoryChanges on Windows).
- **Don't auto-save on external change if user has edits**: Always ask the user first.

## Current State Analysis

### What Already Works
1. **Config listing**: `list_config_files()` returns both global and project configs
2. **Config read/write**: Full CRUD via Tauri commands
3. **Config validation**: Shells out to `rulez validate --json`
4. **Multi-tab editor**: Open multiple configs simultaneously, unsaved-changes tracking
5. **File tab bar**: Close with save/discard/cancel confirmation dialog
6. **YAML schema validation**: Monaco with inline error markers
7. **Comment preservation**: Editor works on raw strings, never parses-then-reserializes

### What's Missing (Phase 14 Scope)
1. **Scope indicator**: No visual indicator of which config is active/overridden
2. **Import flow**: No way to import a config file from an arbitrary path
3. **Export flow**: No way to export/save-as a config to a different path
4. **File watching**: No detection of external file changes
5. **Precedence explanation**: No UI explaining that project overrides global

### Config Precedence Rules (from CLI)
```
1. If {cwd}/.claude/hooks.yaml exists → use it (project config)
2. Else if ~/.claude/hooks.yaml exists → use it (global config)
3. Else → use default empty config
```
This is NOT a merge. Project config completely replaces global. The UI must clearly communicate this.

## Risk Assessment

### Risk 1: Tauri plugin-fs watchImmediate API Stability
**Severity:** LOW
**Issue:** `watchImmediate` is a Tauri v2 API that wraps the `notify` crate.
**Mitigation:** Already have `tauri-plugin-fs` installed. The API is stable in v2. If it fails gracefully (e.g., permission denied), fall back to manual refresh button.

### Risk 2: File Watch Event Debouncing
**Severity:** LOW-MEDIUM
**Issue:** Some editors (VS Code, vim) write files using save-to-temp-then-rename, generating multiple events.
**Mitigation:** Debounce watch events by 500ms. Only react to the last event in a burst.

### Risk 3: Import Validation Requires Binary
**Severity:** MEDIUM
**Issue:** `validateConfig()` shells out to the `rulez` binary. If the binary isn't configured, validation fails.
**Mitigation:** Fall back to basic YAML syntax validation (already done by Monaco/yaml-schema) if binary validation fails. Show a warning: "Full validation requires the rulez binary. See Settings."

## Success Criteria Mapping

| SC | Requirement | Implementation |
|----|------------|----------------|
| SC-1 | Switch between global/project configs with scope indicator | Scope badges in sidebar + existing tab switching |
| SC-2 | Import config files with YAML validation | Import button + dialog + validate + preview/confirm |
| SC-3 | Export current config preserving comments | Export button + save dialog + raw string write |
| SC-4 | Config precedence clearly indicated | Scope badges ("Active"/"Overridden") + tooltip |
| SC-5 | Auto-reload on external file changes (debounced) | Tauri plugin-fs watchImmediate + debounce + conflict dialog |

## Recommended Plan Structure

### Plan 01: Scope Indicators + Config Precedence UI
- Add scope badges to sidebar (Active/Overridden)
- Add precedence explanation tooltip
- Add config scope status to status bar
- Minimal, no new dependencies

### Plan 02: Import/Export with Validation
- Create import flow (dialog → read → validate → preview → apply)
- Create export flow (dialog → write raw string)
- Browser-mode fallbacks for testing
- Wire into sidebar UI

### Plan 03: File Watching + External Change Detection
- Implement file watcher using `watchImmediate()`
- Add debounced change detection (500ms)
- Conflict resolution dialog (reload vs keep)
- Auto-reload for unmodified files
- Cleanup on unmount / file close
- Integration verification

## Sources

### Primary (HIGH confidence)
- `rulez_ui/src-tauri/src/commands/config.rs` — Rust config commands (list, read, write)
- `rulez_ui/src/stores/configStore.ts` — Zustand config state management
- `rulez_ui/src/components/layout/Sidebar.tsx` — Sidebar with Global/Project sections
- `cch_cli/src/config.rs` — CLI config resolution (first-found-wins precedence)
- `@tauri-apps/plugin-fs` docs — `watchImmediate()` API

### Secondary (MEDIUM confidence)
- `rulez_ui/src/lib/tauri.ts` — Tauri abstraction with mock fallbacks
- `rulez_ui/src/stores/settingsStore.ts` — Settings persistence patterns
- Tauri v2 plugin-fs watch API documentation

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — all dependencies already installed
- Architecture: HIGH — builds on existing patterns (sidebar, configStore, tauri.ts)
- Config precedence: HIGH — verified from CLI source code
- File watching: MEDIUM — API exists but untested in this project
- Risk: LOW — no new dependencies, minimal new Rust code

**Research date:** 2026-02-12
**Valid until:** 2026-03-14

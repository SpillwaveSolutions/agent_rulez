# Stack Research: RuleZ UI v1.5 Production Features

**Domain:** Desktop application - production quality features (log viewing, advanced editing, settings, distribution)
**Researched:** 2026-02-10
**Confidence:** HIGH

## Executive Summary

RuleZ UI v1.5 builds on the validated M1 scaffold (Tauri 2.0 + React 18 + Monaco Editor + Zustand). This research identifies NEW libraries needed for production features: log viewing with virtual scrolling, advanced Monaco autocomplete, file watching, settings persistence, onboarding, and desktop distribution. All recommendations integrate with the existing architecture and maintain the project's performance and security standards.

## Recommended Stack Additions

### Virtual Scrolling for Log Viewer

| Library | Version | Purpose | Why Recommended |
|---------|---------|---------|-----------------|
| @tanstack/react-virtual | ^3.13.18 | Headless virtual scrolling for JSONL logs | Lightweight (10-15kb), 60fps performance with 100K+ rows, framework-agnostic, actively maintained by TanStack (proven with React Query already in use). Handles variable-height rows for log entries with different metadata sizes. |

**Integration:** Use with custom log parser component that reads JSONL line-by-line from Tauri fs plugin, passes parsed objects to `useVirtualizer` hook. Compatible with existing Zustand state management for filter/search state.

### Monaco Editor Enhancements

| Library | Version | Purpose | Why Recommended |
|---------|---------|---------|-----------------|
| N/A (use built-in) | 4.7.0 | Custom completion provider | Monaco Editor already includes `monaco.languages.registerCompletionItemProvider` API. Use with JSON Schema from rulez binary to generate field-specific autocomplete. No new dependency needed. |

**Implementation:** Register custom completion provider for `yaml` language ID, use JSON Schema validation results to provide inline docs and autocomplete for RuleZ-specific fields (`inject_inline`, `inject_command`, `enabled_when`, `prompt_match`, etc.). Schema already exists in `public/schema/hooks-schema.json` (planned for M3).

### File System Watching (Rust Backend)

| Library | Version | Purpose | Why Recommended |
|---------|---------|---------|-----------------|
| notify | ^6.1.1 | Cross-platform file watcher | Industry standard (used by rust-analyzer, cargo-watch, mdBook, watchexec). Tauri CLI already uses this internally. Efficient event debouncing, supports all platforms (Linux inotify, macOS FSEvents, Windows ReadDirectoryChangesW). |
| tauri-plugin-fs-watch | ^2.0 | Tauri wrapper for notify | Official Tauri plugin, provides IPC bridge to frontend with watch/watchImmediate commands. Integrates with Tauri's permission system and event system. |

**Integration:** Use `tauri-plugin-fs-watch` to monitor `~/.claude/hooks.yaml` and `.claude/hooks.yaml` for changes. Emit events to frontend when configs change, trigger Zustand store refresh. Debounce events to prevent editor conflicts during user typing.

### Settings Persistence

| Library | Version | Purpose | Why Recommended |
|---------|---------|---------|-----------------|
| N/A (use Zustand middleware) | 5.0.3 | Persist settings to Tauri storage | Zustand already includes `persist` middleware with `localStorage` support. For Tauri desktop app, use Tauri's `@tauri-apps/plugin-store` for secure, encrypted settings storage with native file permissions. |
| @tauri-apps/plugin-store | ^2.0 | Native settings persistence | Official Tauri plugin for secure key-value storage with encryption, native file permissions, and automatic migration. Better than localStorage for desktop apps (type safety, encryption, schema versioning). |

**Integration:** Wrap settings Zustand store with `persist` middleware, use custom storage adapter that calls Tauri Store plugin. Store user preferences: theme, editor font size, Monaco options, default file locations, recent files.

### Onboarding/First-Run Experience

| Library | Version | Purpose | Why Recommended |
|---------|---------|---------|-----------------|
| react-joyride | ^2.9.3 | Interactive product tours | Mature (5.1k GitHub stars), accessible (WCAG 2.1 compliant), flexible styling, supports multi-step tours with callbacks. React 18 compatible, TypeScript support. Simpler API than Intro.js, smaller bundle than alternatives. |

**Alternative:** NextStepjs (2.0+) - Newer, more modern API with Framer Motion animations, but requires additional dependency (motion). Use if animations are priority.

**Integration:** Create tour definitions for: 1) First launch (install rulez binary, create first config), 2) Editor tour (YAML syntax, validation, preview), 3) Debug simulator tour. Store tour completion state in settings store (via Tauri Store plugin). Trigger tours based on settings flags.

### Desktop Distribution & Auto-Updates (Rust Backend)

| Library | Version | Purpose | Why Recommended |
|---------|---------|---------|-----------------|
| tauri-plugin-updater | ^2.9.0 | Auto-update mechanism | Official Tauri plugin, supports NSIS (Windows), MSI (Windows), AppImage (Linux), .app (macOS). Signature verification required (security-first). Works with static JSON or dynamic server. Already used by production Tauri apps. |

**Security Note:** Updater REQUIRES code signing. macOS needs Apple Developer ID, Windows needs Authenticode certificate. Linux AppImages use GPG signatures. Set up signing in CI before enabling updater.

**Integration:** Configure `tauri.conf.json` with update endpoint (GitHub Releases or static S3/gist JSON). Use `check()` method on app launch (background check), show notification when update available. User controls restart timing. Store update preferences in settings.

## Supporting Libraries (NEW)

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| date-fns | ^3.3.1 | Log timestamp formatting | Smaller than moment.js (2kb gzipped vs 67kb), immutable, TypeScript-native. Use for parsing/formatting JSONL log timestamps in log viewer. |
| nanoid | ^5.0.4 | Generate unique IDs | For tracking tour state, temporary log filters, session IDs. Smaller (130 bytes) and more secure than uuid. |

## Development Tools (NEW)

| Tool | Purpose | Notes |
|------|---------|-------|
| @tauri-apps/api types | TypeScript types for Tauri IPC | Already in package.json (2.5.0), verify includes types for new plugins (store, fs-watch, updater). |
| cargo-bundle | Tauri bundler | Included with @tauri-apps/cli, creates installers for all platforms. |

## Installation

### Frontend (React/TypeScript)

```bash
# Virtual scrolling
bun add @tanstack/react-virtual

# Date formatting for logs
bun add date-fns

# Unique IDs
bun add nanoid

# Onboarding tour
bun add react-joyride

# Tauri plugins (frontend bindings)
bun add @tauri-apps/plugin-store
```

### Backend (Rust/Tauri)

```toml
# Add to rulez-ui/src-tauri/Cargo.toml

[dependencies]
# File watching
tauri-plugin-fs-watch = "2.0"

# Settings persistence
tauri-plugin-store = "2.0"

# Auto-updates (optional, requires code signing setup)
tauri-plugin-updater = "2.9"

# File watching (used by fs-watch plugin)
notify = "6.1"
```

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| @tanstack/react-virtual | react-window | Use react-window if you need backward compatibility with React <16.8 or simple fixed-height lists. TanStack Virtual is better for variable-height items (log entries with metadata). |
| react-joyride | NextStepjs | Use NextStepjs if you want modern Framer Motion animations and don't mind the extra dependency. More customizable styling, but newer (less battle-tested). |
| tauri-plugin-store | localStorage | Never use localStorage for sensitive settings (unencrypted, no file permissions). Use Tauri Store for desktop apps. |
| @tanstack/react-virtual | react-lazylog | react-lazylog is specialized for logs but less flexible. Use TanStack Virtual + custom JSONL parser for more control over rendering. |
| notify (direct) | tauri-plugin-fs-watch | Always use the Tauri plugin wrapper for IPC and permission management. Only use notify directly if building custom Rust-only tooling. |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| react-virtualized | Deprecated, no longer maintained | @tanstack/react-virtual or react-window |
| moment.js | Large bundle (67kb), mutable API, deprecated | date-fns (2kb, immutable) or native Intl APIs |
| Intro.js (commercial) | Requires license for commercial use | react-joyride (MIT license) or NextStepjs (open source) |
| Custom file watchers (polling) | Inefficient, battery drain on laptops | notify crate (native OS events) |
| IndexedDB for settings | Async complexity, quota limits, browser API | tauri-plugin-store (native, synchronous, encrypted) |
| Sparkle updater (macOS only) | Platform-specific, requires Objective-C bridge | tauri-plugin-updater (cross-platform) |

## Integration Patterns

### Log Viewer Architecture

```typescript
// Component structure
<LogViewer>
  <LogToolbar> {/* Filter, search, date range */}
    <FilterButtons />
    <SearchInput />
    <DateRangePicker />
  </LogToolbar>
  <VirtualLogList> {/* TanStack Virtual */}
    <LogEntry /> {/* Variable height, expandable metadata */}
  </VirtualLogList>
</LogViewer>

// Data flow
Tauri fs.readTextFile() → Parse JSONL → Filter/search (Zustand) → Virtualizer → Render visible rows
```

**Performance:** Read logs in chunks (1MB at a time), parse on-demand, keep parsed objects in LRU cache. TanStack Virtual handles windowing automatically.

### Monaco Autocomplete Integration

```typescript
// Register custom provider on Monaco mount
monaco.languages.registerCompletionItemProvider('yaml', {
  provideCompletionItems: (model, position) => {
    // Parse current YAML context
    // Fetch schema from rulez binary (via Tauri command)
    // Return CompletionList with field suggestions + docs
  }
});
```

**Schema source:** Call `rulez validate --json` via Tauri shell plugin to get schema-aware validation results, extract field definitions for autocomplete.

### File Watcher Pattern

```rust
// src-tauri/src/commands/watcher.rs
#[tauri::command]
async fn watch_config_file(path: String, app: AppHandle) -> Result<()> {
  let (tx, rx) = channel();
  let watcher = notify::recommended_watcher(tx)?;
  watcher.watch(&path, RecursiveMode::NonRecursive)?;

  // Emit event to frontend on file change (debounced)
  app.emit_all("config-changed", { path });
}
```

**Frontend:** Listen to `config-changed` event, show toast notification, optionally reload config (only if user hasn't made unsaved changes).

### Settings Store Pattern

```typescript
// stores/settingsStore.ts
import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { Store } from '@tauri-apps/plugin-store';

const tauriStore = new Store('settings.json');

export const useSettingsStore = create(
  persist(
    (set) => ({
      theme: 'dark',
      editorFontSize: 14,
      // ... other settings
    }),
    {
      name: 'rulez-settings',
      storage: {
        getItem: (key) => tauriStore.get(key),
        setItem: (key, value) => tauriStore.set(key, value),
        removeItem: (key) => tauriStore.delete(key),
      },
    }
  )
);
```

### Onboarding Tour Definitions

```typescript
// lib/tours.ts
export const firstLaunchTour = [
  {
    target: '.install-button',
    content: 'Install the rulez binary to start using RuleZ',
    disableBeacon: true,
  },
  {
    target: '.create-config',
    content: 'Create your first hooks.yaml configuration',
  },
  // ... more steps
];
```

**Trigger logic:** Check `settings.tours.firstLaunch` flag, show tour if false, set to true on completion.

## Version Compatibility

| Package | Compatible With | Notes |
|---------|-----------------|-------|
| @tanstack/react-virtual@^3.13.18 | React 18.3.1 | Requires React 16.8+ for hooks |
| react-joyride@^2.9.3 | React 18.3.1, TypeScript 5.7+ | Peer dep: react-dom ^16.4.0 \|\| ^17.0.0 \|\| ^18.0.0 |
| tauri-plugin-updater@^2.9.0 | Tauri 2.0+ | Requires Rust 1.77.2+ |
| tauri-plugin-store@^2.0 | Tauri 2.0+ | Requires Rust 1.70+ |
| tauri-plugin-fs-watch@^2.0 | Tauri 2.0+ | Requires notify@6.1+ |
| notify@^6.1.1 | Rust 1.67+ | Cross-platform, used by fs-watch plugin |
| date-fns@^3.3.1 | TypeScript 5.0+ | ESM-only in v3, use imports not require() |

**Known Issues:**
- date-fns v3 is ESM-only, ensure Vite config uses `optimizeDeps.include: ['date-fns']`
- tauri-plugin-updater requires code signing setup before production use (see Tauri docs)
- notify crate on Windows may have permission issues with system directories, test with user directories only

## Stack Patterns by Use Case

**If building log viewer for large files (>10MB):**
- Use @tanstack/react-virtual with `estimateSize` for variable-height rows
- Read file in chunks via Tauri fs plugin (1MB at a time)
- Parse JSONL lazily, cache parsed objects in LRU (maxSize: 1000)
- Index timestamps for fast date range filtering

**If implementing live config reload:**
- Use tauri-plugin-fs-watch with 500ms debounce
- Check for unsaved editor changes before reloading
- Show toast notification "Config changed externally, reload?"
- Merge external changes if no conflicts (same as VS Code behavior)

**If adding onboarding tour:**
- Use react-joyride for simplicity and accessibility
- Define tours as data (not JSX), store in `lib/tours.ts`
- Track completion per-tour in settings store
- Allow users to restart tours from Help menu

**If distributing desktop app:**
- Set up tauri-plugin-updater with GitHub Releases as endpoint
- Use Tauri Action for automated builds (see existing `.github/workflows/tauri-build.yml`)
- Sign binaries: macOS (Apple Developer ID), Windows (Authenticode), Linux (GPG)
- Test updater on all platforms before release (use draft releases for testing)

## Existing Stack (DO NOT Re-Add)

These are already in package.json or Cargo.toml, do NOT add again:

**Frontend:**
- React 18.3.1, React DOM 18.3.1
- @monaco-editor/react 4.7.0, monaco-yaml 5.3.1
- Zustand 5.0.3, @tanstack/react-query 5.64.0
- Tailwind CSS 4.0.6, PostCSS 8.5.1, Autoprefixer 10.4.20
- TypeScript 5.7.3, Vite 6.1.0, @vitejs/plugin-react 4.3.4
- Biome 1.9.4, Playwright 1.50.1
- @tauri-apps/api 2.5.0, @tauri-apps/plugin-shell 2.2.1
- yaml 2.8.2

**Backend (Rust):**
- Tauri 2.0, tauri-build 2.0, tauri-plugin-shell 2.0
- serde 1.0, serde_json 1.0
- tokio 1.0 (features: process, fs)
- dirs 5.0

## Sources

**HIGH Confidence:**
- [TanStack Virtual](https://tanstack.com/virtual/latest) — Official docs, verified v3.13.18 release
- [Tauri Updater Plugin](https://v2.tauri.app/plugin/updater/) — Official Tauri v2 docs
- [Tauri File System Plugin](https://v2.tauri.app/plugin/file-system/) — Official Tauri v2 docs
- [notify crate](https://github.com/notify-rs/notify) — GitHub repo, crates.io verified
- [Zustand persist middleware](https://zustand.docs.pmnd.rs/middlewares/persist) — Official docs

**MEDIUM Confidence:**
- [react-joyride](https://www.npmjs.com/package/react-joyride) — npm registry, v2.9.3 verified
- [Monaco Editor custom completion](https://www.checklyhq.com/blog/customizing-monaco/) — Community tutorial, verified with Monaco docs
- [monaco-yaml schema configuration](https://github.com/remcohaszing/monaco-yaml) — GitHub repo, maintained package

**LOW Confidence (Verify Before Use):**
- tauri-plugin-fs-watch version — Could not verify exact 2.0 version on crates.io (website JS-blocked), assume 2.0 based on Tauri 2.0 ecosystem. **Verify with `cargo search tauri-plugin-fs-watch` before adding.**

---

*Stack research for: RuleZ UI v1.5 production features*
*Researched: 2026-02-10*
*Integration focus: Log viewing, advanced Monaco editing, file watching, settings, onboarding, distribution*

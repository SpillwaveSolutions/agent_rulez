# Phase 11: Rename Fix + Settings Foundation - Research

**Researched:** 2026-02-11
**Domain:** Tauri v2 desktop app (Rust backend) + React/Zustand frontend settings persistence
**Confidence:** MEDIUM

## Summary

The current Tauri UI already uses React, Zustand, and Monaco, with theme persisted to localStorage and hard-coded editor options (font size, tab size). The backend runs `cch` via `std::process::Command` in `src-tauri/src/commands/debug.rs`, and the Tauri shell plugin scope is configured for `cch` in `src-tauri/tauri.conf.json`. The CLI logger default path is `~/.claude/logs/cch.log` in `cch_cli/src/logging.rs`. These are the concrete rename and path targets for Phase 11.

For settings persistence, the standard Tauri v2 approach is `@tauri-apps/plugin-store` (JS) + `tauri-plugin-store` (Rust). It provides an on-disk key/value store and optional auto-save, and can be used to store theme, editor font size/tab size, and a user-defined binary path. A lightweight UI settings panel can read/write a single settings object from the store, while keeping existing Zustand stores for runtime state. In web/test mode, continue using localStorage or a mock fallback since the Store plugin requires Tauri runtime.

Binary path auto-detection should occur before debug/validate calls. If no user override is set, resolve `rulez` from PATH (or attempt `rulez --version`) and store the discovered full path. Be prepared for PATH differences when launched from GUI on macOS and Windows; fallback to manual path entry in settings is required.

**Primary recommendation:** Use `@tauri-apps/plugin-store` to persist settings (theme, editor options, binary path) and update the Tauri backend to execute `rulez` with a resolved path fallback when settings are unset.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| tauri | 2.0 | Desktop shell, IPC, windowing | Existing backend stack in `src-tauri/Cargo.toml` |
| @tauri-apps/api | 2.5.0 | Frontend Tauri APIs | Existing frontend dependency |
| @tauri-apps/plugin-shell | 2.2.1 | Command execution / shell access | Existing dependency and configured plugin |
| tauri-plugin-store | 2.0.0 | Persistent settings storage (Rust) | Official Tauri v2 plugin |
| @tauri-apps/plugin-store | 2.x | Persistent settings storage (JS) | Official Tauri v2 plugin |
| React | 18.3.1 | UI rendering | Existing frontend stack |
| Zustand | 5.0.3 | Client state management | Existing store usage |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| monaco-editor | (via @monaco-editor/react) | Editor UI | Apply font size/tab size preferences |
| dirs | 5.0 | Home directory lookup | Already used in CLI logger paths |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Tauri Store plugin | LocalStorage only | Works in web mode but does not persist reliably across desktop app restarts in all environments; lacks centralized settings file |

**Installation:**
```bash
# Frontend
bun add @tauri-apps/plugin-store

# Backend (src-tauri/Cargo.toml)
tauri-plugin-store = "2.0.0"
```

## Architecture Patterns

### Recommended Project Structure
```
rulez_ui/src/
├── components/settings/     # Settings panel UI (theme/editor/binary path)
├── stores/settingsStore.ts  # Persisted settings model + defaults
├── lib/settings.ts          # Tauri Store wrapper + web fallback
└── lib/tauri.ts             # IPC + command wrappers
```

### Pattern 1: Persisted Settings via Tauri Store
**What:** Use `@tauri-apps/plugin-store` to load a settings store, set/get keys, and auto-save. In web/test mode, mirror to localStorage.
**When to use:** Theme, editor font size/tab size, binary path, and future preferences.
**Example:**
```ts
// Source: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/README.md
import { Store } from '@tauri-apps/plugin-store'

const store = await Store.load('settings.json', {
  defaults: { theme: 'system', editorFontSize: 14, editorTabSize: 2, rulezBinaryPath: null },
  autoSave: true,
})

await store.set('theme', 'dark')
const theme = await store.get('theme')
```

### Pattern 2: Backend Command Invocation Uses Resolved Binary Path
**What:** Backend command handlers resolve a binary path from settings (or PATH) and invoke `rulez` using `Command::new(path)`.
**When to use:** `run_debug` and `validate_config` to ensure RENAME-02 and DBG-05.

### Anti-Patterns to Avoid
- **Storing complex non-serializable data in Tauri Store:** Store only plain JSON types; Zustand maps and editor refs must stay in runtime stores.
- **Hard-coding editor options:** Use settings store to apply `tabSize` and `fontSize` to Monaco options and update when settings change.
- **Assuming PATH is the same as shell PATH:** GUI-launched apps often have a reduced PATH; always allow manual override.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Settings persistence | Custom JSON file IO | Tauri Store plugin | Handles file location, auto-save, and JS bindings consistently |
| Command execution from frontend | Custom IPC wrappers | Tauri shell plugin | Standard plugin with scoping support |

**Key insight:** Official Tauri plugins provide the correct OS-specific storage locations and security model, reducing platform-specific bugs.

## Common Pitfalls

### Pitfall 1: Store Values Not Compatible With JS Bindings
**What goes wrong:** Rust-side store writes non-`serde_json::Value` types that JS bindings cannot decode.
**Why it happens:** Store expects JSON values for interop.
**How to avoid:** Use `serde_json::json!()` for Rust writes and keep settings as JSON-compatible primitives/objects.
**Warning signs:** Settings load as `null` or cause deserialize errors in JS.

### Pitfall 2: Shell Plugin Scope Not Updated
**What goes wrong:** Frontend command execution fails due to mismatched `tauri.conf.json` scope entries.
**Why it happens:** Scope still references `cch` while code uses `rulez`.
**How to avoid:** Update `plugins.shell.scope` to match the `rulez` command name and args.
**Warning signs:** Command errors stating scope is not allowed or command not found.

### Pitfall 3: GUI PATH Mismatch
**What goes wrong:** Auto-detection fails because PATH lacks user shell paths in GUI context.
**Why it happens:** macOS/Windows GUI apps do not inherit shell init PATH.
**How to avoid:** Use detection best-effort and rely on user-configured binary path when not found.
**Warning signs:** Auto-detect works in dev shell but not in packaged app.

## Code Examples

Verified patterns from official sources:

### Register Tauri Store Plugin (Rust)
```rust
// Source: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/README.md
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Use Tauri Store (JS)
```ts
// Source: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/README.md
import { Store } from '@tauri-apps/plugin-store'

const store = await Store.load('settings.json')
await store.set('some-key', { value: 5 })
const val = await store.get<{ value: number }>('some-key')
```

### Execute a Command With Shell Plugin (JS)
```ts
// Source: https://context7.com/tauri-apps/plugins-workspace/llms.txt
import { Command } from '@tauri-apps/plugin-shell'

const output = await Command.create('ls', ['-la', '/tmp']).execute()
console.log(output.code, output.stdout, output.stderr)
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Manual JSON files | Tauri Store plugin | Tauri v2 | Standardizes persistence with JS bindings |

**Deprecated/outdated:**
- Tauri v1 `tauri::api::path` patterns replaced by Manager path APIs (v2).

## Open Questions

1. **Where should settings live in web/test mode?**
   - What we know: Store plugin requires Tauri runtime; current theme uses localStorage.
   - What's unclear: Whether the web/test mode needs parity with desktop settings.
   - Recommendation: Implement localStorage fallback mirroring the same settings keys.

2. **Should auto-detected binary path be persisted?**
   - What we know: Settings must persist across restarts; auto-detection is required.
   - What's unclear: Persisting auto-detect vs recalculating each launch.
   - Recommendation: Persist once detected and revalidate if command fails.

## Sources

### Primary (HIGH confidence)
- https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/store/README.md - Store plugin usage and Rust/JS examples
- https://context7.com/tauri-apps/plugins-workspace/llms.txt - Shell plugin examples and store usage details
- https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/develop/configuration-files.mdx - Tauri config examples

### Secondary (MEDIUM confidence)
- https://github.com/tauri-apps/tauri-docs/blob/v2/src/content/docs/start/migrate/from-tauri-1.mdx - v2 path APIs

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - versions verified from repo and Tauri docs
- Architecture: MEDIUM - settings UI patterns inferred from current structure
- Pitfalls: MEDIUM - based on Tauri plugin constraints and GUI PATH behavior

**Research date:** 2026-02-11
**Valid until:** 2026-03-13

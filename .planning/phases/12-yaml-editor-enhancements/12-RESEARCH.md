# Phase 12: YAML Editor Enhancements - Research

**Researched:** 2026-02-12
**Domain:** Monaco Editor YAML integration, schema-driven autocomplete, memory management
**Confidence:** HIGH

## Summary

Phase 12 targets five capabilities on top of an already well-built Monaco integration: schema-driven autocomplete, inline error markers, clickable error panel, YAML formatting, and memory-safe model lifecycle management. The investigation reveals that **the current codebase already has significant scaffolding in place** — `monaco-yaml` v5.3.1 is installed and configured, the JSON schema exists at `public/schema/hooks-schema.json`, a `ValidationPanel` with click-to-navigate exists, and the `EditorToolbar` already has a format button wired to `editor.action.formatDocument`. The remaining work is primarily about **hardening, completing, and polishing** what exists rather than building from scratch.

The critical gap is **memory management**: the current `YamlEditor` component creates marker listeners on mount but never cleans them up, doesn't manage model lifecycle across file switches, and doesn't dispose the `configureMonacoYaml` return value. The YAML formatting button triggers `editor.action.formatDocument` but no formatting provider is registered, so it's a no-op. Schema autocomplete may already work via `monaco-yaml` but needs verification and potential schema URI fixes.

**Primary recommendation:** Focus implementation on (1) verifying/fixing the existing monaco-yaml schema integration, (2) registering a YAML formatting provider using the `yaml` package, (3) implementing proper model disposal and cleanup on file switches, and (4) polishing the existing ValidationPanel with column-accurate navigation.

## Current State Analysis

### What Already Exists (from codebase investigation)

| Component | File | Status | Notes |
|-----------|------|--------|-------|
| Monaco Editor | `YamlEditor.tsx` | **Working** | `@monaco-editor/react` v4.7.0, themes, cursor tracking |
| monaco-yaml | `lib/schema.ts` | **Configured** | `configureMonacoYaml` called in `beforeMount` with schema |
| JSON Schema | `public/schema/hooks-schema.json` | **Complete** | Draft-07, 300 lines, covers all hook fields |
| Error markers | `YamlEditor.tsx:87-119` | **Working** | Subscribes to `onDidChangeMarkers`, extracts errors/warnings |
| Validation panel | `ValidationPanel.tsx` | **Working** | Click-to-navigate via `revealLineInCenter` + `setPosition` |
| Error counts in status bar | `StatusBar.tsx` | **Working** | Shows error/warning counts from editorStore |
| Editor toolbar | `EditorToolbar.tsx` | **Partial** | Format button exists but triggers built-in formatter (no YAML provider registered) |
| Settings integration | `YamlEditor.tsx` | **Working** | Font size, tab size, theme from settingsStore |
| Editor store | `editorStore.ts` | **Working** | Cursor, selection, errors, warnings, editorRef |
| Config store | `configStore.ts` | **Working** | Multi-file: openFiles Map, activeFile, content tracking |
| yaml package | `package.json` | **Installed** | `yaml` v2.8.2 (eemeli/yaml) |

### What's Missing or Broken

| Gap | Severity | Description |
|-----|----------|-------------|
| **Schema URI loading** | HIGH | Schema configured as `/schema/hooks-schema.json` with `enableSchemaRequest: true` — this relies on the Vite dev server serving the file. Need to verify it works in Tauri production builds. |
| **YAML formatting provider** | HIGH | `handleFormat` in toolbar triggers `editor.action.formatDocument` but no `DocumentFormattingEditProvider` is registered for YAML. Button is a no-op. |
| **Memory management** | HIGH | No model disposal on file switch. No cleanup of marker listener disposables. No cleanup of `configureMonacoYaml` disposable return. |
| **Format on save** | MEDIUM | `onSave` callback exists but doesn't trigger formatting before save. |
| **Marker listener leak** | MEDIUM | `onDidChangeMarkers` is subscribed in `handleMount` but never disposed. Creates a new listener each time the editor mounts. |
| **Schema URI for Tauri** | MEDIUM | In Tauri production, static assets may be served differently. Need to ensure schema file is accessible at runtime. |

## Standard Stack

### Core (Already Installed)
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `@monaco-editor/react` | ^4.7.0 | React Monaco wrapper | Standard React integration, manages editor lifecycle |
| `monaco-yaml` | ^5.3.1 | YAML language support for Monaco | Provides tokenization, validation, autocomplete via yaml-language-server |
| `yaml` | 2.8.2 | YAML parse/stringify | Used for formatting; full YAML 1.1/1.2 support, comment preservation |
| `monaco-editor` | (bundled) | Core editor | Bundled via `@monaco-editor/react` loader |
| `zustand` | ^5.0.3 | State management | Already used for editorStore, configStore, settingsStore |

### No Additional Packages Needed

The entire phase can be implemented with existing dependencies. No new npm packages required.

## Architecture Patterns

### Recommended Changes to Project Structure

No new directories needed. New files:

```
src/
├── lib/
│   ├── schema.ts              # EXISTS - configure monaco-yaml (needs minor fix)
│   ├── yaml-formatter.ts      # NEW - register YAML formatting provider
│   └── editor-lifecycle.ts    # NEW - model disposal and cleanup utilities
├── components/
│   └── editor/
│       ├── YamlEditor.tsx     # EXISTS - needs cleanup/disposal wiring
│       ├── EditorToolbar.tsx   # EXISTS - format button already works once provider is registered
│       └── ValidationPanel.tsx # EXISTS - already complete, may need minor polish
└── stores/
    └── editorStore.ts         # EXISTS - may need model registry additions
```

### Pattern 1: Disposable Management

**What:** Track all Monaco disposables and clean them up on component unmount or file switch.
**When to use:** Every time you call a Monaco API that returns an `IDisposable`.

```typescript
// Source: Monaco Editor official docs + verified via Context7
// Pattern: Collect disposables in a ref, dispose on cleanup

import { useRef, useEffect } from "react";
import type { IDisposable } from "monaco-editor";

function useDisposables() {
  const disposables = useRef<IDisposable[]>([]);

  const track = (disposable: IDisposable) => {
    disposables.current.push(disposable);
    return disposable;
  };

  useEffect(() => {
    return () => {
      for (const d of disposables.current) {
        d.dispose();
      }
      disposables.current = [];
    };
  }, []);

  return track;
}
```

### Pattern 2: Model Registry for File Switching

**What:** Maintain a Map of file paths to Monaco models. Dispose models when files are closed (not when switching between them).
**When to use:** Multi-file editor with tab-based navigation.

```typescript
// Source: Monaco Editor docs (Context7) + Expo blog best practices
// Key insight: editor.setModel() to switch, model.dispose() only on close

const modelRegistry = new Map<string, {
  model: editor.ITextModel;
  viewState: editor.ICodeEditorViewState | null;
}>();

// On file switch: save current state, restore target state
function switchToFile(editor: editor.IStandaloneCodeEditor, path: string, content: string) {
  // Save current view state
  const currentModel = editor.getModel();
  if (currentModel) {
    const currentPath = currentModel.uri.path;
    const entry = modelRegistry.get(currentPath);
    if (entry) {
      entry.viewState = editor.saveViewState();
    }
  }

  // Get or create model for target file
  let entry = modelRegistry.get(path);
  if (!entry) {
    const uri = monaco.Uri.parse(`file:///${path}`);
    const model = monaco.editor.createModel(content, "yaml", uri);
    entry = { model, viewState: null };
    modelRegistry.set(path, entry);
  }

  // Switch model and restore view state
  editor.setModel(entry.model);
  if (entry.viewState) {
    editor.restoreViewState(entry.viewState);
  }
  editor.focus();
}

// On file close: dispose model and remove from registry
function closeFileModel(path: string) {
  const entry = modelRegistry.get(path);
  if (entry) {
    entry.model.dispose();
    modelRegistry.delete(path);
  }
}
```

### Pattern 3: YAML Formatting Provider

**What:** Register a `DocumentFormattingEditProvider` that uses the `yaml` package to parse and re-stringify.
**When to use:** Once, during Monaco initialization (in `beforeMount`).

```typescript
// Source: Monaco Editor language provider API (Context7) + yaml package (Context7)
import { parse, stringify } from "yaml";

function registerYamlFormatter(monaco: typeof import("monaco-editor")): IDisposable {
  return monaco.languages.registerDocumentFormattingEditProvider("yaml", {
    provideDocumentFormattingEdits(model, options) {
      try {
        const content = model.getValue();
        const parsed = parse(content);
        const formatted = stringify(parsed, {
          indent: options.tabSize,
          lineWidth: 0,          // Disable line wrapping
          singleQuote: false,    // Use double quotes
        });
        return [
          {
            range: model.getFullModelRange(),
            text: formatted,
          },
        ];
      } catch {
        // Parse error — don't format, let validation show the error
        return [];
      }
    },
  });
}
```

### Pattern 4: Format on Save

**What:** Trigger formatting before writing file content.
**When to use:** In the save handler.

```typescript
// Trigger format then save
async function formatAndSave(editor: editor.IStandaloneCodeEditor) {
  // Trigger the registered formatting provider
  await editor.getAction("editor.action.formatDocument")?.run();
  // Then save
  const content = editor.getValue();
  await writeConfig(activeFile, content);
}
```

### Anti-Patterns to Avoid

- **Creating new models on every render:** The `@monaco-editor/react` `Editor` component creates models internally when `value` prop changes. When managing models yourself, use `path` prop or manage models via the registry pattern above.
- **Disposing the editor on file switch:** Only dispose models, not the editor. Editor recreation is expensive (~100ms). Reuse the single editor instance.
- **Calling configureMonacoYaml multiple times:** It must be called exactly once. The current code correctly uses a `schemaConfigured` ref guard. The return value is a disposable that should be stored and disposed on app unmount.
- **Ignoring the IDisposable return from event subscriptions:** `onDidChangeMarkers`, `onDidChangeCursorPosition`, etc. all return disposables that must be tracked and cleaned up.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| YAML autocomplete | Custom completion provider | `monaco-yaml` + JSON Schema | monaco-yaml already implements a full YAML language server with schema-driven completions |
| YAML validation | Custom validator | `monaco-yaml` marker output | monaco-yaml validates against JSON Schema and sets markers automatically |
| YAML syntax highlighting | Custom tokenizer | `monaco-yaml` | Included with the package |
| YAML formatting | Custom indentation logic | `yaml` package `stringify()` | Handles all YAML edge cases (multi-line strings, anchors, flow vs block) |
| Model lifecycle | Custom model pool | Monaco's built-in model registry + `createModel`/`dispose` | Monaco already tracks models by URI |

**Key insight:** The heavy lifting for autocomplete and validation is already done by `monaco-yaml`. The main work is ensuring proper configuration, cleanup, and connecting existing pieces.

## Common Pitfalls

### Pitfall 1: Schema URI Resolution in Tauri

**What goes wrong:** `configureMonacoYaml` is configured with `uri: "/schema/hooks-schema.json"` and `enableSchemaRequest: true`. In dev (Vite), this works because the dev server serves `public/` files. In Tauri production, static assets are served from the built bundle but may use a different base URL (e.g., `tauri://localhost/`).
**Why it happens:** Tauri uses a custom protocol for serving web assets.
**How to avoid:** Instead of `enableSchemaRequest: true` with a URL, embed the schema inline:
```typescript
import hooksSchema from "/schema/hooks-schema.json"; // Vite JSON import
configureMonacoYaml(monaco, {
  schemas: [{
    uri: "https://spillwave.dev/schemas/hooks-config/v1.0",
    fileMatch: ["*"],
    schema: hooksSchema,  // Inline the schema object
  }],
});
```
This avoids any HTTP fetch and works in both Vite dev and Tauri production.
**Warning signs:** Autocomplete works in `vite dev` but not in the Tauri desktop build.

### Pitfall 2: configureMonacoYaml Called Multiple Times

**What goes wrong:** React Strict Mode causes `beforeMount` to fire twice in development, potentially calling `configureMonacoYaml` twice, which throws or causes duplicate workers.
**Why it happens:** React 18 Strict Mode double-invokes effects and callbacks in development.
**How to avoid:** The current code already guards with `schemaConfigured.current` ref. This is correct. Ensure the guard persists across component remounts.
**Warning signs:** Console warnings about "monaco-yaml already configured" or duplicate completion suggestions.

### Pitfall 3: Marker Listener Not Disposed

**What goes wrong:** In `YamlEditor.tsx`, `onDidChangeMarkers` is subscribed inside `handleMount` but the returned disposable is never stored or cleaned up. If the editor remounts (e.g., React Strict Mode), multiple listeners accumulate.
**Why it happens:** The subscription is fire-and-forget inside a callback.
**How to avoid:** Store the disposable and clean up in a useEffect return:
```typescript
const markerDisposable = useRef<IDisposable | null>(null);

// In handleMount:
markerDisposable.current?.dispose();
markerDisposable.current = monaco.editor.onDidChangeMarkers(/* ... */);

// In useEffect cleanup:
useEffect(() => {
  return () => {
    markerDisposable.current?.dispose();
  };
}, []);
```
**Warning signs:** `setValidationResults` called multiple times for a single change; growing memory usage.

### Pitfall 4: YAML Formatting Loses Comments

**What goes wrong:** `yaml` package's `parse()` + `stringify()` round-trip drops YAML comments because `parse()` returns plain JS objects.
**Why it happens:** Comments are not part of the YAML data model.
**How to avoid:** Use `yaml` package's `parseDocument()` + `doc.toString()` to preserve comments:
```typescript
import { parseDocument } from "yaml";

const doc = parseDocument(content);
const formatted = doc.toString({ indent: tabSize, lineWidth: 0 });
```
`parseDocument` preserves comments, blank lines, and document structure.
**Warning signs:** User formats a file and all their comments disappear.

### Pitfall 5: Model Leak on File Switch

**What goes wrong:** When switching between files using the `value` prop of `@monaco-editor/react`'s `Editor`, the component internally creates new models but may not dispose old ones, especially if using the `defaultValue` prop or not using the `path` prop.
**Why it happens:** `@monaco-editor/react` tries to be helpful by managing models, but doesn't always clean up on prop changes.
**How to avoid:** Either:
- Use the `path` prop on `<Editor>` which tells the wrapper to reuse/create models by URI, OR
- Manage models manually via the model registry pattern (see Architecture Patterns above)
**Warning signs:** `monaco.editor.getModels().length` grows with each file switch; heap snapshot shows unreferenced `TextModel` objects.

### Pitfall 6: Format Provider Conflicts with monaco-yaml

**What goes wrong:** `monaco-yaml` may register its own formatting provider. Registering a custom one could create conflicts or the wrong one might win.
**Why it happens:** Monaco's provider resolution can be unpredictable with multiple providers for the same language.
**How to avoid:** Check if `monaco-yaml` provides formatting. If it does, configure it rather than registering a custom provider. If it doesn't (verified: it does NOT provide formatting), the custom provider is safe to register.
**Warning signs:** Format produces unexpected results or does nothing.

## Code Examples

### Example 1: Inline Schema Configuration (Avoiding Fetch)

```typescript
// Source: monaco-yaml GitHub README + verified behavior
import { configureMonacoYaml } from "monaco-yaml";
import hooksSchema from "@/../public/schema/hooks-schema.json";

export function configureYamlSchema(monaco: MonacoInstance): IDisposable {
  return configureMonacoYaml(monaco, {
    schemas: [
      {
        uri: "https://spillwave.dev/schemas/hooks-config/v1.0",
        fileMatch: ["*"],
        schema: hooksSchema as unknown as JSONSchema,
      },
    ],
  });
}
```

### Example 2: Complete YAML Formatting Provider (Comment-Preserving)

```typescript
// Source: eemeli/yaml docs (Context7) + Monaco language provider API (Context7)
import { parseDocument } from "yaml";
import type { IDisposable, languages, editor } from "monaco-editor";

export function registerYamlFormatter(
  monaco: typeof import("monaco-editor"),
): IDisposable {
  return monaco.languages.registerDocumentFormattingEditProvider("yaml", {
    provideDocumentFormattingEdits(
      model: editor.ITextModel,
      options: languages.FormattingOptions,
    ): languages.TextEdit[] {
      try {
        const content = model.getValue();
        const doc = parseDocument(content);

        // Check for parse errors — don't format broken YAML
        if (doc.errors.length > 0) {
          return [];
        }

        const formatted = doc.toString({
          indent: options.tabSize,
          lineWidth: 0,
        });

        // Avoid no-op edits
        if (formatted === content) {
          return [];
        }

        return [
          {
            range: model.getFullModelRange(),
            text: formatted,
          },
        ];
      } catch {
        return [];
      }
    },
  });
}
```

### Example 3: Proper Disposable Cleanup in YamlEditor

```typescript
// Key additions to YamlEditor.tsx for proper cleanup
const disposablesRef = useRef<IDisposable[]>([]);
const yamlDisposableRef = useRef<IDisposable | null>(null);

const handleBeforeMount: BeforeMount = useCallback((monaco) => {
  monaco.editor.defineTheme(LIGHT_THEME_NAME, lightTheme);
  monaco.editor.defineTheme(DARK_THEME_NAME, darkTheme);

  if (!schemaConfigured.current) {
    // Store the disposable returned by configureMonacoYaml
    yamlDisposableRef.current = configureYamlSchema(monaco);

    // Register YAML formatter
    const formatterDisposable = registerYamlFormatter(monaco);
    disposablesRef.current.push(formatterDisposable);

    schemaConfigured.current = true;
  }
}, []);

// Cleanup on unmount
useEffect(() => {
  return () => {
    for (const d of disposablesRef.current) {
      d.dispose();
    }
    disposablesRef.current = [];
    // Note: yamlDisposableRef is NOT disposed here because
    // configureMonacoYaml is global and should persist for app lifetime
  };
}, []);
```

### Example 4: Format on Save Integration

```typescript
// In MainContent.tsx — format before save
const handleSave = useCallback(async () => {
  if (!activeFile) return;
  const editorRef = useEditorStore.getState().editorRef;

  // Format first, then save the formatted content
  if (editorRef) {
    const formatAction = editorRef.getAction("editor.action.formatDocument");
    if (formatAction) {
      await formatAction.run();
    }
  }

  const content = useConfigStore.getState().getActiveContent();
  if (content === null) return;
  try {
    await writeConfig(activeFile, content);
    markSaved(activeFile);
  } catch (err) {
    console.error("Failed to save file:", err);
  }
}, [activeFile, markSaved]);
```

### Example 5: Model Lifecycle Management

```typescript
// editor-lifecycle.ts — utilities for model management
import type { editor } from "monaco-editor";

interface ModelEntry {
  model: editor.ITextModel;
  viewState: editor.ICodeEditorViewState | null;
}

const modelRegistry = new Map<string, ModelEntry>();

export function getOrCreateModel(
  monaco: typeof import("monaco-editor"),
  path: string,
  content: string,
): editor.ITextModel {
  const existing = modelRegistry.get(path);
  if (existing) {
    return existing.model;
  }

  const uri = monaco.Uri.parse(`file:///${path}`);
  // Check if Monaco already has this model (e.g., from @monaco-editor/react)
  const existingModel = monaco.editor.getModel(uri);
  if (existingModel) {
    modelRegistry.set(path, { model: existingModel, viewState: null });
    return existingModel;
  }

  const model = monaco.editor.createModel(content, "yaml", uri);
  modelRegistry.set(path, { model, viewState: null });
  return model;
}

export function saveViewState(
  editor: editor.IStandaloneCodeEditor,
  path: string,
): void {
  const entry = modelRegistry.get(path);
  if (entry) {
    entry.viewState = editor.saveViewState();
  }
}

export function restoreViewState(
  editor: editor.IStandaloneCodeEditor,
  path: string,
): void {
  const entry = modelRegistry.get(path);
  if (entry?.viewState) {
    editor.restoreViewState(entry.viewState);
  }
}

export function disposeModel(path: string): void {
  const entry = modelRegistry.get(path);
  if (entry) {
    entry.model.dispose();
    modelRegistry.delete(path);
  }
}

export function disposeAllModels(): void {
  for (const [, entry] of modelRegistry) {
    entry.model.dispose();
  }
  modelRegistry.clear();
}

export function getModelCount(): number {
  return modelRegistry.size;
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `monaco-yaml` v4 (complex webpack config) | `monaco-yaml` v5+ (works with Vite, no config) | v5.0.0 (2023) | No webpack/CRA ejection needed |
| `enableSchemaRequest` with URL fetch | Inline schema via `schema` property | Always available | More reliable in Tauri/Electron |
| `yaml.dump()` (js-yaml) | `parseDocument().toString()` (eemeli/yaml) | yaml v2.0+ | Comment preservation, better YAML 1.2 support |
| Manual model management | `@monaco-editor/react` `path` prop | v4.4+ | Automatic model reuse by path |

**Deprecated/outdated:**
- `js-yaml` for formatting: Use `yaml` (eemeli/yaml) instead. It has better TypeScript support, comment preservation via `parseDocument`, and active maintenance.
- `initializeMonacoYaml()`: Renamed to `configureMonacoYaml()` in monaco-yaml v5.

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Schema autocomplete already works (less work than expected) | HIGH | Positive | Verify first, then only fix what's broken |
| Schema fetch fails in Tauri production | MEDIUM | HIGH | Switch to inline schema (no fetch needed) |
| YAML formatting drops comments | HIGH (if using `parse`) | MEDIUM | Use `parseDocument` + `toString` instead |
| Memory leaks from undisposed listeners | HIGH (exists now) | MEDIUM | Track and dispose all IDisposable returns |
| `@monaco-editor/react` model management conflicts with manual management | LOW | HIGH | Choose one approach: either use `path` prop or fully manual. Don't mix. |
| monaco-yaml worker persistence after model disposal | LOW | LOW | Workers are shared; they clean up when last model is disposed |

## Dependencies and Prerequisites

### From Phase 11 (Completed)
- Settings store with `editorFontSize`, `editorTabSize`, `theme` — **DONE, already wired**
- Binary path resolution — **DONE, not needed for Phase 12**
- Rename sweep (cch → rulez) — **DONE**

### Internal Dependencies Within Phase 12
1. Schema configuration fix should be done first (affects autocomplete + validation)
2. Formatting provider registration should be done before format-on-save
3. Memory management can be done independently
4. ValidationPanel polish can be done independently

### External Dependencies
- None — all packages already installed at correct versions

## Open Questions

1. **Is autocomplete already working?**
   - What we know: `monaco-yaml` is configured with the schema. The schema is comprehensive.
   - What's unclear: Whether the schema URI fetch succeeds in both dev and Tauri. Whether autocomplete suggestions actually appear.
   - Recommendation: Test manually first. If working, just document. If not, switch to inline schema.

2. **Should we use `@monaco-editor/react` `path` prop or manual model management?**
   - What we know: The current code uses `value` prop without `path`. The `path` prop would let the wrapper handle model reuse.
   - What's unclear: Whether `path` prop conflicts with `monaco-yaml` schema matching (`fileMatch: ["*"]`).
   - Recommendation: Start with the `path` prop approach (simpler). Fall back to manual management only if needed.

3. **How to handle model disposal when configStore.closeFile is called?**
   - What we know: `configStore.closeFile` removes from `openFiles` Map. Currently no Monaco model cleanup is triggered.
   - What's unclear: Best place to put the cleanup — store middleware, component effect, or callback.
   - Recommendation: Add a `useEffect` in `MainContent.tsx` that watches `openFiles` and disposes models for closed files.

## Sources

### Primary (HIGH confidence)
- `/microsoft/monaco-editor` (Context7) — Model disposal, setModelMarkers, formatting providers, disposable pattern
- `/suren-atoyan/monaco-react` (Context7) — React wrapper lifecycle, `onMount`, `beforeMount`, `path` prop
- `/eemeli/yaml` (Context7) — `parseDocument`, `stringify`, formatting options, comment preservation
- `/redhat-developer/yaml-language-server` (Context7) — Schema-driven autocomplete and validation architecture
- Codebase analysis — YamlEditor.tsx, schema.ts, editorStore.ts, ValidationPanel.tsx, configStore.ts, hooks-schema.json

### Secondary (MEDIUM confidence)
- Perplexity (verified with official sources) — monaco-yaml v5 configuration, memory management patterns, formatting provider registration
- monaco-yaml GitHub README (https://github.com/remcohaszing/monaco-yaml) — Configuration API, schema inline support

### Tertiary (LOW confidence)
- None — all findings verified against multiple sources

## Metadata

**Confidence breakdown:**
- Standard stack: **HIGH** — All libraries already installed and partially integrated
- Architecture: **HIGH** — Patterns verified against official docs and Context7
- Pitfalls: **HIGH** — Based on actual code analysis showing existing issues
- Memory management: **HIGH** — Monaco disposal API is well-documented and stable

**Research date:** 2026-02-12
**Valid until:** 2026-03-12 (stable — all libraries are mature)

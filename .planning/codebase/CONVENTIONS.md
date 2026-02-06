# Coding Conventions

**Analysis Date:** 2026-02-06

## Naming Patterns

**Files:**
- Component files: PascalCase (e.g., `ConfirmDialog.tsx`, `AppShell.tsx`)
- Store files: camelCase with "Store" suffix (e.g., `configStore.ts`, `editorStore.ts`, `uiStore.ts`)
- Type definition files: `index.ts` (e.g., `src/types/index.ts`)
- Page objects: descriptive names with "page" suffix (e.g., `base.page.ts`, `editor.page.ts`)
- Utility files: descriptive camelCase (e.g., `tauri.ts`, `monaco-theme.ts`)

**Functions:**
- Components: PascalCase, exported as named exports
  - Example: `export function ConfirmDialog() {...}`
- Hook functions: camelCase with `use` prefix from Zustand stores
  - Example: `export const useConfigStore = create<...>`
- Action handlers: camelCase with `handle` prefix
  - Example: `handleSave()`, `handleDiscard()`, `handleSubmit()`
- Async actions: camelCase with descriptive names
  - Example: `handleRequestClose()`, `clickAndWait()`
- Utility functions: camelCase
  - Example: `loadMockConfig()`, `getBlockScenarios()`

**Variables:**
- State variables: camelCase
  - Example: `activeFile`, `openFiles`, `pendingClosePath`, `isLoading`
- Props interfaces: PascalCase with "Props" suffix
  - Example: `interface ConfirmDialogProps`, `interface FileTabProps`
- Constants: camelCase or UPPER_SNAKE_CASE for global constants
  - Example: `mockConfigs`, `QueryClient`

**Types:**
- Exported interfaces: PascalCase (e.g., `ConfigFile`, `FileState`, `CursorPosition`)
- Union types: PascalCase (e.g., `type Theme = "light" | "dark" | "system"`)
- Type imports: use `type` keyword
  - Example: `import type { ConfigFile, FileState } from "@/types"`

## Code Style

**Formatting:**
- Tool: Biome 1.9.4
- Indent: 2 spaces
- Line width: 100 characters
- Semicolons: always
- Quotes: double quotes for JavaScript/TypeScript

**Linting:**
- Tool: Biome with recommended rules
- Rules enforced:
  - `noUnusedImports`: warn
  - `noUnusedVariables`: warn
  - `noExplicitAny`: warn
  - `noNonNullAssertion`: off (allowed)
- Run: `bun run lint` or `bun run lint:fix`

**TypeScript:**
- Strict mode enabled
- Target: ES2022
- JSX: react-jsx
- Key compiler options:
  - `strict: true` - Full type safety
  - `noUnusedLocals: true` - Error on unused variables
  - `noUnusedParameters: true` - Error on unused parameters
  - `noFallthroughCasesInSwitch: true` - Prevent switch fallthrough
  - `noUncheckedIndexedAccess: true` - Safe object indexing
  - `noImplicitOverride: true` - Explicit override keyword
- Run type check: `bun run typecheck`

## Import Organization

**Order:**
1. External dependencies (React, libraries)
2. Internal absolute imports (using `@/` alias)
3. Relative imports (if any)
4. CSS imports

**Examples:**
```typescript
// React and external libraries first
import { useEffect } from "react";
import { expect, test } from "@playwright/test";

// Absolute imports using @/ alias
import { useConfigStore } from "@/stores/configStore";
import { ConfirmDialog } from "@/components/ui/ConfirmDialog";
import type { ConfigFile } from "@/types";

// CSS/styles last
import "./styles/globals.css";
```

**Path Aliases:**
- `@/*` resolves to `src/*` (configured in `tsconfig.json`)
- Always use `@/` for internal imports from `src/`
- Examples:
  - `@/stores/configStore`
  - `@/components/layout/AppShell`
  - `@/lib/tauri`
  - `@/types`

## Error Handling

**Patterns:**
- Try-catch blocks for async operations
  - Always extract error message: `err instanceof Error ? err.message : "Fallback message"`
- State-based error display (set error state, render error UI)
- Example from `DebugSimulator.tsx`:
  ```typescript
  async function handleSubmit(params: DebugParams) {
    setIsLoading(true);
    setError(null);
    try {
      const debugResult = await runDebug(params);
      setResult(debugResult);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Simulation failed");
      setResult(null);
    } finally {
      setIsLoading(false);
    }
  }
  ```

## Logging

**Framework:** `console` (standard browser console)

**Patterns:**
- No explicit logging in source files observed
- Error states managed via component state (`setError()`)
- Status messages via `useUIStore().setStatusMessage()`

## Comments

**When to Comment:**
- JSDoc blocks for exported functions and stores (see examples below)
- Inline comments for non-obvious logic
- Section separators for large components

**JSDoc/TSDoc Pattern:**
Example from `base.page.ts`:
```typescript
/**
 * Navigate to the application root
 */
async goto(): Promise<void> {
  await this.page.goto("/");
}
```

Use single-line `/**` blocks for simple functions. Include parameter and return types in the signature.

## Function Design

**Size:**
- Keep components and functions focused (most are 30-70 lines)
- Long components structured with clear sections (header, content, footer)

**Parameters:**
- Props passed as destructured interface parameter
- Example: `export function ConfirmDialog({ isOpen, title, message, onSave, onDiscard, onCancel }: ConfirmDialogProps)`
- Zustand store actions accept single parameters or destructured updates

**Return Values:**
- React components return JSX.Element
- Zustand store selectors return specific types (boolean, string, number, array, etc.)
- Async functions return Promise<T>
- Handler functions return void

**Zustand Store Pattern:**
Stores follow this structure:
```typescript
interface State {
  // State properties
}

interface Actions {
  // Action method signatures
}

export const useStore = create<State & Actions>((set, get) => ({
  // State initialization
  prop1: initialValue,

  // Action implementations
  setProp1: (value) => set({ prop1: value }),
}));
```

## Module Design

**Exports:**
- Named exports for functions, components, stores
  - Example: `export function AppShell() {...}`
  - Example: `export const useConfigStore = create<...>`
- Type exports use `export type` or `export interface`
  - Example: `export interface ConfirmDialogProps {...}`
  - Example: `export type Theme = "light" | "dark" | "system"`

**Barrel Files:**
- Used in `src/components/pages/index.ts` and `tests/pages/index.ts`
- Example from `tests/pages/index.ts`:
  ```typescript
  export { BasePage } from "./base.page";
  export { AppShellPage } from "./app-shell.page";
  // ... other exports
  ```

**Directory Structure:**
- Stores: `src/stores/`
- Components: `src/components/` with subdirectories by feature
  - `ui/` - Reusable primitives (buttons, dialogs, toggles)
  - `layout/` - App structure (header, sidebar, panels)
  - `editor/` - Editor components
  - `files/` - File operations
  - `simulator/` - Debug simulator components
- Types: `src/types/`
- Utilities: `src/lib/`
- Styles: `src/styles/`

---

*Convention analysis: 2026-02-06*

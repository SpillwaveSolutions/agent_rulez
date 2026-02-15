# Phase 16: Onboarding — Research

## Goal
First-run wizard to guide new users through setup.

## Success Criteria
1. First-time users see a setup wizard on initial app launch
2. Wizard detects whether `rulez` binary is installed and accessible via PATH
3. Wizard generates a sample `hooks.yaml` config with documented example rules
4. Wizard guides user through a test simulation to verify setup works
5. User can re-run onboarding wizard from settings panel

## What Already Exists

### E2E Tests (Complete)
- `tests/onboarding.spec.ts`: 7 tests covering all 5 success criteria
- `tests/pages/onboarding.page.ts`: POM with wizard dialog locators
- Tests expect: dialog role, buttons matching `/sample config|generate/i`, `/run test|simulate/i`, `/finish|complete|done/i`, `/skip/i`

### Settings Infrastructure
- `lib/settings.ts`: Settings interface with Tauri Store / localStorage dual-mode
- `settingsStore.ts`: Zustand store with `isLoaded` flag for startup readiness
- Missing: `onboardingComplete` field

### App Entry Point
- `App.tsx`: loads settings on startup, renders `<AppShell />` unconditionally at line 43
- This is where the wizard conditional goes

### Settings Panel
- `SettingsPanel.tsx`: theme, font size, tab size, binary path fields
- Missing: "Re-run Onboarding" button

### Binary Detection (Rust-side)
- `debug.rs`: `resolve_rulez_binary_path()` exists but not exposed standalone
- Need new `check_binary` Tauri command

### Sample Config Template
- `cch_cli/src/cli/init.rs`: `DEFAULT_HOOKS_YAML` constant (well-documented sample)
- Can duplicate in TypeScript (simpler than cross-crate sharing)
- `write_config` Tauri command creates parent directories automatically

### Existing UI Patterns
- `ConfirmDialog.tsx`: modal dialog pattern with overlay, z-50, Escape key
- `DebugSimulator.tsx` + `runDebug()`: test simulation already works

## Plan Structure

- **Plan 16-01**: Backend + settings schema + onboarding wizard UI
- **Plan 16-02**: Settings panel integration + polish

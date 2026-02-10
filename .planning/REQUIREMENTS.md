# Requirements: v1.4 Stability & Polish

## JSON Schema Validation

- REQ-SCHEMA-01: Validate incoming hook events against JSON Schema before rule processing
- REQ-SCHEMA-02: Generate schema automatically from Event struct using schemars derive macro
- REQ-SCHEMA-03: Pre-compile schema validators at startup (LazyLock) for <0.1ms validation
- REQ-SCHEMA-04: Fail-open semantics — log warnings on invalid events but continue processing
- REQ-SCHEMA-05: Exit code 1 (config error) on malformed event JSON, not exit code 2 (block)
- REQ-SCHEMA-06: Support JSON Schema draft-07 and 2020-12

## Debug CLI Enhancements

- REQ-DEBUG-01: Add UserPromptSubmit to SimEventType enum with aliases (prompt, user-prompt)
- REQ-DEBUG-02: Add --prompt flag to debug CLI for specifying prompt text
- REQ-DEBUG-03: Clear REGEX_CACHE at start of debug run() for state isolation
- REQ-DEBUG-04: Improved debug output — show matched rules, actions taken, timing info
- REQ-DEBUG-05: Reuse existing process_event() pipeline (no debug-specific logic in hooks.rs)

## E2E Test Fixes

- REQ-E2E-01: Add canonicalize_path() helper to resolve symlinks (macOS /var -> /private/var)
- REQ-E2E-02: All E2E tests use canonical paths in event setup
- REQ-E2E-03: Fix broken pipe issues — use Stdio::null() or wait_with_output() as appropriate
- REQ-E2E-04: CI matrix includes ubuntu-latest, macos-latest, windows-latest for E2E tests
- REQ-E2E-05: Binary artifact validation — verify rulez binary exists before test execution

## Tauri UI Build & CI

- REQ-TAURI-01: Create .github/workflows/tauri-build.yml for cross-platform builds
- REQ-TAURI-02: Linux CI uses ubuntu-22.04 with libwebkit2gtk-4.1-dev (NOT 4.0)
- REQ-TAURI-03: E2E tests run in web mode (Playwright) before Tauri build
- REQ-TAURI-04: Multi-platform build matrix (Linux, macOS, Windows)
- REQ-TAURI-05: Upload build artifacts (.dmg, .msi, .AppImage)
- REQ-TAURI-06: Fix e2e.yml workflow directory mismatch (rulez_ui -> rulez-ui)

## Cross-Cutting

- REQ-PERF-01: Total event processing latency remains <10ms p95
- REQ-PERF-02: Binary size remains <5MB
- REQ-COMPAT-01: Zero breaking changes — all features are additive
- REQ-COMPAT-02: All 605+ existing tests continue to pass

---

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| REQ-SCHEMA-01 | Phase 7 | Pending |
| REQ-SCHEMA-02 | Phase 7 | Pending |
| REQ-SCHEMA-03 | Phase 7 | Pending |
| REQ-SCHEMA-04 | Phase 7 | Pending |
| REQ-SCHEMA-05 | Phase 7 | Pending |
| REQ-SCHEMA-06 | Phase 7 | Pending |
| REQ-DEBUG-01 | Phase 8 | Pending |
| REQ-DEBUG-02 | Phase 8 | Pending |
| REQ-DEBUG-03 | Phase 8 | Pending |
| REQ-DEBUG-04 | Phase 8 | Pending |
| REQ-DEBUG-05 | Phase 8 | Pending |
| REQ-E2E-01 | Phase 9 | Pending |
| REQ-E2E-02 | Phase 9 | Pending |
| REQ-E2E-03 | Phase 9 | Pending |
| REQ-E2E-04 | Phase 9 | Pending |
| REQ-E2E-05 | Phase 9 | Pending |
| REQ-TAURI-01 | Phase 10 | Pending |
| REQ-TAURI-02 | Phase 10 | Pending |
| REQ-TAURI-03 | Phase 10 | Pending |
| REQ-TAURI-04 | Phase 10 | Pending |
| REQ-TAURI-05 | Phase 10 | Pending |
| REQ-TAURI-06 | Phase 10 | Pending |
| REQ-PERF-01 | Phase 7 | Pending |
| REQ-PERF-02 | Phase 7 | Pending |
| REQ-COMPAT-01 | Phase 9 | Pending |
| REQ-COMPAT-02 | Phase 9 | Pending |

---

*Updated 2026-02-10 — v1.4 Stability & Polish roadmap created*

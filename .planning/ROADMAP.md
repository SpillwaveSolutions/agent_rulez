# RuleZ Core Roadmap

**Current Focus:** v1.4 Stability & Polish (Phases 7-10)

---

## Milestones

- ✅ **v1.2 P2 Features** — Phases 1-3 (shipped 2026-02-07) — [Archive](milestones/v1.2-ROADMAP.md)
- ✅ **v1.3 Advanced Matching & Validation** — Phases 4-6 (shipped 2026-02-10) — [Archive](milestones/v1.3-ROADMAP.md)
- 🚧 **v1.4 Stability & Polish** — Phases 7-10 (active)

---

## Completed: v1.2 P2 Features

<details>
<summary>✅ v1.2 P2 Features (Phases 1-3) — SHIPPED 2026-02-07</summary>

- [x] Phase 1: Inline Content Injection (1/1 plans) — inject_inline field
- [x] Phase 2: Command-Based Context Generation (2/2 plans) — inject_command field
- [x] Phase 3: Conditional Rule Activation (3/3 plans) — enabled_when field

See [v1.2-ROADMAP.md](milestones/v1.2-ROADMAP.md) for full details.

</details>

## Completed: v1.3 Advanced Matching & Validation

<details>
<summary>✅ v1.3 Advanced Matching & Validation (Phases 4-6) — SHIPPED 2026-02-10</summary>

- [x] Phase 4: Prompt Matching (4/4 plans) — regex intent routing with AND/OR logic
- [x] Phase 5: Field Validation (3/3 plans) — fail-closed field existence and type checks
- [x] Phase 6: Inline Script Blocks (3/3 plans) — evalexpr expressions and shell scripts in YAML

See [v1.3-ROADMAP.md](milestones/v1.3-ROADMAP.md) for full details.

</details>

---

## Active: v1.4 Stability & Polish

**Goal:** Close infrastructure gaps from v1.3 technical debt. Focus on JSON Schema validation, debug CLI parity, cross-platform E2E reliability, and Tauri 2.0 CI integration.

**Depth:** Standard (4 phases, following natural infrastructure boundaries)

**Coverage:** 23/23 requirements mapped ✓

### Phase 7: JSON Schema Validation

**Goal:** Validate incoming hook events against JSON Schema to catch malformed payloads before rule processing.

**Dependencies:** None (independent from other v1.4 phases)

**Requirements:** REQ-SCHEMA-01, REQ-SCHEMA-02, REQ-SCHEMA-03, REQ-SCHEMA-04, REQ-SCHEMA-05, REQ-SCHEMA-06, REQ-PERF-01, REQ-PERF-02

**Success Criteria:**
1. User submits malformed event JSON and receives exit code 1 with clear validation error message (not exit code 2)
2. User submits valid event and processing completes in <10ms p95 (schema validation adds <0.1ms overhead)
3. User updates to schema with unsupported draft version and binary fails config load with explicit error
4. User submits event with invalid structure and sees warning logged to audit trail, but processing continues (fail-open mode)
5. Binary size remains <5MB after adding schemars dependency

**Key Deliverables:**
- Add schemars 1.2.1 dependency to Cargo.toml
- Derive JsonSchema trait on Event and related structs
- Pre-compile schema validators at startup using LazyLock
- Add validate_event_schema() function called before process_event()
- Implement draft version validation (only draft-07 and 2020-12 allowed)
- Fail-open mode: log warnings for invalid events, continue processing
- Performance regression tests in CI (criterion benchmarks)

### Phase 8: Debug CLI Enhancements

**Goal:** Close testing gap for UserPromptSubmit events and improve debug CLI output quality.

**Dependencies:** None (independent from other v1.4 phases)

**Requirements:** REQ-DEBUG-01, REQ-DEBUG-02, REQ-DEBUG-03, REQ-DEBUG-04, REQ-DEBUG-05

**Success Criteria:**
1. User runs `rulez debug prompt --prompt "test text"` and sees event processed with matching rules
2. User runs debug command twice and second invocation has clean state (no REGEX_CACHE contamination)
3. User sees which rules matched, actions taken, and timing info in debug output
4. User can simulate UserPromptSubmit events with same pipeline as real events (no debug-specific code paths)

**Key Deliverables:**
- Add UserPromptSubmit variant to SimEventType enum with aliases
- Add --prompt flag to debug CLI args parser
- Clear REGEX_CACHE at start of debug run() for state isolation
- Implement LRU cache for REGEX_CACHE (max 100 entries, fixes unbounded growth)
- Enhance debug output with matched rules, actions, timing
- Reuse process_event() pipeline (no special debug logic in hooks.rs)
- Add state isolation tests to verify no cross-invocation leakage

### Phase 9: E2E Test Stabilization

**Goal:** Make E2E tests pass reliably on Linux, macOS, and Windows in CI (currently fail on Ubuntu).

**Dependencies:** Phase 7 and Phase 8 (validates schema validation and debug CLI work correctly)

**Requirements:** REQ-E2E-01, REQ-E2E-02, REQ-E2E-03, REQ-E2E-04, REQ-E2E-05, REQ-COMPAT-01, REQ-COMPAT-02

**Success Criteria:**
1. User runs E2E tests on macOS and paths resolve correctly despite /var symlink to /private/var
2. User runs E2E tests on Linux and no broken pipe errors occur from unread stdio
3. User runs full test suite on all three platforms in CI matrix and all 605+ tests pass
4. User checks binary artifact name before tests run and sees validation that correct binary exists

**Key Deliverables:**
- Add canonicalize_path() helper in tests/common/mod.rs using fs::canonicalize()
- Update all E2E tests to use canonical paths in event setup
- Fix broken pipe issues: use wait_with_output() instead of spawn() + wait()
- Add CI matrix with ubuntu-latest, macos-latest, windows-latest
- Binary artifact validation step (verify `which rulez` returns expected path)
- Explicit tempfile cleanup with drop(temp_dir) at test end
- Symlink resolution tests (Unix-only, validates canonicalization)

### Phase 10: Tauri CI Integration

**Goal:** Build Tauri desktop app for all platforms in CI and upload release artifacts.

**Dependencies:** Phase 9 (E2E tests must pass before Tauri builds are useful)

**Requirements:** REQ-TAURI-01, REQ-TAURI-02, REQ-TAURI-03, REQ-TAURI-04, REQ-TAURI-05, REQ-TAURI-06

**Success Criteria:**
1. User pushes to release branch and GitHub Actions builds .dmg (macOS), .msi (Windows), .AppImage (Linux)
2. User sees E2E tests run first in web mode (fast, 2-3 min), Tauri builds only start if tests pass (fail-fast)
3. User runs Tauri build on ubuntu-22.04 and webkit2gtk-4.1 installs successfully (NOT 4.0)
4. User downloads build artifacts and desktop app launches on all three platforms

**Key Deliverables:**
- Create .github/workflows/tauri-build.yml workflow file
- E2E test job runs first in web mode (Playwright, fast feedback)
- Tauri build job depends on E2E test success (fail-fast pattern)
- Linux build uses explicit ubuntu-22.04 runner with libwebkit2gtk-4.1-dev
- Multi-platform build matrix (ubuntu-22.04, macos-latest, windows-latest)
- Upload artifacts for .dmg, .msi, .AppImage (release branches only)
- Fix e2e.yml workflow directory mismatch (rulez_ui -> rulez-ui)

---

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Inline Content Injection | v1.2 | 1/1 | ✅ Complete | 2026-02-06 |
| 2. Command-Based Context | v1.2 | 2/2 | ✅ Complete | 2026-02-06 |
| 3. Conditional Rule Activation | v1.2 | 3/3 | ✅ Complete | 2026-02-07 |
| 4. Prompt Matching | v1.3 | 4/4 | ✅ Complete | 2026-02-09 |
| 5. Field Validation | v1.3 | 3/3 | ✅ Complete | 2026-02-09 |
| 6. Inline Script Blocks | v1.3 | 3/3 | ✅ Complete | 2026-02-09 |
| 7. JSON Schema Validation | v1.4 | 0/? | 🔄 Planning | — |
| 8. Debug CLI Enhancements | v1.4 | 0/? | 🔄 Planning | — |
| 9. E2E Test Stabilization | v1.4 | 0/? | 🔄 Planning | — |
| 10. Tauri CI Integration | v1.4 | 0/? | 🔄 Planning | — |

---

*Created 2026-02-06 — Updated 2026-02-10 v1.4 milestone started*

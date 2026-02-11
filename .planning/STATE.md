# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-10)

**Core value:** LLMs do not enforce policy. LLMs are subject to policy.
**Current focus:** v1.4 Stability & Polish

## Current Position

Milestone: v1.4 Stability & Polish
Phase: 10 — Tauri CI Integration (2/2 plans complete)
Current Plan: Completed
Status: Phase 10 complete — ready for next phase
Last activity: 2026-02-11 — Completed Phase 10 Plan 02 (Tauri CI Build Workflow)

Progress: ████████████████░░░░ 70%

## Performance Metrics

**Velocity:**
- Total plans completed: 27 (6 v1.2 + 10 v1.3 + 11 v1.4)
- Average duration: ~5min (Phases 4-10)
- Total execution time: 137min (Phase 4: 61min, Phase 5: 21min, Phase 6: 18min, Phase 7: 7min, Phase 8: 12min, Phase 9: 16min, Phase 10: 2min)

**Phase 7 Progress:**
| Plan | Duration | Tasks | Files | Status |
|------|----------|-------|-------|--------|
| 07-01 | 7min | 2 | 5 (1 new) | Complete |
| 07-02 | 8min | 2 | 2 (1 new) | Complete |

**Phase 8 Progress:**
| Plan | Duration | Tasks | Files | Status |
|------|----------|-------|-------|--------|
| 08-01 | 12min | 2 | 2 | Complete |
| 08-02 | 16min | 1 | 3 | Complete |

**Phase 9 Progress:**
| Plan | Duration | Tasks | Files | Status |
|------|----------|-------|-------|--------|
| 09-01 | 4min | 2 | 2 | Complete |
| 09-02 | 6min | 2 | 1 | Complete |
| 09-03 | 6min | 2 | 2 (1 new) | Complete |

**Phase 10 Progress:**
| Plan | Duration | Tasks | Files | Status |
|------|----------|-------|-------|--------|
| 10-01 | <1min | 1 | 1 | Complete |
| 10-02 | 1min | 1 | 1 (1 new) | Complete |

## Verification Results

| Phase | Score | Status |
|-------|-------|--------|
| 7 - JSON Schema Validation | 10/10 must-haves | PASSED |
| 8 - Debug CLI Enhancements | 8/8 must-haves | PASSED |

## Accumulated Context

### Decisions

**Phase 10 - Tauri CI Integration (10-02):**
- E2E tests run FIRST in web mode (2-3min) before any Tauri builds start
- Linux builds use explicit ubuntu-22.04 with libwebkit2gtk-4.1-dev (NOT ubuntu-latest or webkit 4.0)
- Matrix uses fail-fast: false to allow all platform builds to complete for diagnostics
- Rust cache targets rulez-ui/src-tauri workspace for faster incremental builds
- Auto-release on version tags using conditional expressions in tauri-action parameters

**Phase 10 - Tauri CI Integration (10-01):**
- All rulez_ui references replaced with rulez-ui to match actual directory name

**Phase 9 - E2E Test Stabilization (09-03):**
- Import std::os::unix::fs::symlink inside #[cfg(unix)] test function body (not at module level) to avoid compilation errors on non-Unix platforms
- Use explicit drop() for single temp_dirs, rely on loop scope drop for loop-created temp_dirs
- Symlink tests validate both blocking and allowing behavior to ensure config resolution works correctly

**Phase 9 - E2E Test Stabilization (09-02):**
- Run E2E tests on both ubuntu-latest and macos-latest in GitHub Actions
- Symlink tests (#[cfg(unix)]) automatically skipped on Windows
- Matrix strategy enables parallel cross-platform validation

**Phase 9 - E2E Test Stabilization (09-01):**
- Use canonicalize_path() wrapper with fallback instead of raw fs::canonicalize()
- Apply canonicalization at event JSON creation time to match binary's internal canonicalization

**Phase 8 - Debug CLI Enhancements (08-02):**
- Replace unbounded HashMap REGEX_CACHE with LRU cache (100 entry cap)
- Use lock-based test isolation to prevent parallel test interference
- Test LRU behavior directly with cache.put()/get() rather than through helper functions

**Phase 8 - Debug CLI Enhancements (08-01):**
- Export REGEX_CACHE from hooks module for debug CLI state isolation
- Clear REGEX_CACHE at start of each debug invocation for clean test runs
- Maintain comprehensive integration test coverage for all debug event types

**Phase 7 - JSON Schema Validation (07-02):**
- Performance test allows 2s wall-clock time (not 100ms) to account for process spawn overhead
- Tracing logger outputs to stdout, not stderr - tests check stdout for error messages
- Binary size test is ignored by default (requires release build)
- Test names accurately reflect behavior: fail-open schema validation vs fail-closed serde deserialization

**Phase 7 - JSON Schema Validation (07-01):**
- Schema validation is fail-open: logs warnings but continues processing
- Serde deserialization is fail-closed: missing required fields are fatal
- Schema is auto-generated from Event struct using schemars 1.2
- Pre-compile validator at startup using LazyLock for <0.1ms validation time
- Three-phase event processing: parse JSON -> validate schema (fail-open) -> deserialize Event (fail-closed)

All historical decisions logged in PROJECT.md Key Decisions table.

### Pending Todos

0 pending

### Blockers/Concerns

None active.

## Session Continuity

Last session: 2026-02-11
Stopped at: Completed Phase 10 (Tauri CI Integration)
Resume file: None

Next action: Phase 10 complete — ready for verification or next phase

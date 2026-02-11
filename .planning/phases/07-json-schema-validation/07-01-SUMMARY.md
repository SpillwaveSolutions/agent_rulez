---
phase: 07-json-schema-validation
plan: 01
subsystem: validation
tags: [json-schema, schemars, jsonschema, validation, event-parsing]

# Dependency graph
requires:
  - phase: 06-hook-logging
    provides: Event and EventType types that need schema validation
provides:
  - Auto-generated JSON Schema for Event struct using schemars 1.2
  - Pre-compiled schema validator using LazyLock for zero-overhead validation
  - Fail-open schema validation (warns but continues on deviations)
  - Fail-closed serde deserialization (fatal on missing required fields)
  - Clear error messages for malformed JSON (exit code 1)
affects: [future-schema-export, api-documentation, event-handling]

# Tech tracking
tech-stack:
  added: [schemars 1.2, jsonschema 0.28]
  patterns: [fail-open validation, LazyLock for static initialization, schema auto-generation]

key-files:
  created: [rulez/src/schema.rs]
  modified: [Cargo.toml, rulez/Cargo.toml, rulez/src/models.rs, rulez/src/main.rs]

key-decisions:
  - "Schema validation is fail-open: logs warnings but continues processing"
  - "Serde deserialization is fail-closed: missing required fields are fatal"
  - "Schema is auto-generated from Event struct, not maintained manually"
  - "Pre-compile validator at startup using LazyLock for <0.1ms validation time"
  - "Use jsonschema 0.28 (not 0.41 which doesn't exist for Rust crate)"

patterns-established:
  - "Three-phase event processing: parse JSON -> validate schema (fail-open) -> deserialize Event (fail-closed)"
  - "LazyLock pattern for compile-time static initialization"
  - "Schema draft version 2020-12 via schemars 1.2"

# Metrics
duration: 7min
completed: 2026-02-10
---

# Phase 7 Plan 1: JSON Schema Validation Summary

**Auto-generated JSON Schema validation with fail-open semantics using schemars 1.2 and jsonschema 0.28, pre-compiled LazyLock validator, and three-phase event processing**

## Performance

- **Duration:** 7 minutes
- **Started:** 2026-02-10T23:58:30Z
- **Completed:** 2026-02-10T00:05:37Z
- **Tasks:** 2
- **Files modified:** 5 (1 created)

## Accomplishments
- JSON Schema auto-generated from Event struct with schemars 1.2 (draft 2020-12)
- Pre-compiled validator using LazyLock for thread-safe, one-time initialization
- Schema validation integrated into event processing pipeline with fail-open semantics
- Clear distinction between fail-open schema validation and fail-closed serde deserialization
- All 605 existing tests pass

## Task Commits

Each task was committed atomically:

1. **Task 1: Add dependencies and derive JsonSchema on Event types, create schema module** - `eb12c7e` (feat)
2. **Task 2: Integrate schema validation into process_hook_event pipeline** - `c7ecd59` (feat)

## Files Created/Modified
- `Cargo.toml` - Added schemars 1.2 and jsonschema 0.28 to workspace dependencies
- `rulez/Cargo.toml` - Added schemars and jsonschema dependencies to rulez package
- `rulez/src/models.rs` - Added JsonSchema import and derive to Event and EventType, schemars annotation for DateTime<Utc>
- `rulez/src/cli/debug.rs` - Fixed pre-existing bug: Timing.len() -> Timing.rules_evaluated
- `rulez/src/schema.rs` - NEW: Schema validation module with validate_event_schema(), generate_event_schema(), schema_draft_version()
- `rulez/src/main.rs` - Added schema module declaration, modified process_hook_event to use three-phase validation

## Decisions Made
- **Schema is fail-open:** Extra fields, wrong optional types, etc. produce warnings but continue processing (REQ-SCHEMA-04)
- **Serde is fail-closed:** Missing required fields (hook_event_name, session_id) cannot construct Event struct, so this is fatal
- **Three-phase processing:** Parse raw JSON -> validate schema (fail-open) -> deserialize to Event (fail-closed)
- **LazyLock initialization:** Pre-compile validator at startup for <0.1ms validation time (REQ-SCHEMA-03)
- **DateTime<Utc> handling:** Use `#[schemars(with = "String")]` to represent chrono DateTime as String in schema
- **jsonschema version:** Use 0.28 (not 0.41 which was mentioned in research but doesn't exist for Rust crate)
- **Exit codes:** Malformed JSON exits with 1, serde deserialization errors exit with 1 (both are config/data errors, not policy blocks which exit with 2)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed debug.rs attempting to call .len() on Timing struct**
- **Found during:** Task 1 (compilation after adding JsonSchema derives)
- **Issue:** debug.rs line 101 called `t.len()` on a `Timing` struct which has no `len()` method, causing compilation failure
- **Fix:** Changed `t.len()` to `t.rules_evaluated` to access the correct field
- **Files modified:** rulez/src/cli/debug.rs
- **Verification:** Code compiles, clippy passes with -D warnings
- **Committed in:** eb12c7e (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Bug fix was necessary for compilation. Pre-existing issue surfaced by stricter compilation with new dependencies. No scope creep.

## Issues Encountered
None - both tasks executed smoothly once pre-existing bug was fixed.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- JSON Schema validation is fully integrated and tested
- Schema can be exported in future phases for API documentation or external validation
- Pre-compiled validator pattern can be reused for other validation needs
- All 605 tests pass, no regressions introduced

## Self-Check: PASSED

All files verified:
- FOUND: Cargo.toml
- FOUND: rulez/Cargo.toml
- FOUND: rulez/src/models.rs
- FOUND: rulez/src/cli/debug.rs
- FOUND: rulez/src/schema.rs (new)
- FOUND: rulez/src/main.rs

All commits verified:
- FOUND: eb12c7e (Task 1)
- FOUND: c7ecd59 (Task 2)

---
*Phase: 07-json-schema-validation*
*Completed: 2026-02-10*

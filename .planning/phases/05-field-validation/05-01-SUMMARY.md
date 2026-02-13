---
phase: 05-field-validation
plan: 01
subsystem: validation
tags: [field-validation, json-pointer, config-validation, serde, rust]

# Dependency graph
requires:
  - phase: 04-prompt-matching
    provides: Matchers struct pattern with validation at config load time
provides:
  - require_fields and field_types fields in Matchers struct for tool_input JSON validation
  - dot_to_pointer utility function for converting dot notation to JSON Pointer (RFC 6901)
  - Config-time validation for field paths and type specifiers
affects: [06-field-matching-logic, field-validation]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Field path validation at config load time (fail-fast pattern)"
    - "Dot notation to JSON Pointer conversion (RFC 6901 compliant)"
    - "Helper method pattern for validation (validate_field_path)"

key-files:
  created: []
  modified:
    - rulez/src/models.rs
    - rulez/src/config.rs
    - rulez/src/hooks.rs

key-decisions:
  - "Use dot notation for field paths in YAML (user-friendly) with RFC 6901 JSON Pointer conversion"
  - "Validate field paths at config load time (fail-fast) not at match time (fail-late)"
  - "Support 6 type specifiers: string, number, boolean, array, object, any"
  - "field_types implicitly requires field existence (no separate require_fields needed when using field_types)"

patterns-established:
  - "Field path validation: reject empty, leading/trailing/consecutive dots"
  - "Type specifier validation: whitelist of 6 valid types"
  - "Helper method pattern: extract validation logic for reuse (validate_field_path)"

# Metrics
duration: 8min
completed: 2026-02-09
---

# Phase 5 Plan 01: Field Validation Foundation Summary

**Field validation data model with require_fields/field_types in Matchers, dot-to-pointer conversion utility, and config-time validation for paths and type specifiers**

## Performance

- **Duration:** 8 min
- **Started:** 2026-02-09T22:44:19Z
- **Completed:** 2026-02-09T22:52:19Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Added require_fields (Vec<String>) and field_types (HashMap<String, String>) to Matchers struct
- Implemented dot_to_pointer utility function with RFC 6901 escaping (~ and / handling)
- Config-time validation catches malformed field paths and invalid type specifiers before runtime
- 16 new tests added (5 dot_to_pointer unit tests, 11 validation tests)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add require_fields, field_types to Matchers struct and dot_to_pointer utility** - `099ea6a` (feat)
2. **Task 2: Add config-time validation for require_fields and field_types** - `117e90a` (feat)

## Files Created/Modified
- `rulez/src/models.rs` - Added require_fields and field_types fields to Matchers, dot_to_pointer utility, 5 unit tests
- `rulez/src/config.rs` - Added validate_field_path helper method, require_fields/field_types validation logic, 11 validation tests
- `rulez/src/hooks.rs` - Updated test Matchers instantiations to include new fields

## Decisions Made

**1. Dot notation for field paths (user-friendly)**
- Users write `"user.name"` not `"/user/name"` in YAML
- dot_to_pointer converts to JSON Pointer format at validation/match time
- Follows principle: make easy things easy (simple field paths are simple to write)

**2. Validate at config load time (fail-fast)**
- Invalid field paths rejected when config loads, not when rule matches
- Consistent with existing prompt_match validation pattern from Phase 4
- Better UX: user sees error immediately on rulez validate

**3. Type specifiers whitelist**
- Support 6 types: string, number, boolean, array, object, any
- Matches JSON type system (not TypeScript or JSON Schema)
- "any" allows opt-out of type checking while still requiring field existence

**4. field_types implicitly requires existence**
- If field_types specifies a type for "user.name", that field must exist
- No need to also add "user.name" to require_fields
- Reduces YAML verbosity

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated all Matchers test instantiations across codebase**
- **Found during:** Task 1 (after adding new fields to Matchers struct)
- **Issue:** Compilation failed because existing test code instantiated Matchers without the new require_fields and field_types fields
- **Fix:** Added `require_fields: None, field_types: None,` to all existing Matchers literals in models.rs (7 instances), config.rs (13 instances), and hooks.rs (16 instances)
- **Files modified:** rulez/src/models.rs, rulez/src/config.rs, rulez/src/hooks.rs
- **Verification:** cargo check passes, all 179 unit tests pass
- **Committed in:** 099ea6a (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Auto-fix was necessary to unblock compilation. This is standard practice when adding fields to structs - all existing instantiations must be updated. No scope creep.

## Issues Encountered

None - plan executed smoothly. The deviation (updating test instantiations) was expected and handled via Rule 3.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

**Ready for Plan 02 (Field Matching Logic):**
- Matchers struct has require_fields and field_types fields
- dot_to_pointer utility function available for JSON Pointer conversion
- Config validation ensures only valid field paths and type specifiers reach matching logic
- All 418 existing tests pass (407 from Phase 4 + 11 new validation tests)

**Blocker check:** None - all prerequisites for matching logic are in place.

---
*Phase: 05-field-validation*
*Completed: 2026-02-09*

## Self-Check: PASSED

All files and commits verified:
- ✓ rulez/src/models.rs exists
- ✓ rulez/src/config.rs exists
- ✓ rulez/src/hooks.rs exists
- ✓ 099ea6a commit exists
- ✓ 117e90a commit exists

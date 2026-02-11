---
phase: 05-field-validation
plan: 02
subsystem: validation
tags: [field-validation, runtime-matching, fail-closed, json-pointer, rust]

# Dependency graph
requires:
  - phase: 05-field-validation
    plan: 01
    provides: require_fields/field_types in Matchers, dot_to_pointer utility, config validation
provides:
  - validate_required_fields function for runtime field validation
  - Field validation integrated as matcher condition in matches_rule
  - field_validation_matched debug tracking in MatcherResults
affects: [06-field-validation-integration-tests, field-validation]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Field validation as matcher condition (blocks rule when validation fails)"
    - "Fail-closed on missing tool_input, null values, type mismatches"
    - "Error accumulation pattern (collect all errors before returning)"
    - "Type-only error messages (security - never leak field values)"

key-files:
  created: []
  modified:
    - rulez/src/models.rs
    - rulez/src/hooks.rs

key-decisions:
  - "Field validation integrated as final matcher check (after prompt_match, before rule matches)"
  - "Fail-closed behavior: missing tool_input causes all validation to fail"
  - "Null values treated as missing (not as valid JSON null)"
  - "Error accumulation: collect ALL errors before logging (don't short-circuit on first failure)"
  - "Security: error messages show types only, never actual field values"
  - "Empty strings and arrays count as present (JSON semantics, not application semantics)"

patterns-established:
  - "Field validation execution order: require_fields + field_types combined, checked together"
  - "Debug mode tracking: field_validation_matched added to MatcherResults"
  - "Test coverage pattern: 12 unit tests covering all validation scenarios"

# Metrics
duration: 5min
completed: 2026-02-09
---

# Phase 5 Plan 02: Field Matching Logic Summary

**Runtime field validation with validate_required_fields function integrated into rule matching, fail-closed behavior for missing/null/mismatched fields, and comprehensive test coverage**

## Performance

- **Duration:** 5 min
- **Started:** 2026-02-09T22:55:09Z
- **Completed:** 2026-02-09T23:00:43Z
- **Tasks:** 2
- **Files modified:** 2
- **Tests added:** 12 unit tests

## Accomplishments

- Implemented validate_required_fields function with fail-closed behavior
- Integrated field validation into matches_rule and matches_rule_with_debug
- Added field_validation_matched to MatcherResults for debug tracking
- Rules with require_fields/field_types now block when validation fails
- 12 comprehensive unit tests covering all validation scenarios
- All 430 existing tests pass (191 unit tests x 2 + integration tests)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add field_validation_matched to MatcherResults and implement validate_required_fields** - `4c8155e` (feat)
2. **Task 2: Integrate field validation into matches_rule and matches_rule_with_debug** - `ee022fb` (feat)

## Files Created/Modified

- `rulez/src/models.rs` - Added field_validation_matched field to MatcherResults
- `rulez/src/hooks.rs` - Added validate_required_fields function (129 lines), integrated into matches_rule and matches_rule_with_debug, added 12 unit tests (648 lines total)

## Decisions Made

**1. Field validation as final matcher condition**
- Positioned after prompt_match check, before final `true` return in matches_rule
- Ensures field validation acts as an additional requirement for rule matching
- Consistent with fail-closed philosophy: if validation fails, rule doesn't match

**2. Fail-closed on missing tool_input**
- If event has no tool_input, validation fails immediately
- Logs warning and returns false (safe default)
- Prevents rules from accidentally matching when they shouldn't

**3. Null values treated as missing**
- JSON null is semantically different from field absence, but treated the same for validation
- Explicit choice: require_fields means "field must have a non-null value"
- Error message distinguishes: "field 'x' is null (treated as missing)"

**4. Error accumulation (not short-circuit)**
- Collect ALL validation errors before returning
- Join errors with "; " in single warning log
- Better UX: user sees all problems at once, not just first one

**5. Type-only error messages (security)**
- Never log actual field values (could leak sensitive data)
- Error format: "field 'count' expected number, got string"
- Shows what's wrong without exposing PII/credentials

**6. Empty strings and arrays count as present**
- `{"command": ""}` passes require_fields check for "command"
- `{"items": []}` passes require_fields check for "items"
- Follows JSON semantics: empty is different from null/missing
- Application-level emptiness checks belong in validators, not matchers

## Deviations from Plan

None - plan executed exactly as written. All 12 tests specified in plan were implemented and pass.

## Test Coverage

### Unit Tests Added (12 tests)

All tests in `rulez/src/hooks.rs`:

1. `test_field_validation_no_fields_configured` - Rule with no require_fields/field_types passes
2. `test_field_validation_missing_tool_input` - Missing tool_input fails (fail-closed)
3. `test_field_validation_present_field` - Present required field passes
4. `test_field_validation_missing_field` - Missing required field fails
5. `test_field_validation_null_field_is_missing` - Null field treated as missing
6. `test_field_validation_nested_field` - Dot notation "user.name" resolves correctly
7. `test_field_validation_type_match` - field_types with correct type passes
8. `test_field_validation_type_mismatch` - field_types with wrong type fails
9. `test_field_validation_empty_string_is_present` - Empty string counts as present
10. `test_field_validation_empty_array_is_present` - Empty array counts as present
11. `test_field_validation_any_type` - "any" type accepts any non-null value
12. `test_field_validation_field_types_implies_existence` - field_types requires field to exist

### Test Results

```
running 12 tests
test hooks::tests::test_field_validation_missing_tool_input ... ok
test hooks::tests::test_field_validation_no_fields_configured ... ok
test hooks::tests::test_field_validation_type_match ... ok
test hooks::tests::test_field_validation_empty_array_is_present ... ok
test hooks::tests::test_field_validation_empty_string_is_present ... ok
test hooks::tests::test_field_validation_present_field ... ok
test hooks::tests::test_field_validation_field_types_implies_existence ... ok
test hooks::tests::test_field_validation_any_type ... ok
test hooks::tests::test_field_validation_missing_field ... ok
test hooks::tests::test_field_validation_nested_field ... ok
test hooks::tests::test_field_validation_null_field_is_missing ... ok
test hooks::tests::test_field_validation_type_mismatch ... ok

test result: ok. 12 passed; 0 failed
```

**Total test count:** 430 tests (191 unit tests x 2 binaries + integration tests)

**No regressions:** All existing Phase 4 and earlier tests continue to pass.

## Issues Encountered

None - implementation went smoothly. The validate_required_fields function worked correctly on first try.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

**Ready for Plan 03 (Field Validation Integration Tests):**
- validate_required_fields function fully implemented and tested
- Field validation integrated into rule matching logic
- Debug mode tracks field_validation_matched
- All must-have truths verified by unit tests
- Fail-closed behavior confirmed
- Error accumulation pattern working

**Blocker check:** None - all prerequisites for integration tests are in place.

## Technical Implementation Notes

### validate_required_fields Function

**Location:** `rulez/src/hooks.rs` (lines 127-254)

**Algorithm:**
1. Check if rule has require_fields or field_types (return true if neither)
2. Get tool_input from event (fail-closed if missing or not an object)
3. Build combined field set: require_fields + field_types keys
4. For each field:
   - Convert dot path to JSON Pointer via dot_to_pointer()
   - Look up value with tool_input.pointer(&pointer_path)
   - Check presence: None or Null -> error
   - Check type (if in field_types): mismatch -> error
5. If any errors, log all together and return false
6. Otherwise return true

**Error message format:**
- Missing: `"field 'command' is missing"`
- Null: `"field 'command' is null (treated as missing)"`
- Type mismatch: `"field 'count' expected number, got string"`

**Performance:**
- O(n) where n = number of required fields
- No regex compilation (unlike prompt_match)
- JSON Pointer lookup is O(depth) per field
- Total: O(n * max_field_depth) - typically < 1ms for typical rules

### Integration Points

**matches_rule function:**
```rust
// Check field validation (require_fields / field_types)
if rule.matchers.require_fields.is_some() || rule.matchers.field_types.is_some() {
    if !validate_required_fields(rule, event) {
        return false;
    }
}
```

**matches_rule_with_debug function:**
```rust
// Check field validation (require_fields / field_types)
if rule.matchers.require_fields.is_some() || rule.matchers.field_types.is_some() {
    let field_valid = validate_required_fields(rule, event);
    matcher_results.field_validation_matched = Some(field_valid);
    if !field_valid {
        overall_match = false;
    }
}
```

Both positioned after prompt_match check, before final return/tuple.

---

*Phase: 05-field-validation*
*Completed: 2026-02-09*

## Self-Check: PASSED

All files and commits verified:
- ✓ rulez/src/models.rs modified (field_validation_matched added)
- ✓ rulez/src/hooks.rs modified (validate_required_fields + integration + tests)
- ✓ 4c8155e commit exists (Task 1)
- ✓ ee022fb commit exists (Task 2)
- ✓ All 12 field validation tests pass
- ✓ All 430 tests pass (no regressions)

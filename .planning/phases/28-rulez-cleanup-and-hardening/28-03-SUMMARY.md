---
phase: 28-rulez-cleanup-and-hardening
plan: "03"
subsystem: tooling
tags: [evalexpr, config-cache, mtime, tool_input, enabled_when]

# Dependency graph
requires:
  - phase: 28-01
    provides: "pub(crate) get_or_compile_regex, fail-closed regex evaluation"
provides:
  - "tool_input_ prefixed variables in build_eval_context() for enabled_when expressions"
  - "mtime-based config cache in Config::from_file() to avoid redundant disk reads"
affects: [28-04, 28-05, hooks.rs, config.rs]

# Tech tracking
tech-stack:
  added: []
  patterns: ["mtime-based single-slot cache with LazyLock+Mutex", "tool_input field injection with type-safe evalexpr Value mapping"]

key-files:
  created: []
  modified:
    - "rulez/src/hooks.rs"
    - "rulez/src/config.rs"

key-decisions:
  - "Numbers from tool_input are stored as Float (f64) in evalexpr -- users must compare with 30.0 not 30"
  - "Cache placed in from_file() not load() so both project and global config paths benefit"
  - "Complex JSON types (arrays, objects, null) silently skipped in tool_input injection"

patterns-established:
  - "tool_input_ prefix convention for exposing JSON fields in eval context"
  - "Single-slot config cache pattern: CachedConfig struct + CONFIG_CACHE static"

# Metrics
duration: 5min
completed: 2026-03-05
---

# Phase 28 Plan 03: tool_input eval context injection and mtime config cache

**Expose tool_input JSON fields as tool_input_ prefixed variables in enabled_when expressions; add mtime-based Config cache to skip redundant disk reads**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-05T23:33:33Z
- **Completed:** 2026-03-05T23:38:29Z
- **Tasks:** 3
- **Files modified:** 2

## Accomplishments
- `build_eval_context()` now injects string, bool, and number fields from `event.tool_input` with `tool_input_` prefix
- Users can write `enabled_when: "tool_input_command =~ \"git push\""` to match on tool input fields
- `Config::from_file()` caches parsed config and checks file mtime before re-reading from disk
- 5 new unit tests covering string, bool, number, None, and complex-type-skipping cases
- Full CI pipeline passes: fmt, clippy, test, llvm-cov

## Task Commits

Each task was committed atomically:

1. **Task 1: Add tool_input fields to build_eval_context()** - `4a873a9` (feat)
2. **Task 2: Add mtime-based config cache to Config::from_file()** - `ebd1b7e` (feat)
3. **Task 3: Add tests and run full CI pipeline** - `27a3d91` (test)

## Files Created/Modified
- `rulez/src/hooks.rs` - Added tool_input field injection in build_eval_context(); 5 new unit tests
- `rulez/src/config.rs` - Added CachedConfig struct, CONFIG_CACHE static, mtime check in from_file()

## Decisions Made
- Numbers from tool_input JSON are stored as evalexpr Float (f64) -- comparison expressions must use `30.0` not `30`
- Cache is in `from_file()` (not `load()`) so both project-level and global config paths benefit from caching
- Complex JSON types (arrays, objects, null) are silently skipped during tool_input injection -- only string, bool, number are supported by evalexpr

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed number comparison in test assertion**
- **Found during:** Task 3 (unit tests)
- **Issue:** evalexpr Float(30.0) does not equal integer literal 30 in expressions
- **Fix:** Changed test expression from `tool_input_timeout == 30` to `tool_input_timeout == 30.0`
- **Files modified:** rulez/src/hooks.rs
- **Verification:** Test passes after fix
- **Committed in:** 27a3d91 (Task 3 commit)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Minor test fix. No scope creep.

## Issues Encountered
None beyond the number comparison deviation above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- tool_input fields available for all enabled_when expressions across the engine
- Config caching reduces hook latency for repeated invocations
- Ready for Phase 28 Plan 04 (next cleanup/hardening item)

---
*Phase: 28-rulez-cleanup-and-hardening*
*Completed: 2026-03-05*

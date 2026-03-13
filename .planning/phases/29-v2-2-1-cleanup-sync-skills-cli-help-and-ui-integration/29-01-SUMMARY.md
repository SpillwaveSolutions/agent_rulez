---
phase: 29-v2-2-1-cleanup-sync-skills-cli-help-and-ui-integration
plan: 01
subsystem: skills
tags: [release-workflow, naming, cleanup, skill-migration]

# Dependency graph
requires: []
provides:
  - "Renamed release-cch skill to release-rulez in .claude/skills/"
  - "Renamed release-cch skill to release-rulez in .opencode/skill/"
  - "Fixed preflight-check.sh to use workspace-level cargo commands"
  - "Updated all asset names from cch-* to rulez-*"
affects: [release-workflow, skill-loading]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Workspace-level cargo commands instead of subdirectory cd"

key-files:
  created:
    - ".claude/skills/release-rulez/SKILL.md"
    - ".claude/skills/release-rulez/scripts/preflight-check.sh"
    - ".opencode/skill/release-rulez/SKILL.md"
    - ".opencode/skill/release-rulez/scripts/preflight-check.sh"
  modified:
    - ".claude/settings.local.json"
    - ".claude/commands/cch-release.md"
    - ".opencode/command/cch-release.md"
    - "AGENTS.md"

key-decisions:
  - "Used workspace-level cargo commands (--workspace, --all) instead of cd into cch_cli subdirectory"
  - "Updated AGENTS.md skill reference as deviation Rule 3 (blocking remaining release-cch reference)"

patterns-established:
  - "Skill directories must match binary name (release-rulez, not release-cch)"

requirements-completed: [CLEANUP-01, CLEANUP-02, CLEANUP-05]

# Metrics
duration: 13min
completed: 2026-03-12
---

# Phase 29 Plan 01: Rename Release Skill Summary

**Renamed release-cch skill to release-rulez across .claude/skills/ and .opencode/skill/ with 149+ cch references replaced and preflight scripts fixed for workspace-level cargo commands**

## Performance

- **Duration:** 13 min
- **Started:** 2026-03-12T23:51:32Z
- **Completed:** 2026-03-13T00:04:51Z
- **Tasks:** 2
- **Files modified:** 25

## Accomplishments
- Renamed both .claude/skills/release-cch/ and .opencode/skill/release-cch/ to release-rulez
- Replaced all cch/CCH references with rulez/RuleZ across 21 files (skills, scripts, commands, AGENTS.md, settings)
- Fixed preflight-check.sh in both copies to use workspace-level cargo commands instead of cd into non-existent cch_cli directory
- Updated all release asset names from cch-* to rulez-* prefix across docs, scripts, and commands
- Zero remaining release-cch references in active code (only in .planning/ historical docs)

## Task Commits

Each task was committed atomically:

1. **Task 1: Rename release-cch to release-rulez in .claude/skills/** - `401811a` (feat)
2. **Task 2: Mirror all changes to .opencode/skill/release-rulez/** - `3aaefec` (feat)

## Files Created/Modified
- `.claude/skills/release-rulez/` - All 11 files (renamed from release-cch, all cch refs replaced)
- `.opencode/skill/release-rulez/` - All 10 files (renamed from release-cch, all cch refs replaced)
- `.claude/settings.local.json` - Updated 4 permission paths from release-cch to release-rulez
- `.claude/commands/cch-release.md` - Updated skill paths and asset names
- `.opencode/command/cch-release.md` - Updated skill paths and asset names
- `AGENTS.md` - Updated skill entry name, description, and location

## Decisions Made
- Used workspace-level cargo commands (`cargo fmt --all`, `cargo clippy --workspace`, `cargo test --workspace`) instead of `cd "$REPO_ROOT/cch_cli"` since the repo uses workspace-level builds
- Replaced "CCH" with "RuleZ" (product name) and "cch" with "rulez" (binary/asset name) based on context

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed AGENTS.md release-cch reference**
- **Found during:** Task 2 (repo-wide reference scan)
- **Issue:** AGENTS.md still referenced release-cch in skill name, description, and location
- **Fix:** Updated skill entry to release-rulez with RuleZ description and correct path
- **Files modified:** AGENTS.md
- **Verification:** grep confirms zero remaining release-cch references in active code
- **Committed in:** 3aaefec (Task 2 commit)

**2. [Rule 3 - Blocking] Fixed cch-release.md command files**
- **Found during:** Task 1 and Task 2 (repo-wide reference scan)
- **Issue:** Both .claude/commands/cch-release.md and .opencode/command/cch-release.md referenced release-cch paths and had cch-* asset names
- **Fix:** Updated all skill path references and asset names in both command files
- **Files modified:** .claude/commands/cch-release.md, .opencode/command/cch-release.md
- **Verification:** grep confirms zero cch references in command files
- **Committed in:** 401811a (Task 1), 3aaefec (Task 2)

---

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Both auto-fixes necessary to eliminate all release-cch references. No scope creep.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Release skill fully renamed and functional
- All script paths updated, ready for next release cycle
- No blockers for subsequent plans

---
*Phase: 29-v2-2-1-cleanup-sync-skills-cli-help-and-ui-integration*
*Completed: 2026-03-12*

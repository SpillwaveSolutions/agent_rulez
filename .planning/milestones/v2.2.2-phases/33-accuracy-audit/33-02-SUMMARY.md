---
phase: 33-accuracy-audit
plan: 02
subsystem: docs
tags: [documentation, audit, yaml-schema, cli-flags, cross-references]

requires:
  - phase: 33-01
    provides: "Validated cli-commands.md, hooks-yaml-schema.md, quick-reference.md"
provides:
  - "13 audited docs with last_validated frontmatter"
  - "All YAML examples use correct schema (rules/matchers/actions)"
  - "Template YAML validated with rulez validate"
affects: [mastering-hooks, docs]

tech-stack:
  added: []
  patterns: ["YAML frontmatter with last_validated for doc audit trail"]

key-files:
  created: []
  modified:
    - docs/guides/claude-code-guide.md
    - docs/guides/gemini-cli-guide.md
    - docs/guides/opencode-guide.md
    - docs/features/external-logging.md
    - docs/features/lint.md
    - docs/features/test.md
    - docs/before-agent-guide.md
    - mastering-hooks/SKILL.md
    - mastering-hooks/references/rule-patterns.md
    - mastering-hooks/references/troubleshooting-guide.md
    - mastering-hooks/references/agent-inline-hooks.md
    - mastering-hooks/references/platform-adapters.md
    - mastering-hooks/assets/hooks-template.yaml

key-decisions:
  - "Used version 1.0 format in template (code requires X.Y format despite schema doc saying 1 or 1.0)"
  - "Moved governance.reason out of actions into governance section (reason is not an actions field)"
  - "Replaced tool_execute/tool_result event types in test.md with PreToolUse/PostToolUse"

patterns-established:
  - "enabled_when uses underscores (env_CI) not dots (env.CI) for evalexpr compatibility"
  - "YAML config uses rules/matchers/actions, not hooks/match/action"

requirements-completed: [AUDIT-01, AUDIT-02]

duration: 8min
completed: 2026-03-16
---

# Phase 33 Plan 02: Guides, Features, Skill Docs, and Template Audit Summary

**Audited 13 docs and 1 template for stale YAML field names, wrong CLI flags, and cross-reference consistency**

## Performance

- **Duration:** 8 min
- **Started:** 2026-03-16T21:45:19Z
- **Completed:** 2026-03-16T21:53:00Z
- **Tasks:** 2
- **Files modified:** 13

## Accomplishments

- Fixed YAML examples in 10 docs from old schema (hooks/event/match/action) to correct schema (rules/matchers/actions)
- Fixed 15+ incorrect CLI flags across all docs (--input, --tail, --status, --rule, --event, --project, --user, --version --json)
- Fixed enabled_when syntax from dot notation (env.CI) to evalexpr underscore notation (env_CI)
- Fixed event type references from non-existent values (tool_execute, tool_result) to correct values (PreToolUse, PostToolUse)
- Template YAML validates with rulez validate
- All cross-reference links resolve to existing files
- No stale cch references in any audit-scope files

## Task Commits

1. **Task 1: Audit guides and feature docs** - `58e484f` (fix)
2. **Task 2: Audit skill docs, templates, and cross-reference links** - `59f96fb` (fix)

## Files Created/Modified

- `docs/guides/claude-code-guide.md` - Added frontmatter, fixed YAML schema and matcher descriptions
- `docs/guides/gemini-cli-guide.md` - Added frontmatter, fixed YAML schema (command_pattern -> command_match)
- `docs/guides/opencode-guide.md` - Added frontmatter, fixed YAML schema
- `docs/features/external-logging.md` - Added frontmatter, fixed debug --input flag to --path
- `docs/features/lint.md` - Added frontmatter, fixed operations: [tool_execute] -> [PreToolUse]
- `docs/features/test.md` - Added frontmatter, fixed event_type values and troubleshooting
- `docs/before-agent-guide.md` - Added frontmatter, fixed YAML schema, removed non-existent log flags
- `mastering-hooks/SKILL.md` - Fixed version, CLI flags, YAML schema, explain config -> explain rules
- `mastering-hooks/references/rule-patterns.md` - Fixed enabled_when syntax and field placement
- `mastering-hooks/references/troubleshooting-guide.md` - Fixed CLI flags, YAML schema, enabled_when examples
- `mastering-hooks/references/agent-inline-hooks.md` - Added frontmatter
- `mastering-hooks/references/platform-adapters.md` - Added frontmatter
- `mastering-hooks/assets/hooks-template.yaml` - Rewritten with correct schema, validates with rulez validate

## Decisions Made

- Used version "1.0" format in template because code regex requires X.Y format (not just "1")
- Moved reason strings from actions to governance section since Actions struct has no reason field
- Replaced non-existent event types (tool_execute, tool_result) with actual types (PreToolUse, PostToolUse)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed version format in template YAML**
- **Found during:** Task 2 (template validation)
- **Issue:** Template used `version: "1"` but code regex requires `\d+\.\d+` format
- **Fix:** Changed to `version: "1.0"`
- **Verification:** `rulez validate --config /tmp/hooks-template-test.yaml` passes
- **Committed in:** 59f96fb (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Minor fix necessary for template correctness. No scope creep.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- All documentation in docs/ and mastering-hooks/ is now audited and consistent with source code
- No remaining stale field names, flags, or broken cross-references in audit scope

---
*Phase: 33-accuracy-audit*
*Completed: 2026-03-16*

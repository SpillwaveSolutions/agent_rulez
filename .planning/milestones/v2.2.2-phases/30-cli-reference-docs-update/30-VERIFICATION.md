---
phase: 30-cli-reference-docs-update
verified: 2026-03-14T22:00:00Z
status: gaps_found
score: 2/3 must-haves verified
re_verification: false
gaps:
  - truth: "A user reading quick-reference.md finds all current events, actions, matchers, and CLI commands in one place"
    status: partial
    reason: "quick-reference.md contains 3 inaccurate CLI command examples that contradict both the binary --help output and the cli-commands.md doc"
    artifacts:
      - path: "mastering-hooks/references/quick-reference.md"
        issue: "Line 96: `rulez install --project` uses nonexistent --project flag (actual: default is project, use --global/-g for global). Line 101: `rulez logs --tail 20` uses nonexistent --tail flag (actual: --limit/-l). Line 103: `rulez explain config` references nonexistent subcommand (actual subcommands: rule, rules, event)."
    missing:
      - "Fix line 96: change `rulez install --project` to `rulez install` (project is default) or `rulez install --global` to show the flag"
      - "Fix line 101: change `rulez logs --tail 20` to `rulez logs --limit 20` or `rulez logs -l 20`"
      - "Fix line 103: change `rulez explain config` to `rulez explain rules` (the actual subcommand for listing all rules)"
---

# Phase 30: CLI Reference Docs Update Verification Report

**Phase Goal:** All reference documentation accurately reflects the current state of RuleZ v2.2.1
**Verified:** 2026-03-14T22:00:00Z
**Status:** gaps_found
**Re-verification:** No -- initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | A user reading cli-commands.md can find accurate documentation for `rulez test`, `rulez lint`, and `rulez upgrade` with correct flags and examples | VERIFIED | All three commands documented with flags matching `--help` output exactly. test: `<TEST_FILE>` arg + `-v/--verbose`. lint: `-c/--config` + `-v/--verbose`. upgrade: `--check`. |
| 2 | A user reading hooks-yaml-schema.md sees parallel eval, config caching, globset matching, and external logging fields documented | VERIFIED | Engine Behavior section (line 337+) documents all 6 features: PARALLEL_THRESHOLD=10, CachedConfig mtime invalidation, globset with build_glob_set(), regex fail-closed, tool_input_ prefix with Float caveat, external logging (OTLP/Datadog/Splunk). All cross-checked against source in hooks.rs and config.rs. |
| 3 | A user reading quick-reference.md finds all current events, actions, matchers, and CLI commands in one place | PARTIAL | 22 CLI commands listed (all present), 16 event types listed, 7 matcher types, 6 action types, exit codes, debug aliases. However, 3 command examples use incorrect/nonexistent flags: `--project` (line 96), `--tail` (line 101), `explain config` (line 103). |

**Score:** 2/3 truths fully verified, 1 partial

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `mastering-hooks/references/cli-commands.md` | Complete CLI reference for all rulez commands | VERIFIED | 770 lines, all 14 top-level commands documented with accurate flags verified against --help, multi-CLI subcommands (gemini/copilot/opencode install/hook/doctor) included |
| `mastering-hooks/references/hooks-yaml-schema.md` | Complete hooks.yaml schema reference with v2.0-v2.2.1 features | VERIFIED | 456 lines, Engine Behavior section at line 337 covers all 6 features. Source cross-checks confirm PARALLEL_THRESHOLD, CachedConfig, build_glob_set, get_or_compile_regex, tool_input_ prefix, LoggingConfig all exist in hooks.rs/config.rs |
| `mastering-hooks/references/quick-reference.md` | At-a-glance reference for all RuleZ capabilities | PARTIAL | 150 lines, comprehensive coverage of events/matchers/actions/commands but 3 inaccurate command examples |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|----|--------|---------|
| cli-commands.md | rulez --help output | manual cross-check | WIRED | All 14 commands match. test/lint/upgrade flags verified character-by-character against --help. logs flags (--limit, --mode, --decision) match. |
| hooks-yaml-schema.md | rulez/src/hooks.rs | schema field documentation | WIRED | PARALLEL_THRESHOLD=10 confirmed at hooks.rs:644, CachedConfig at config.rs:16, build_glob_set at hooks.rs:797, build_eval_context at hooks.rs:558, get_or_compile_regex at hooks.rs:51 |
| quick-reference.md | rulez --help | command listing | PARTIAL | All commands listed but 3 examples use wrong flags/subcommands that don't exist in binary |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| CLIDOC-01 | 30-01 | cli-commands.md documents all CLI commands including test, lint, upgrade with accurate flags and examples | SATISFIED | All 14 commands documented, flags match --help exactly |
| CLIDOC-02 | 30-02 | hooks-yaml-schema.md reflects parallel eval, config caching, globset matching, and external logging fields | SATISFIED | Engine Behavior section documents all 6 features with source-verified accuracy |
| CLIDOC-03 | 30-02 | quick-reference.md updated with latest events, actions, matchers, and CLI commands | PARTIAL | All items present but 3 command examples have incorrect flags |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| quick-reference.md | 96 | Incorrect flag `--project` (does not exist) | WARNING | User confusion -- flag will error |
| quick-reference.md | 101 | Incorrect flag `--tail` (does not exist, actual: `--limit`) | WARNING | User confusion -- flag will error |
| quick-reference.md | 103 | Incorrect subcommand `explain config` (does not exist, actual: `explain rules`) | WARNING | User confusion -- command will error |

### Human Verification Required

None -- all checks are automated against binary --help output.

### Gaps Summary

The phase is nearly complete. cli-commands.md (CLIDOC-01) and hooks-yaml-schema.md (CLIDOC-02) are fully accurate and verified against source code and binary output.

quick-reference.md (CLIDOC-03) has 3 inaccurate command examples on lines 96, 101, and 103 that use flags/subcommands that do not exist in the current binary. These are likely leftover from the pre-update state that were not caught during plan 02 execution. The fixes are trivial single-line edits.

---

_Verified: 2026-03-14T22:00:00Z_
_Verifier: Claude (gsd-verifier)_

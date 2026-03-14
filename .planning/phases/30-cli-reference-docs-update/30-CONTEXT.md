# Phase 30: CLI Reference Docs Update - Context

**Gathered:** 2026-03-14
**Status:** Ready for planning
**Source:** Conversation context from milestone initialization

<domain>
## Phase Boundary

Update the three core mastering-hooks reference docs (`cli-commands.md`, `hooks-yaml-schema.md`, `quick-reference.md`) to accurately reflect all changes from v2.0 through v2.2.1. This is a docs-only phase — no code changes.

</domain>

<decisions>
## Implementation Decisions

### Target Files
- `mastering-hooks/references/cli-commands.md` — must document all 19+ CLI commands
- `mastering-hooks/references/hooks-yaml-schema.md` — must reflect engine changes
- `mastering-hooks/references/quick-reference.md` — must be a current at-a-glance reference

### CLI Commands to Document or Update
- `rulez test <file.yaml>` — batch test scenarios, pass/fail summary, exit code 1 on failure (added v2.2)
- `rulez lint` — duplicate names, overlapping rules, dead rules, missing descriptions (added v2.2)
- `rulez upgrade` — self_update crate with rustls backend (added v2.0)
- All existing commands should be verified against `rulez --help` and `rulez <cmd> --help`

### Engine Features to Document in Schema
- Parallel rule evaluation (PARALLEL_THRESHOLD=10, join_all) — v2.0
- Config caching with mtime-based invalidation — v2.0
- Globset matching replacing naive contains() — v2.0
- External logging: OTLP, Datadog, Splunk backends via curl — v2.2
- tool_input fields exposed in enabled_when eval context — v2.0
- Regex fail-closed semantics — v2.0

### Source of Truth
- Cross-check all docs against `rulez --help`, `rulez <cmd> --help`, and Rust source code
- The actual binary output is the canonical reference, not existing docs
- Check `rulez/src/cli/` for all subcommand implementations

### Claude's Discretion
- Organization and formatting within each doc
- Whether to add new sections or restructure existing ones
- Level of detail for examples

</decisions>

<specifics>
## Specific Ideas

- Run `rulez --help` and each subcommand's `--help` to get exact current flags
- Check `rulez/src/cli/` directory for all subcommand source files
- Phase 29 (v2.2.1) already documented 9 missing CLI commands in mastering-hooks — verify those additions are complete
- v2.0 Phase 28 fixed 7 field name mismatches in skill docs — verify those are resolved

</specifics>

<deferred>
## Deferred Ideas

- Per-CLI usage guides (Phase 31)
- Feature-specific standalone docs for logging/lint/test (Phase 32)
- Full accuracy audit across all docs (Phase 33)

</deferred>

---

*Phase: 30-cli-reference-docs-update*
*Context gathered: 2026-03-14 from milestone conversation*

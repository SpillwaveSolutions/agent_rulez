# Phase 33: Accuracy Audit - Context

**Gathered:** 2026-03-16
**Status:** Ready for planning

<domain>
## Phase Boundary

Verify every documentation file against the actual CLI binary and Rust source code. Fix stale field names, wrong flags, broken examples, outdated file paths, and inconsistencies across all docs. Add YAML frontmatter with `last_modified` and `last_validated` dates to every audited file. No new features or docs — audit and fix only.

</domain>

<decisions>
## Implementation Decisions

### Audit scope
- Full sweep: all .md files in docs/, docs/guides/, docs/features/, mastering-hooks/references/
- Include mastering-hooks/SKILL.md (user-facing, Claude Code reads it)
- Include mastering-hooks/assets/ template files (.yaml.template)
- Include docs/config-schema.md and docs/event-schema.md (API-level docs)
- Approximately 15-20 files total

### Verification method
- Build the binary with `cargo build` and run `rulez --help` and `rulez <cmd> --help` for every subcommand
- First validate that `--help` output itself is complete and accurate against source code (all commands registered, descriptions current) — fix --help before using it to audit docs
- Extract YAML config examples from docs and run `rulez validate` on them to confirm they parse
- Check that cross-reference links between docs actually resolve (link targets exist at specified paths)

### Fix strategy
- Fix issues in-place as they're found, one commit per doc file
- When a doc section needs significant rewriting (not just field name swaps), rewrite it during the audit — this is the last phase, no point deferring
- Add YAML frontmatter header to every audited doc file:
  ```yaml
  ---
  last_modified: 2026-03-16
  last_validated: 2026-03-16
  ---
  ```

### Staleness signals to check
- Old binary name `cch` (renamed to `rulez` in v1.5) — any remaining references are definitely stale
- CLI flags: compare every documented flag against actual `rulez <cmd> --help` output
- YAML field names: check documented hooks.yaml fields against models.rs and config.rs structs
- File paths and install locations: verify documented paths (~/.claude/hooks.yaml, ~/.claude/logs/rulez.log, etc.)
- Version references: verify "Added in vX.Y" annotations are accurate
- Cross-doc consistency: same feature described the same way everywhere (e.g., flag description in cli-commands.md matches guide description)

### Claude's Discretion
- Order of files to audit (can prioritize highest-traffic docs first)
- Whether to group related files or audit strictly one-by-one
- How to handle docs that are mostly accurate with minor issues vs docs that need major rework
- Exact frontmatter format beyond the required fields

</decisions>

<specifics>
## Specific Ideas

- Phase 28 (v2.0) fixed 7 field name mismatches in skill docs — verify those fixes are still correct
- Phase 29 (v2.2.1) added 9 missing CLI commands to mastering-hooks — verify completeness
- Phase 30 updated 3 core reference docs — verify they haven't drifted
- The binary `--help` must be validated first before using it as source of truth for doc auditing
- Commits should be per-file for clean git history and easy review/revert

</specifics>

<code_context>
## Existing Code Insights

### Reusable Assets
- `rulez --help` and `rulez <cmd> --help` output as canonical reference for CLI docs
- `rulez/src/cli/` directory contains all subcommand implementations with clap definitions
- `rulez/src/models.rs` defines all YAML config structs (field names, types, defaults)
- `rulez/src/config.rs` defines config loading paths and defaults
- `rulez validate` command can verify YAML examples parse correctly

### Established Patterns
- Phase 30 CONTEXT.md established "source of truth is binary output, not existing docs"
- Per-file commits used in Phases 30-32 for clean git history

### Integration Points
- All docs reference each other via relative markdown links — link validation catches broken cross-references
- SKILL.md references field names that must match models.rs
- Template files in assets/ contain YAML that must match current schema

</code_context>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 33-accuracy-audit*
*Context gathered: 2026-03-16*

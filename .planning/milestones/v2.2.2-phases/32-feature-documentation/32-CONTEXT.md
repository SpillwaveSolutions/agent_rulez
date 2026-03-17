# Phase 32: Feature Documentation - Context

**Gathered:** 2026-03-16
**Status:** Ready for planning

<domain>
## Phase Boundary

Standalone documentation for three new features added in v2.0-v2.2.1: external logging backends (OTLP, Datadog, Splunk), `rulez lint`, and `rulez test`. Each feature gets its own doc with working examples users can follow end-to-end. No code changes.

</domain>

<decisions>
## Implementation Decisions

### Doc placement & structure
- Three standalone docs in `docs/features/`: `external-logging.md`, `lint.md`, `test.md`
- Each doc follows a consistent template: Overview, Prerequisites, Quick Start, Configuration, Examples, Troubleshooting, Further Reading
- Cross-reference heavily to existing reference docs (cli-commands.md, hooks-yaml-schema.md, event-schema.md) rather than duplicating content

### Example depth
- Full working examples that users can copy-paste and run immediately
- Include expected terminal output after each command example
- Include a complete, runnable test YAML file with multiple scenarios (pass + fail cases)
- Use realistic rule names and scenarios (e.g., 'deny-rm-rf', 'audit-file-writes')
- Lint doc shows before/after pairs for each rule (bad config that triggers, then fixed version)
- Logging doc includes verification steps for each backend (trigger a rule, check backend received the log)
- Include common mistakes/gotchas in the Troubleshooting section

### Logging doc scope
- Equal coverage for all three backends (OTLP, Datadog, Splunk) — each gets its own section
- Show sample JSON payload that each backend receives, linking to event-schema.md for full spec
- Brief section on combining multiple backends simultaneously (e.g., OTLP + local file logging)
- Include a "When to use which backend" comparison table (protocol, auth method, best-for)

### Lint/Test doc tone
- Tutorial-first: start with a quick walkthrough ("Lint your first config", "Write your first test"), then follow with full reference
- Lint doc uses structured "rule cards" for each rule: Rule Name, What it Detects, Why it Matters, Bad Example, Fixed Example (ESLint-style)
- Test doc documents the test YAML schema (all available fields in a test scenario with descriptions)
- Test doc includes a brief CI integration section (minimal GitHub Actions step for `rulez test`)

### Claude's Discretion
- Exact section ordering within the standard template
- Typography and formatting choices
- How much introductory text vs jumping straight to examples
- Troubleshooting item selection (pick the most likely gotchas)

</decisions>

<specifics>
## Specific Ideas

- Phase 31 guides used full working examples and tutorial-style tone — maintain consistency
- Lint rule cards inspired by ESLint's rule documentation format
- Logging comparison table should help users pick the right backend quickly
- CI section for test doc should be brief (5-10 lines of GitHub Actions YAML)
- Cross-reference links should point to specific sections in reference docs, not just the file

</specifics>

<code_context>
## Existing Code Insights

### Reusable Assets
- `docs/config-schema.md` (494 lines): Already documents logging config fields — link to it, don't duplicate
- `docs/event-schema.md` (236 lines): Documents event payload structure — reference for logging payload section
- `mastering-hooks/references/cli-commands.md`: Documents `rulez test` and `rulez lint` flags — link to it
- `mastering-hooks/references/hooks-yaml-schema.md`: Documents external logging YAML fields — link to it

### Established Patterns
- Phase 31 guides (claude-code-guide.md, gemini-cli-guide.md, opencode-guide.md) established the tutorial-first, full-examples style
- Reference docs in mastering-hooks/references/ use a more dense, reference-style format

### Integration Points
- `docs/features/` directory needs to be created (new)
- Feature docs link to: cli-commands.md (flags), hooks-yaml-schema.md (config), event-schema.md (payload), config-schema.md (settings)

</code_context>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 32-feature-documentation*
*Context gathered: 2026-03-16*

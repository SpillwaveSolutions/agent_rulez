# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into future planning.*

## Milestone: v2.2.2 — Documentation Audit & Multi-CLI Guides

**Shipped:** 2026-03-17
**Phases:** 4 | **Plans:** 8 | **Sessions:** ~4

### What Was Built
- CLI reference docs updated for all 14 rulez commands with accurate flags from --help output
- Per-CLI usage guides for Claude Code, Gemini CLI, and OpenCode (3 new guides)
- Feature documentation for external logging (OTLP/Datadog/Splunk), lint (9 rules), and test
- Accuracy audit across 13 docs with `last_validated` frontmatter for audit trail
- Quick-reference.md expanded to 22 CLI commands, all action types, and exit codes

### What Worked
- **Audit-last pattern**: Writing all docs first (phases 30-32), then auditing everything (phase 33) caught cross-doc inconsistencies efficiently
- **Binary as source of truth**: Cross-checking docs against `rulez --help` output caught many stale references that would have been missed by reading source code alone
- **Consistent guide structure**: Establishing a template (overview, prereqs, quick-start, verify, troubleshoot) in phase 31 made all three CLI guides uniform and easy to follow
- **3-day execution**: Docs-only milestone completed quickly without the overhead of Rust compilation or test cycles

### What Was Inefficient
- Phase 31/32 plan checkboxes in ROADMAP.md weren't marked as `[x]` during execution — had to notice and fix during milestone completion
- Some feature docs referenced phase numbers that don't exist in this milestone's numbering (e.g., "phase 33-external-logging" in SUMMARY provides field)

### Patterns Established
- `last_validated` YAML frontmatter in docs for audit trail tracking
- ESLint-style rule cards for documenting lint rules (bad/fixed YAML examples)
- Tutorial-first feature documentation in `docs/features/` directory
- Per-CLI usage guide structure with platform-specific sections

### Key Lessons
1. **Docs-only milestones ship fast** — 4 phases in 3 days because there's no compile/test cycle. Good cadence to alternate code and docs milestones.
2. **Audit as a separate phase works** — Catching stale `--input` flag in troubleshooting, wrong field names in schemas, and inconsistent cross-references justified the dedicated audit phase.
3. **Documentation entropy is real** — After 12 milestones of code changes, docs had accumulated significant drift from actual behavior. Regular doc audits should be part of the process.

### Cost Observations
- Model mix: ~80% opus, ~20% sonnet
- Sessions: ~4
- Notable: Docs-only work is highly efficient — minimal tool overhead, no build waits

---

## Cross-Milestone Trends

### Process Evolution

| Milestone | Sessions | Phases | Key Change |
|-----------|----------|--------|------------|
| v2.2.2 | ~4 | 4 | Docs-only milestone pattern, audit-last approach |

### Top Lessons (Verified Across Milestones)

1. Dedicated audit phases catch drift that incremental reviews miss
2. Docs-only milestones are fast and valuable — prevents documentation debt from accumulating

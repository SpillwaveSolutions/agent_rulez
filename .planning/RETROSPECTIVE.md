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

## Milestone: v2.3.0 — Multi-Runtime Skill Portability

**Shipped:** 2026-03-18
**Phases:** 5 (34-38) | **Plans:** 0 (direct implementation) | **Sessions:** ~2

### What Was Built
- `Runtime` enum with 5 variants (Claude, OpenCode, Gemini, Codex, Custom) — each resolves its own install paths and conventions
- `SkillInventory` discovery: scans `.claude/skills/`, `.claude/commands/`, and extra sources (mastering-hooks at repo root)
- 6-transform pipeline across 6 dedicated files: tool name rewrite, path refs, command filename flattening, frontmatter conversion, MCP exclusion, color handling
- `rulez skills install --runtime <rt>` / `--dry-run` / `rulez skills clean` with clean-install writer
- GEMINI.md marker-based update and AGENTS.md auto-generation for Codex
- `rulez skills status`, `rulez skills diff`, `rulez skills sync` — complete DX subcommand family
- 2,099 new Rust LOC across 13 source files; PR #116 — 9/9 CI checks passed

### What Worked
- **Module-first design**: Creating `rulez/src/skills/` as a dedicated module with clear internal boundaries (discovery → transform → writer → config_gen) made the dependency chain clean
- **`Custom` runtime variant**: Adding a catch-all `Runtime::Custom(PathBuf)` from the start avoided needing a separate "generic" code path — the `--dir` flag routes through the same pipeline
- **Clean-install writer pattern**: Removing the target directory before writing prevents orphaned files from previous installs — zero special-case cleanup logic needed
- **2-day ship velocity**: Implementing 5 phases without GSD phase artifacts (directly in release branch) moved faster than the full execute-phase cycle for a well-specified feature

### What Was Inefficient
- **Missing GSD phase artifacts**: No PLAN.md, SUMMARY.md, VERIFICATION.md, or VALIDATION.md for any of the 5 phases — the milestone audit had to rely on integration checker code analysis instead of structured evidence. This made the audit slower and less reliable.
- **CONFIG-04 and DX-04 slipped**: Two requirements were marked `[x]` in REQUIREMENTS.md but not fully implemented (no context-aware mastering-hooks transform, no color output). Should have caught this before tagging.
- **Local main divergence**: Local branch had 18 diverging commits vs. 1 on origin/main after the squash merge — had to use `git checkout origin/main -- .planning/` to sync. Avoid letting local main diverge from remote after squash merges.

### Patterns Established
- `rulez skills` as a separate subcommand family (orthogonal to `rulez install` which handles hook registration)
- `<!-- RULEZ_SKILLS_START -->` / `<!-- RULEZ_SKILLS_END -->` marker pattern for idempotent config file injection
- `TransformPipeline::for_runtime(runtime)` as the factory for per-runtime transform chains

### Key Lessons
1. **Mark requirements complete only when implemented** — checked `[x]` in REQUIREMENTS.md before confirming DX-04 and CONFIG-04 were in the code. The audit caught it, but it added overhead. Rule: requirements table reflects implementation state, not intent.
2. **Direct-to-release-branch implementation skips GSD workflow** — fast to ship but loses verification artifacts. For well-specified features with CI coverage this is acceptable; for complex features use `gsd:execute-phase` to get the evidence trail.
3. **Squash-merge leaves local main diverged** — after `release/v2.3.0` merged to `origin/main`, local `main` had 18 unrelated commits. Clean up with `git checkout origin/main -- <dirs>` when `git reset --hard` is blocked by RuleZ itself.

### Cost Observations
- Model mix: ~70% opus, ~30% sonnet
- Sessions: ~2
- Notable: Feature implementation + audit + milestone completion in a single extended session

---

## Cross-Milestone Trends

### Process Evolution

| Milestone | Sessions | Phases | Key Change |
|-----------|----------|--------|------------|
| v2.2.2 | ~4 | 4 | Docs-only milestone pattern, audit-last approach |
| v2.3.0 | ~2 | 5 | Direct-to-release-branch implementation, integration checker as substitute audit |

### Top Lessons (Verified Across Milestones)

1. Dedicated audit phases catch drift that incremental reviews miss
2. Docs-only milestones are fast and valuable — prevents documentation debt from accumulating
3. Requirements table must reflect implementation state, not intent — check [x] only when code exists
4. Direct branch implementation trades GSD evidence trail for speed — acceptable when CI is comprehensive

# Requirements: RuleZ v2.2.2

**Defined:** 2026-03-13
**Core Value:** LLMs do not enforce policy. LLMs are subject to policy.

## v2.2.2 Requirements

Requirements for documentation audit and multi-CLI guides release.

### CLI Docs Audit

- [ ] **CLIDOC-01**: `cli-commands.md` documents all CLI commands including `test`, `lint`, `upgrade` with accurate flags and examples
- [ ] **CLIDOC-02**: `hooks-yaml-schema.md` reflects parallel eval, config caching, globset matching, and external logging fields
- [ ] **CLIDOC-03**: `quick-reference.md` updated with latest events, actions, matchers, and CLI commands

### Multi-CLI Usage Guides

- [ ] **GUIDE-01**: Claude Code usage guide covers install, configure, verify, and troubleshoot workflow
- [ ] **GUIDE-02**: Gemini CLI usage guide covers install, dual-fire events, and verify workflow
- [ ] **GUIDE-03**: OpenCode usage guide covers install, plugin setup, and verify workflow

### Feature Documentation

- [ ] **FEAT-01**: External logging backends (OTLP, Datadog, Splunk) documented with configuration examples
- [ ] **FEAT-02**: `rulez lint` rules documented (duplicate names, overlapping rules, dead rules, missing descriptions)
- [ ] **FEAT-03**: `rulez test` batch testing workflow documented with example test files

### Accuracy Audit

- [ ] **AUDIT-01**: All docs cross-checked against `rulez --help` output and source code for correctness
- [ ] **AUDIT-02**: Stale field names, command flags, examples, and file paths fixed across all reference docs

## Future Requirements

None — docs-only milestone.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Codex CLI guide | No hooks support — scenarios skip unconditionally |
| Copilot CLI guide refresh | Existing docs already comprehensive |
| New code features | Docs-only milestone, no engine changes |
| RuleZ UI docs | UI docs already current from v2.2.1 |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| CLIDOC-01 | — | Pending |
| CLIDOC-02 | — | Pending |
| CLIDOC-03 | — | Pending |
| GUIDE-01 | — | Pending |
| GUIDE-02 | — | Pending |
| GUIDE-03 | — | Pending |
| FEAT-01 | — | Pending |
| FEAT-02 | — | Pending |
| FEAT-03 | — | Pending |
| AUDIT-01 | — | Pending |
| AUDIT-02 | — | Pending |

**Coverage:**
- v2.2.2 requirements: 11 total
- Mapped to phases: 0
- Unmapped: 11 (pending roadmap)

---
*Requirements defined: 2026-03-13*
*Last updated: 2026-03-13 after initial definition*

# RuleZ Roadmap

**Current Focus:** v2.2.2 Documentation Audit & Multi-CLI Guides (Phases 30-33)

---

## Milestones

- ✅ **v1.2 P2 Features** — Phases 1-3 (shipped 2026-02-07) — [Archive](milestones/v1.2-ROADMAP.md)
- ✅ **v1.3 Advanced Matching & Validation** — Phases 4-6 (shipped 2026-02-10) — [Archive](milestones/v1.3-ROADMAP.md)
- ✅ **v1.4 Stability & Polish** — Phases 7-10 (shipped 2026-02-10) — [Archive](milestones/v1.4-ROADMAP.md)
- ✅ **v1.6 RuleZ UI** — Phases 11-17 (shipped 2026-02-12) — [Archive](milestones/v1.6-ROADMAP.md)
- ✅ **v1.7 Multi-Platform Hook Support** — Phases 18-21 (shipped 2026-02-13) — [Archive](milestones/v1.7-ROADMAP.md)
- ✅ **v1.8 Tool Name Canonicalization** — Phase 22 (shipped 2026-02-22) — [Archive](milestones/v1.8-ROADMAP.md)
- ✅ **v1.9 Multi-CLI E2E Testing** — Phases 23, 25 (shipped 2026-03-05) — [Archive](milestones/v1.9-ROADMAP.md)
- ✅ **v2.0 RuleZ Cleanup and Hardening** — Phase 28 (shipped 2026-03-05) — [Archive](milestones/v2.0-ROADMAP.md)
- ✅ **v2.1 Multi-CLI E2E Testing (continued)** — Phases 24, 26, 27 (shipped 2026-03-09) — [Archive](milestones/v2.1-ROADMAP.md)
- ✅ **v2.2.0 Subagent Hooks, DX, Performance & Enterprise** — Phases 29-36 (shipped 2026-03-11) — [Archive](milestones/v2.2.0-ROADMAP.md)
- ✅ **v2.2.1 Cleanup, Sync Skills, CLI Help & UI Integration** — Phase 29 (shipped 2026-03-13) — [Archive](milestones/v2.2.1-ROADMAP.md)
- 🚧 **v2.2.2 Documentation Audit & Multi-CLI Guides** — Phases 30-33 (in progress)

---

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

- [x] **Phase 30: CLI Reference Docs Update** - Update cli-commands.md, hooks-yaml-schema.md, and quick-reference.md to reflect v2.0-v2.2.1 changes (completed 2026-03-14)
- [x] **Phase 31: Multi-CLI Usage Guides** - Create per-CLI usage guides for Claude Code, Gemini, and OpenCode (completed 2026-03-14)
- [ ] **Phase 32: Feature Documentation** - Document external logging, rulez lint, and rulez test with configuration examples
- [ ] **Phase 33: Accuracy Audit** - Cross-check all docs against source code and CLI help output, fix stale references

## Phase Details

### Phase 30: CLI Reference Docs Update
**Goal**: All reference documentation accurately reflects the current state of RuleZ v2.2.1
**Depends on**: Nothing (first phase)
**Requirements**: CLIDOC-01, CLIDOC-02, CLIDOC-03
**Success Criteria** (what must be TRUE):
  1. A user reading cli-commands.md can find accurate documentation for `rulez test`, `rulez lint`, and `rulez upgrade` with correct flags and examples
  2. A user reading hooks-yaml-schema.md sees parallel eval, config caching, globset matching, and external logging fields documented
  3. A user reading quick-reference.md finds all current events, actions, matchers, and CLI commands in one place
**Plans:** 2/2 plans complete
Plans:
- [x] 30-01-PLAN.md — Update cli-commands.md with all CLI commands, flags, and examples
- [x] 30-02-PLAN.md — Update hooks-yaml-schema.md and quick-reference.md with engine features and current reference data

### Phase 31: Multi-CLI Usage Guides
**Goal**: Users of Claude Code, Gemini CLI, and OpenCode each have a dedicated guide for installing, configuring, and verifying RuleZ
**Depends on**: Phase 30 (reference docs should be current before writing guides that link to them)
**Requirements**: GUIDE-01, GUIDE-02, GUIDE-03
**Success Criteria** (what must be TRUE):
  1. A Claude Code user can follow the guide end-to-end to install RuleZ, create a hooks.yaml, verify it fires, and troubleshoot common issues
  2. A Gemini CLI user can follow the guide to install RuleZ, understand dual-fire events, and verify hook execution
  3. An OpenCode user can follow the guide to install RuleZ, set up the plugin, and verify hook execution
**Plans:** 2/2 plans complete
Plans:
- [ ] 31-01-PLAN.md — Create Claude Code usage guide (install, configure, verify, troubleshoot)
- [ ] 31-02-PLAN.md — Create Gemini CLI and OpenCode usage guides with platform-specific details

### Phase 32: Feature Documentation
**Goal**: New features from v2.0-v2.2.1 (external logging, lint, test) have standalone documentation with working examples
**Depends on**: Phase 30 (reference docs updated first so feature docs can cross-reference accurately)
**Requirements**: FEAT-01, FEAT-02, FEAT-03
**Success Criteria** (what must be TRUE):
  1. A user can configure OTLP, Datadog, or Splunk logging backends by following the external logging documentation and its configuration examples
  2. A user can understand all `rulez lint` rules (duplicate names, overlapping rules, dead rules, missing descriptions) and interpret lint output
  3. A user can create a test YAML file and run `rulez test` to validate their hooks configuration, following the batch testing documentation
**Plans**: TBD

### Phase 33: Accuracy Audit
**Goal**: Every documentation file is verified against the actual CLI binary and source code -- no stale field names, wrong flags, or broken examples
**Depends on**: Phase 30, Phase 31, Phase 32 (audit runs after all docs are written/updated)
**Requirements**: AUDIT-01, AUDIT-02
**Success Criteria** (what must be TRUE):
  1. Every CLI command documented in reference docs matches `rulez --help` and `rulez <cmd> --help` output exactly (flags, descriptions, defaults)
  2. All field names, file paths, and configuration examples in docs match the current source code (no stale references from pre-v2.0 versions)
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 30 -> 31 -> 32 -> 33

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1-3 | v1.2 | 6/6 | Complete | 2026-02-07 |
| 4-6 | v1.3 | 10/10 | Complete | 2026-02-10 |
| 7-10 | v1.4 | 9/9 | Complete | 2026-02-10 |
| 11-17 | v1.6 | 19/19 | Complete | 2026-02-12 |
| 18-21 | v1.7 | 15/15 | Complete | 2026-02-13 |
| 22 | v1.8 | 2/2 | Complete | 2026-02-22 |
| 23, 25 | v1.9 | 5/5 | Complete | 2026-03-05 |
| 28 | v2.0 | 8/8 | Complete | 2026-03-05 |
| 24, 26, 27 | v2.1 | 4/4 | Complete | 2026-03-09 |
| 29 | v2.2.1 | 2/2 | Complete | 2026-03-13 |
| 30. CLI Reference Docs Update | v2.2.2 | 2/2 | Complete | 2026-03-14 |
| 31. Multi-CLI Usage Guides | 2/2 | Complete   | 2026-03-14 | - |
| 32. Feature Documentation | v2.2.2 | 0/TBD | Not started | - |
| 33. Accuracy Audit | v2.2.2 | 0/TBD | Not started | - |

**Total:** 33 phases across 12 milestones. 82 plans complete, 4 phases pending (v2.2.2).

---

*Created 2026-02-06 -- Updated 2026-03-14 Phase 31 planned (2 plans).*

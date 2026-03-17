# RuleZ Roadmap

**Current Focus:** v2.3.0 Multi-Runtime Skill Portability — Phase 37 next

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
- ✅ **v2.2.2 Documentation Audit & Multi-CLI Guides** — Phases 30-33 (shipped 2026-03-17) — [Archive](milestones/v2.2.2-ROADMAP.md)
- 🚧 **v2.3.0 Multi-Runtime Skill Portability** — Phases 34-38 (in progress)

---

### 🚧 v2.3.0 Multi-Runtime Skill Portability (In Progress)

**Milestone Goal:** Build an installer-based conversion pipeline that transforms canonical Claude Code skills into runtime-specific installations. Author once in `.claude/`, convert at install time, run everywhere.

## Phases

- [x] **Phase 34: Runtime Profiles and Skill Discovery** - Data model and discovery layer for all supported runtimes
- [x] **Phase 35: Transformation Engine** - Content transformation pipeline with 6 transform types
- [x] **Phase 36: CLI Integration and File Writer** - `rulez skills install` and `rulez skills clean` with file writer
- [ ] **Phase 37: Config File Generation and Mastering-Hooks** - Auto-generate GEMINI.md/AGENTS.md skill registries and handle mastering-hooks
- [ ] **Phase 38: Status, Diff, Sync, and DX Polish** - Complete `rulez skills` subcommand family with status/diff/sync and colorized output

## Phase Details

### Phase 34: Runtime Profiles and Skill Discovery
**Goal**: Users can describe any supported runtime and discover all skills from canonical `.claude/` sources
**Depends on**: Nothing (foundation phase)
**Requirements**: PROFILE-01, PROFILE-02, PROFILE-03, PROFILE-04
**Status**: Complete (2026-03-16)
**Success Criteria** (what must be TRUE):
  1. A `Runtime` enum covers Claude, OpenCode, Gemini, Codex, and Custom variants
  2. Each runtime profile correctly resolves its skills dir, commands dir, command separator, tool name style, and path prefix
  3. Skill discovery scans `.claude/skills/` and `.claude/commands/` and returns a manifest of all skills and commands
  4. Extra skill sources outside `.claude/` (e.g., mastering-hooks at repo root) are discovered automatically
  5. Custom runtime support works with a `--dir` flag pointing to any generic skill target
**Plans**: TBD

### Phase 35: Transformation Engine
**Goal**: Skill content is correctly transformed from Claude Code conventions to each target runtime's conventions
**Depends on**: Phase 34
**Requirements**: XFORM-01, XFORM-02, XFORM-03, XFORM-04, XFORM-05
**Status**: Complete (2026-03-16)
**Success Criteria** (what must be TRUE):
  1. Tool names in skill content are rewritten from PascalCase to runtime convention (lowercase for OpenCode, snake_case for Gemini)
  2. Path references (`~/.claude/`) are rewritten to the target runtime equivalent
  3. Command filenames are flattened from dot-separated to hyphen-separated with all cross-references updated
  4. YAML frontmatter is converted (allowed-tools to tools format, color fields handled, unsupported fields stripped)
  5. MCP tools are excluded for Gemini and preserved for OpenCode/Codex
**Plans**: TBD

### Phase 36: CLI Integration and File Writer
**Goal**: Users can install transformed skills to any target runtime using `rulez skills install` and remove them with `rulez skills clean`
**Depends on**: Phase 35
**Requirements**: CLI-01, CLI-02, CLI-03, CLI-04
**Status**: Complete (2026-03-16)
**Success Criteria** (what must be TRUE):
  1. `rulez skills install --runtime <rt>` writes transformed skills to the target runtime directory
  2. `rulez skills install --dry-run` prints a preview of what would be installed without writing any files
  3. `rulez skills clean --runtime <rt>` removes generated skill files for the specified runtime
  4. The writer uses clean-install semantics (removes existing target directory before writing fresh)
**Plans**: TBD

### Phase 37: Config File Generation and Mastering-Hooks
**Goal**: Installing skills to Gemini or Codex automatically updates the runtime config file with a skill registry, and mastering-hooks installs correctly from its non-standard location
**Depends on**: Phase 36
**Requirements**: CONFIG-01, CONFIG-02, CONFIG-03, CONFIG-04
**Success Criteria** (what must be TRUE):
  1. After `rulez skills install --runtime gemini`, `GEMINI.md` contains an updated skill registry section bounded by `<!-- RULEZ_SKILLS_START -->` / `<!-- RULEZ_SKILLS_END -->` markers
  2. After `rulez skills install --runtime codex`, `AGENTS.md` is generated (or updated) with a skill registry section listing all installed skills
  3. Non-skill content in GEMINI.md and AGENTS.md is preserved unchanged across repeated installs
  4. `rulez skills install` discovers and installs mastering-hooks from the repo root with context-aware platform reference rewriting
**Plans**: TBD

Plans:
- [ ] 37-01: Config file generation (GEMINI.md and AGENTS.md marker-based update)
- [ ] 37-02: Mastering-hooks special handling and platform reference rewriting

### Phase 38: Status, Diff, Sync, and DX Polish
**Goal**: Users can inspect installation state, preview changes, and sync all runtimes in one command with a polished colorized CLI experience
**Depends on**: Phase 37
**Requirements**: DX-01, DX-02, DX-03, DX-04
**Success Criteria** (what must be TRUE):
  1. `rulez skills status` shows a table with each runtime, installation state, and human-readable relative timestamps (e.g., "2 hours ago") indicating freshness
  2. `rulez skills diff --runtime <rt>` shows a colored diff of what would change if skills were re-installed now
  3. `rulez skills sync` installs to all detected runtimes in one command and reports per-runtime progress
  4. All install/sync operations emit colorized output with progress indicators showing which files are being written
**Plans**: TBD

Plans:
- [ ] 38-01: `rulez skills status` with freshness comparison
- [ ] 38-02: `rulez skills diff` with colored output
- [ ] 38-03: `rulez skills sync` and DX polish (colorized output, progress indicators)

---

## Progress

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
| 30-33 | v2.2.2 | 8/8 | Complete | 2026-03-17 |
| 34 | v2.3.0 | TBD | Complete | 2026-03-16 |
| 35 | v2.3.0 | TBD | Complete | 2026-03-16 |
| 36 | v2.3.0 | TBD | Complete | 2026-03-16 |
| 37. Config File Generation and Mastering-Hooks | v2.3.0 | 0/2 | Not started | - |
| 38. Status, Diff, Sync, and DX Polish | v2.3.0 | 0/3 | Not started | - |

**Total:** 38 phases across 13 milestones. 83 plans complete (v2.3.0 plans TBD).

---

*Created 2026-02-06 — Updated 2026-03-17 after v2.3.0 roadmap created.*

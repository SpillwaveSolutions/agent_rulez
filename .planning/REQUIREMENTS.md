# Requirements: RuleZ Multi-Runtime Skill Portability

**Defined:** 2026-03-16
**Core Value:** Author skills once in `.claude/`, convert at install time, run everywhere.

## v2.3.0 Requirements

### Profiles — Runtime profiles and discovery

- [x] **PROFILE-01**: Runtime profiles define per-platform conventions (skills dir, commands dir, command separator, tool name style, path prefix)
- [x] **PROFILE-02**: Skill discovery scans `.claude/skills/` and `.claude/commands/` building a manifest of skills and commands
- [x] **PROFILE-03**: Extra skill sources outside standard location (mastering-hooks at repo root) discovered automatically
- [x] **PROFILE-04**: Custom runtime support via `--dir` flag for generic skill targets

### Transform — Content transformation engine

- [x] **XFORM-01**: Tool names converted from Claude PascalCase to runtime conventions (lowercase for OpenCode, snake_case for Gemini)
- [x] **XFORM-02**: Path references rewritten (`~/.claude/` -> `~/.config/opencode/`, `~/.gemini/`, `~/.codex/`)
- [x] **XFORM-03**: Command filenames flattened from dot-separated to hyphen-separated with cross-reference rewriting
- [x] **XFORM-04**: YAML frontmatter converted (allowed-tools -> tools format, color hex, strip unsupported fields)
- [x] **XFORM-05**: MCP tools excluded for Gemini (auto-discovered), preserved for OpenCode/Codex

### CLI — CLI integration and file writer

- [x] **CLI-01**: `rulez skills install --runtime <rt>` installs transformed skills to target runtime directory
- [x] **CLI-02**: `rulez skills install --dry-run` previews what would be installed without writing
- [x] **CLI-03**: `rulez skills clean --runtime <rt>` removes generated skill files for a runtime
- [x] **CLI-04**: Clean-install writer removes existing target directory before writing fresh

### Config — Config file generation

- [ ] **CONFIG-01**: After installing to `.gemini/skills/`, auto-update `GEMINI.md` skill registry section using `<!-- RULEZ_SKILLS_START -->` / `<!-- RULEZ_SKILLS_END -->` markers
- [ ] **CONFIG-02**: Auto-generate `AGENTS.md` for Codex with skill registry section
- [ ] **CONFIG-03**: Preserve non-skill sections of config files during update
- [ ] **CONFIG-04**: Mastering-hooks platform references rewritten with context-aware handling (lives at repo root, not in `.claude/skills/`)

### DX — Developer experience polish

- [ ] **DX-01**: `rulez skills status` shows human-readable relative timestamps (e.g., "2 hours ago") and mtime freshness comparison
- [ ] **DX-02**: `rulez skills diff --runtime <rt>` shows colored diff of what would change if skills were re-installed
- [ ] **DX-03**: `rulez skills sync` installs to all detected runtimes in one command with per-runtime progress
- [ ] **DX-04**: Colorized terminal output with progress indicators for install/sync operations

## Future Requirements

### Extended Portability
- **PORT-01**: Copilot VSCode extension skill generation (different model from file-based skills)
- **PORT-02**: Watch mode that auto-reinstalls when `.claude/skills/` changes
- **PORT-03**: YAML-configurable transformation rules for custom runtimes

## Out of Scope

| Feature | Reason |
|---------|--------|
| Copilot skill distribution | VSCode extension model is fundamentally different from file-based skills |
| YAML-configurable transforms | 4 well-known runtimes have stable conventions; Custom variant handles long tail |
| Global skill registry/marketplace | Not needed for single-project portability |
| Bidirectional sync (other -> Claude) | One canonical source (Claude Code), convert outward only |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| PROFILE-01 | Phase 34 | Complete |
| PROFILE-02 | Phase 34 | Complete |
| PROFILE-03 | Phase 34 | Complete |
| PROFILE-04 | Phase 34 | Complete |
| XFORM-01 | Phase 35 | Complete |
| XFORM-02 | Phase 35 | Complete |
| XFORM-03 | Phase 35 | Complete |
| XFORM-04 | Phase 35 | Complete |
| XFORM-05 | Phase 35 | Complete |
| CLI-01 | Phase 36 | Complete |
| CLI-02 | Phase 36 | Complete |
| CLI-03 | Phase 36 | Complete |
| CLI-04 | Phase 36 | Complete |
| CONFIG-01 | Phase 37 | Pending |
| CONFIG-02 | Phase 37 | Pending |
| CONFIG-03 | Phase 37 | Pending |
| CONFIG-04 | Phase 37 | Pending |
| DX-01 | Phase 38 | Pending |
| DX-02 | Phase 38 | Pending |
| DX-03 | Phase 38 | Pending |
| DX-04 | Phase 38 | Pending |

**Coverage:**
- v2.3.0 requirements: 21 total
- Mapped to phases: 21
- Unmapped: 0 ✓
- Complete: 13 (Phases 34-36)
- Pending: 8 (Phases 37-38)

---
*Requirements defined: 2026-03-16*
*Last updated: 2026-03-17 after roadmap creation*

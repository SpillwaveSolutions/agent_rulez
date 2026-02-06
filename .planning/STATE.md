# Living Memory

**Last Updated:** 2026-02-06
**Current Focus:** Brainstorming next steps after monorepo reorganization

---

## Position

- **Project:** RuleZ (renamed from CCH)
- **Status:** Monorepo reorganization complete
- **Next:** Determine priorities across all three components

---

## Recent Changes (2026-02-06)

1. **Converted SDD → GSD**
   - Kept .speckit/ as reference
   - Created .planning/ for GSD workflow
   - Codebase mapped with 7 documents

2. **Monorepo Reorganization**
   - `cch_cli/` → `rulez/` (binary now `rulez` not `cch`)
   - `rulez_ui/` → `rulez-ui/`
   - `mastering-hooks/` unchanged
   - Empty `src/` directory removed
   - All Cargo.toml, Taskfile.yml, CLAUDE.md updated

---

## Component Status

| Component | Status | Next Action |
|-----------|--------|-------------|
| **RuleZ Core** | v1.1.0 | Determine next features |
| **Mastering Hooks** | Complete skill | Consider plugin conversion |
| **RuleZ UI** | M1 done | Lower priority |

---

## Open Questions

1. What's the next priority for RuleZ Core?
   - More matchers/actions?
   - Performance improvements?
   - Better CLI UX?

2. Should mastering-hooks become a plugin now?
   - What's the plugin format?
   - Use the agent-skill-converter?

3. What about integration testing (IQ/OQ/PQ)?
   - Is it fully set up?
   - Are all tests passing?

---

## Pending Todos

- 0 pending (monorepo reorg completed)

---

## Context for Next Session

Monorepo is reorganized. Ready to brainstorm next steps and reprioritize the roadmap based on actual needs rather than the RuleZ UI-focused roadmap that was converted from SDD.

Key files:
- `rulez/` - Core binary (the main product)
- `mastering-hooks/` - Skill to help users use RuleZ
- `rulez-ui/` - Optional desktop app

---

*State file for GSD workflow continuity*

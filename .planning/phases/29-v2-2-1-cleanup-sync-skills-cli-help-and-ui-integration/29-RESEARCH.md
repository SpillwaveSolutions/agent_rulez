# Phase 29: v2.2.1 Cleanup - Sync Skills, CLI Help, and UI Integration - Research

**Researched:** 2026-03-12
**Domain:** Skill documentation, CLI consistency, UI routing
**Confidence:** HIGH

## Summary

This is a cleanup/sync phase addressing documentation and wiring gaps that accumulated across phases 29-36 (v2.2 milestone). The work is entirely non-Rust -- it involves renaming stale "cch" references in shell scripts and markdown files, adding missing CLI command documentation to the mastering-hooks skill, and wiring an existing but unrouted React component (ConfigDiffView) into the UI app.

All five gaps have been verified through direct codebase inspection. The `release-cch` skill has 69 occurrences of "cch" across 9 files in `.claude/skills/` and 80 occurrences across 8 files in `.opencode/skill/`. The `preflight-check.sh` script references `cch_cli` directory paths that no longer exist (the Rust workspace root is now `rulez/`). The `mastering-hooks` skill's `cli-commands.md` documents 10 commands but is missing `test`, `lint`, `upgrade`, and all multi-platform install commands (`gemini install`, `copilot install`, `opencode install`). The `ConfigDiffView` component exists at `rulez-ui/src/components/config/ConfigDiffView.tsx` but is only imported in that single file -- no route, no navigation button, no way for users to reach it.

**Primary recommendation:** Tackle this as five independent work items: (1) preflight-check.sh path fix, (2) release-cch -> release-rulez rename, (3) add missing CLI commands to mastering-hooks docs, (4) wire ConfigDiffView into UI, (5) verify the opencode skill copy is also updated.

## Standard Stack

This phase involves no new libraries or dependencies. All work is on existing files.

### Core Files to Modify

| Location | File Count | Change Type |
|----------|-----------|-------------|
| `.claude/skills/release-cch/` | 9 files | Rename cch -> rulez (69 occurrences) |
| `.opencode/skill/release-cch/` | 8 files | Rename cch -> rulez (80 occurrences) |
| `mastering-hooks/references/cli-commands.md` | 1 file | Add 7 missing command docs |
| `rulez-ui/src/` | ~4 files | Wire ConfigDiffView into navigation |

### Technologies in Play

| Technology | Version | Purpose | Notes |
|-----------|---------|---------|-------|
| Bash scripts | N/A | preflight-check.sh, read-version.sh, verify-release.sh | Path references to fix |
| Markdown | N/A | Skill documentation, references | Bulk rename cch -> rulez |
| React 18 | 18.x | UI components | Wire ConfigDiffView |
| Zustand | (existing) | UI state (uiStore.ts) | Add "diff" to MainView type |
| Monaco DiffEditor | (existing) | ConfigDiffView component | Already implemented, just unrouted |

## Architecture Patterns

### Skill Directory Structure (existing)
```
.claude/skills/release-cch/     # <-- directory name needs renaming
  SKILL.md                       # Skill metadata + workflow
  scripts/
    preflight-check.sh           # References cch_cli paths
    read-version.sh              # Minor cch reference
    verify-release.sh            # cch references in asset names
    generate-changelog.sh
  references/
    release-workflow.md          # cch asset names in tables/diagrams
    hotfix-workflow.md           # cch references
    troubleshooting.md           # cch references
  templates/
    pr-body.md                   # cch references
  README.md                     # cch references
```

### UI Navigation Pattern (existing)
```
Header.tsx
  - View switcher: "Editor" | "Logs"   <-- needs "Diff" option added
  - Drives `mainView` state in uiStore

uiStore.ts
  - MainView type: "editor" | "logs"   <-- needs "diff" added
  - RightPanelTab type: "simulator" | "tree" | "settings"

MainContent.tsx
  - Switches on mainView: editor | logs  <-- needs diff case
  - ConfigDiffView already exists at components/config/ConfigDiffView.tsx
```

### Pattern: Adding a New Main View to RuleZ UI

1. **uiStore.ts**: Add `"diff"` to `MainView` type union
2. **Header.tsx**: Add "Diff" button to view switcher (alongside Editor/Logs)
3. **MainContent.tsx**: Add `if (mainView === "diff")` case returning `<ConfigDiffView />`

This follows the exact pattern used when the "Logs" view was added.

### Mastering-Hooks CLI Reference Pattern

The existing `cli-commands.md` follows this pattern for each command:
```markdown
### command-name

Description paragraph.

\`\`\`bash
rulez command [OPTIONS]

Options:
  --flag    Description
\`\`\`

**Examples**:
\`\`\`bash
rulez command --example
\`\`\`
```

New commands to document should follow this exact format.

### Anti-Patterns to Avoid
- **Partial rename**: Do NOT rename just the visible text while leaving directory names unchanged. The directory `.claude/skills/release-cch/` should be renamed to `.claude/skills/release-rulez/` so Claude Code can discover it properly.
- **Forgetting the OpenCode copy**: The `.opencode/skill/release-cch/` directory mirrors `.claude/skills/release-cch/` and has the same stale references. Both must be updated together.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Bulk rename | Manual find-replace | `sed -i` or targeted edits | 149 total occurrences across 17 files |
| ConfigDiffView | New component | Existing `ConfigDiffView.tsx` | Already fully implemented with Monaco DiffEditor |
| CLI command docs | Guessing at flags | `rulez <command> --help` output | Source of truth is Rust clap definitions |

## Common Pitfalls

### Pitfall 1: Breaking Script Paths After Directory Rename
**What goes wrong:** If the skill directory is renamed from `release-cch` to `release-rulez`, all internal script references using relative paths (`SCRIPT_DIR`) will still work. But any external references in SKILL.md or README that hardcode `.claude/skills/release-cch/scripts/...` will break.
**How to avoid:** Search the entire repo for references to `release-cch` path after renaming.

### Pitfall 2: preflight-check.sh Still References cch_cli Directory
**What goes wrong:** `preflight-check.sh` does `cd "$REPO_ROOT/cch_cli"` on lines 86 and 134. This directory no longer exists -- the Rust workspace uses `rulez/` as the crate directory.
**How to avoid:** Replace `cch_cli` with `rulez` in all `cd` commands and error messages. Also update the Cargo workspace-relative commands since the repo now uses workspace-level `cargo` commands (per CLAUDE.md: `cargo fmt --all --check`, `cargo clippy --all-targets --all-features --workspace`).

### Pitfall 3: Release Asset Names in Documentation
**What goes wrong:** The `release-workflow.md` and `SKILL.md` list asset names as `cch-linux-x86_64.tar.gz`, `cch-macos-aarch64.tar.gz`, etc. But the actual GitHub Actions workflow already uses `rulez-*` names (verified: no `cch-` or `cch_` references in `.github/workflows/`).
**How to avoid:** Update all asset name references in skill docs to use `rulez-*` prefix.

### Pitfall 4: Forgetting to Update Skill Metadata
**What goes wrong:** The SKILL.md frontmatter has `name: release-cch` and `metadata.project: "cch"`. Claude Code uses this metadata for skill discovery. Stale names mean the skill may not trigger correctly.
**How to avoid:** Update the YAML frontmatter in SKILL.md: `name: release-rulez`, `metadata.project: "rulez"`, and update the description text.

### Pitfall 5: Missing CLI Commands in Mastering-Hooks
**What goes wrong:** Users ask about `rulez test` or `rulez lint` and the skill has no documentation for them, leading to incorrect guidance.
**How to avoid:** Document all seven missing commands by consulting the Rust source:
- `rulez test` (cli/test.rs) -- batch test scenarios
- `rulez lint` (cli/lint.rs) -- config linting
- `rulez upgrade` (cli/upgrade.rs) -- self-update
- `rulez gemini install` (cli/gemini_install.rs) -- Gemini CLI setup
- `rulez copilot install` (cli/copilot_install.rs) -- Copilot setup
- `rulez opencode install` (cli/opencode_install.rs) -- OpenCode setup
- `rulez <platform> doctor` -- diagnostic commands

## Code Examples

### UI: Adding "Diff" View to Header.tsx
```typescript
// In the view switcher div, add alongside Editor and Logs buttons:
<button
  type="button"
  onClick={() => setMainView("diff")}
  className={`px-2 py-0.5 text-xs rounded transition-colors ${
    mainView === "diff"
      ? "bg-white dark:bg-gray-600 text-gray-900 dark:text-gray-100 shadow-sm"
      : "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200"
  }`}
  aria-label="Diff"
>
  Diff
</button>
```

### UI: Adding Diff Case to MainContent.tsx
```typescript
// After the logs check:
if (mainView === "diff") {
  return (
    <main className="flex-1 flex flex-col min-w-0 overflow-hidden">
      <ConfigDiffView />
    </main>
  );
}
```

### UI: Updating uiStore.ts Type
```typescript
export type MainView = "editor" | "logs" | "diff";
```

### preflight-check.sh: Path Fix
```bash
# OLD (broken):
cd "$REPO_ROOT/cch_cli"

# NEW (correct - use workspace-level commands):
cd "$REPO_ROOT"
# Then update cargo commands to use --workspace flag
```

## State of the Art

| Old State | Current State | Impact |
|-----------|--------------|--------|
| Binary named `cch` | Binary renamed to `rulez` (commit 39e6185, 2026-02-06) | Skill docs still say cch |
| No `test`/`lint` commands | Added in Phase 30/36 (v2.2) | Not documented in mastering-hooks |
| No multi-platform install docs | Gemini/Copilot/OpenCode install commands exist | Not documented in cli-commands.md |
| ConfigDiffView created (Phase 34) | Component exists but unrouted | Users cannot access it |
| GitHub workflows used `cch-*` asset names | Workflows already updated to `rulez-*` | Only skill docs are stale |

## Open Questions

1. **Should the skill directory be renamed?**
   - What we know: The directory is `.claude/skills/release-cch/`. Claude Code discovers skills by directory name + SKILL.md frontmatter.
   - What's unclear: Whether renaming the directory will break any cached references in Claude Code.
   - Recommendation: YES, rename to `release-rulez`. Update the SKILL.md frontmatter `name` field. Claude Code re-scans on each session so no caching issue.

2. **Should `rulez <platform> hook` commands be documented?**
   - What we know: CLI source has `gemini_hook.rs`, `copilot_hook.rs`, `opencode_hook.rs` alongside the install commands.
   - What's unclear: Whether these are user-facing or internal plumbing.
   - Recommendation: Document `install` and `doctor` commands. The `hook` commands are the actual hook handlers invoked by the platform -- they are plumbing, not user-facing.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust (cargo test) + Vitest (UI) |
| Config file | `Cargo.toml` (workspace), `rulez-ui/vite.config.ts` |
| Quick run command | `cargo test --tests --all-features --workspace` |
| Full suite command | `cargo llvm-cov --all-features --workspace --no-report` |

### Phase Requirements -> Test Map

This is a cleanup/documentation phase. Most changes are to markdown and shell scripts which do not have automated tests. The UI wiring change can be validated:

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| CLEANUP-01 | preflight-check.sh uses correct paths | manual | Run `bash .claude/skills/release-rulez/scripts/preflight-check.sh` | N/A |
| CLEANUP-02 | No "cch" references in skill files | manual | `grep -ri cch .claude/skills/release-rulez/` should return 0 | N/A |
| CLEANUP-03 | CLI commands documented | manual | Visual inspection of cli-commands.md | N/A |
| CLEANUP-04 | ConfigDiffView accessible in UI | smoke | `cd rulez-ui && npx vitest run` | Depends on existing tests |
| CLEANUP-05 | OpenCode skill copy updated | manual | `grep -ri cch .opencode/skill/release-rulez/` should return 0 | N/A |

### Sampling Rate
- **Per task commit:** `cargo fmt --all --check && cargo clippy --all-targets --all-features --workspace -- -D warnings`
- **Per wave merge:** `cargo test --tests --all-features --workspace`
- **Phase gate:** Full CI pipeline before push

### Wave 0 Gaps
None -- this phase modifies documentation and UI wiring only. No new test infrastructure needed.

## Sources

### Primary (HIGH confidence)
- Direct codebase inspection of all files referenced in this research
- `grep -ri cch` across skill directories -- verified exact counts
- UI component inspection -- confirmed ConfigDiffView is unrouted
- CLI source files -- confirmed `test.rs`, `lint.rs`, `upgrade.rs`, `gemini_install.rs`, `copilot_install.rs`, `opencode_install.rs` exist
- GitHub workflows -- confirmed no stale `cch` references in `.github/workflows/`

## Metadata

**Confidence breakdown:**
- Stale cch references: HIGH - direct grep counts verified
- preflight-check.sh fix: HIGH - read the script, confirmed `cch_cli` paths
- Missing CLI docs: HIGH - compared cli-commands.md against actual CLI source files
- ConfigDiffView wiring: HIGH - read all UI routing code, confirmed no references
- UI wiring pattern: HIGH - existing Editor/Logs pattern is clear

**Research date:** 2026-03-12
**Valid until:** 2026-04-12 (stable -- no external dependencies)

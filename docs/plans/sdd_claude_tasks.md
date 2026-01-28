# Migration Plan: Speckit to Claude Tasks + Parallel Feature Implementation

**Created:** 2026-01-25
**Status:** Ready for Implementation

## Summary

1. Migrate OpenCode files to Claude format
2. Hydrate Claude tasks from speckit
3. **Parallel Implementation**: Spin up multiple agents to work on:
   - **phase2-governance** (Rust, in `cch_cli/`)
   - **rulez-ui** (React/Tauri, in `rulez_ui/`)

---

## Part 0: Parallel Agent Strategy

### Agent Assignments

| Feature | Directory | Technology | Agent Skills |
|---------|-----------|------------|--------------|
| phase2-governance | `cch_cli/` | Rust | rust-expert, qa-enforcer |
| rulez-ui | `rulez_ui/` | React/Tauri/TypeScript | react-best-practices, mastering-typescript |

### Access Rights

**Phase2-Governance Agent:**
- Read/Write: `cch_cli/`
- Read: `.speckit/features/phase2-governance/`

**RuleZ-UI Agent:**
- Read/Write: `rulez_ui/`
- Read: `.speckit/features/rulez-ui/`

### Available Skills (in `.claude/skills/`)

| Skill | Use For |
|-------|---------|
| mastering-typescript | rulez-ui TypeScript development |
| react-best-practices | rulez-ui React components |
| mastering-git-cli | Both - git operations |
| mastering-github-cli | Both - PR creation |
| pr-reviewer | Both - code review |
| documentation-specialist | Both - docs |
| architect-agent | Both - planning |

### Agent Work Breakdown

**Phase2-Governance (Rust):**
- P2.2: Enhanced Logging (4 tasks)
- P2.3: CLI Enhancements (4 tasks)
- P2.4: Trust Levels (4 tasks)
- Total: 12 tasks

**RuleZ-UI (React/Tauri):**
- M1: Project Setup (3 tasks) - **rulez_ui/ is empty, needs full setup**
- M2: Monaco Editor (3 tasks)
- M3: Schema Validation (4 tasks)
- M4: File Operations (4 tasks)
- M5: Rule Tree View (3 tasks)
- M6: Debug Simulator (5 tasks)
- M7: Theming (4 tasks)
- M8: Playwright Tests (5 tasks)
- Total: 31 tasks

---

## Part 1: Speckit to Claude Tasks Migration

### Understanding

- **Claude native tasks** are session-scoped (ephemeral) using `TaskCreate`, `TaskUpdate`, `TaskList`, `TaskGet`
- **`.speckit` files** remain the persistent source of truth
- **Strategy**: Hydrate Claude tasks from speckit at session start, sync back on completion

### Task Hydration Sequence

Create Claude tasks from `.speckit/features/phase2-governance/tasks.md` for incomplete phases:

**Phase 2.1 Core Governance** (P2.1-T01 through P2.1-T06) - Already implemented per git history, but verify checkboxes in tasks.md

**Phase 2.2 Enhanced Logging** (4 tasks):
| Task ID | Subject | Dependencies |
|---------|---------|--------------|
| P2.2-T01 | Add Decision enum to models | P2.1-T06 (complete) |
| P2.2-T02 | Extend LogEntry struct with governance fields | P2.2-T01 |
| P2.2-T03 | Update log writer for governance fields | P2.2-T02 |
| P2.2-T04 | Update log querying with mode/decision filters | P2.2-T03 |

**Phase 2.3 CLI Enhancements** (4 tasks, parallel to P2.2):
| Task ID | Subject | Dependencies |
|---------|---------|--------------|
| P2.3-T01 | Enhance cch explain rule command | P2.1-T06 (complete) |
| P2.3-T02 | Add activity statistics to explain | P2.3-T01 |
| P2.3-T03 | Add JSON output format to explain | P2.3-T02 |
| P2.3-T04 | Update CLI help text for governance | P2.3-T03 |

**Phase 2.4 Trust Levels** (4 tasks, parallel to P2.2/P2.3):
| Task ID | Subject | Dependencies |
|---------|---------|--------------|
| P2.4-T01 | Add trust field to run action | P2.1-T06 (complete) |
| P2.4-T02 | Create TrustLevel enum | P2.4-T01 |
| P2.4-T03 | Log trust levels in entries | P2.4-T02 |
| P2.4-T04 | Document trust levels in SKILL.md | P2.4-T03 |

### Implementation Steps

1. **Verify Phase 2.1 completion** - Check if tasks should be marked complete in tasks.md
2. **Create Claude tasks** for P2.2, P2.3, P2.4 using `TaskCreate` with:
   - `subject`: Task title (imperative form)
   - `description`: Details from tasks.md
   - `activeForm`: Present continuous form
   - `metadata`: `{"speckit_id": "P2.X-TXX", "phase": "2.X"}`
3. **Establish dependencies** using `TaskUpdate` with `addBlockedBy`
4. **Update tasks.md** after completing each task (change `[ ]` to `[x]`)

### Files to Update

- `.speckit/features/phase2-governance/tasks.md` - Mark completed tasks
- `.speckit/features.md` - Update phase2-governance status when complete

---

## Part 2: OpenCode to Claude Migration

### Command File Migration

**Source:** `.opencode/command/cch-release.md`
**Target:** `.claude/commands/cch-release.md`

**Changes:**
- Update 6 path references from `.opencode/skill/release-cch/` to `.claude/skills/release-cch/`

### Skill Directory Migration

**Source:** `.opencode/skill/release-cch/`
**Target:** `.claude/skills/release-cch/`

**File List:**
| Source File | Target File | Changes |
|-------------|-------------|---------|
| SKILL.md | SKILL.md | 15 path updates |
| references/release-workflow.md | references/release-workflow.md | None |
| references/hotfix-workflow.md | references/hotfix-workflow.md | 1 path update |
| references/troubleshooting.md | references/troubleshooting.md | 1 path update |
| scripts/read-version.sh | scripts/read-version.sh | Fix REPO_ROOT depth |
| scripts/generate-changelog.sh | scripts/generate-changelog.sh | Fix REPO_ROOT depth |
| scripts/preflight-check.sh | scripts/preflight-check.sh | Fix REPO_ROOT depth |
| scripts/verify-release.sh | scripts/verify-release.sh | Fix REPO_ROOT depth |
| templates/changelog-entry.md | templates/changelog-entry.md | None |
| templates/pr-body.md | templates/pr-body.md | 1 path update |
| (new) | README.md | Create for Claude format |

### Script Path Fix

All 4 scripts need REPO_ROOT depth correction:
```bash
# OpenCode (4 levels deep)
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"

# Claude (5 levels deep due to .claude/skills vs .opencode/skill)
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"
```

### Global Search/Replace

```
.opencode/skill/release-cch/  →  .claude/skills/release-cch/
.opencode/command/            →  .claude/commands/
```

---

## Part 3: Update Speckit Files

### Files to Update

1. **`.speckit/features.md`** - Add note that tasks can be hydrated to Claude native tasks
2. **`.speckit/constitution.md`** - Add workflow section for Claude tasks integration

---

## Execution Order

### Step 1: Migrate OpenCode Files
1. Create `.claude/skills/release-cch/` directory structure
2. Copy and update SKILL.md with path changes
3. Copy and update scripts with REPO_ROOT fix
4. Copy and update references with path changes
5. Copy templates (minimal changes)
6. Create README.md
7. Create `.claude/commands/cch-release.md` with path updates
8. Test `/cch-release` command works

### Step 2: Verify Phase 2.1 Status
1. Check git history for P2.1-T01 through P2.1-T06 completion
2. Update tasks.md checkboxes if needed
3. Update features.md status if P2.1 is complete

### Step 3: Hydrate Claude Tasks
1. Create 12 Claude tasks for P2.2, P2.3, P2.4
2. Set up dependency chain using `addBlockedBy`
3. Display task list to user

### Step 4: Spin Up Parallel Agents
1. Launch phase2-governance agent with access to `cch_cli/`
2. Launch rulez-ui agent with access to `rulez_ui/`
3. Agents work in parallel on their respective features

### Step 5: Update Documentation
1. Add Claude tasks workflow note to constitution.md
2. Optionally create a `speckit-hydrate` command for future use

---

## Verification

After migration:

- [ ] `/cch-release` command loads and shows help
- [ ] `/cch-release prepare` workflow functions correctly
- [ ] All scripts run correctly (read-version.sh returns version)
- [ ] `TaskList` shows tasks with correct dependencies
- [ ] No `.opencode/` references remain in `.claude/` files
- [ ] tasks.md accurately reflects completion status
- [ ] phase2-governance agent is implementing P2.2/P2.3/P2.4
- [ ] rulez-ui agent is implementing M1-M8

---

## Critical Files

**OpenCode Sources:**
- `.opencode/command/cch-release.md`
- `.opencode/skill/release-cch/SKILL.md`
- `.opencode/skill/release-cch/scripts/*.sh`

**Claude Targets:**
- `.claude/commands/cch-release.md`
- `.claude/skills/release-cch/`

**Speckit:**
- `.speckit/features/phase2-governance/tasks.md`
- `.speckit/features/rulez-ui/tasks.md`
- `.speckit/features.md`
- `.speckit/constitution.md`

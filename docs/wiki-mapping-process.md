# CCH Wiki Mapping Process

A guide describing how documentation files from the main repository are mapped and synchronized to the GitHub Wiki.

---

## Overview

CCH uses a manual copy-based wiki synchronization process where:
1. Source markdown files live in the main repository
2. A mapping file (`docs/wiki-mapping.yml`) defines source to wiki page relationships
3. Files are copied to the wiki repository and renamed according to the mapping
4. Wiki repository is committed and pushed separately

---

## Source Locations

Documentation comes from four main locations in the repository:

### 1. Core Documentation (`docs/`)

| Source Path | Wiki Page | Description |
|-------------|-----------|-------------|
| `docs/README.md` | `Home.md` | Main project documentation |
| `docs/USER_GUIDE_CLI.md` | `User-Guide-CLI.md` | CCH binary command reference |
| `docs/USER_GUIDE_SKILL.md` | `User-Guide-Skill.md` | CCH skill usage guide |
| `docs/BACKLOG.md` | `Backlog.md` | Product backlog items |
| `docs/IQ_OQ_PQ_IntegrationTesting.md` | `IQ-OQ-PQ-Integration-Testing.md` | Validation testing guide |
| `CHANGELOG.md` | `Changelog.md` | Version history |

### 2. DevOps Documentation (`docs/devops/`)

| Source Path | Wiki Page | Description |
|-------------|-----------|-------------|
| `docs/devops/BRANCHING.md` | `DevOps-Branching-Strategy.md` | Git workflow |
| `docs/devops/CI_TIERS.md` | `DevOps-CI-Tiers.md` | CI/CD configuration |
| `docs/devops/RELEASE_PROCESS.md` | `DevOps-Release-Process.md` | Release procedures |

### 3. Product Requirements (`docs/prds/`)

| Source Path | Wiki Page | Description |
|-------------|-----------|-------------|
| `docs/prds/cch_cli_prd.md` | `PRD-CCH-CLI.md` | CLI requirements |
| `docs/prds/cch_system.md` | `PRD-CCH-System.md` | System architecture |
| `docs/prds/mastering_hooks.md` | `PRD-Mastering-Hooks.md` | Hooks skill requirements |
| `docs/prds/phase2_prd.md` | `PRD-Phase2-Governance.md` | Phase 2 requirements |
| `docs/prds/rulez_ui_prd.md` | `PRD-RuleZ-UI.md` | Desktop UI requirements |

### 4. Feature Specs (`.speckit/features/`)

Feature specifications follow the SpecKit SDD pattern:

| Source Path Pattern | Wiki Page Pattern |
|---------------------|-------------------|
| `.speckit/features/{name}/spec.md` | `Feature-{Name}-Spec.md` |
| `.speckit/features/{name}/specify.md` | `Feature-{Name}-Spec.md` |
| `.speckit/features/{name}/plan.md` | `Feature-{Name}-Plan.md` |
| `.speckit/features/{name}/tasks.md` | `Feature-{Name}-Tasks.md` |

**Examples:**
- `.speckit/features/rulez-ui/spec.md` -> `Feature-RuleZ-UI-Spec.md`
- `.speckit/features/phase2-governance/plan.md` -> `Feature-Phase2-Governance-Plan.md`

---

## The Mapping File (`docs/wiki-mapping.yml`)

The mapping file serves as the source of truth for wiki synchronization. It's organized by category:

```yaml
# Core documentation
docs:
  docs/README.md: "Home"
  docs/USER_GUIDE_CLI.md: "User-Guide-CLI"

# DevOps documents
devops:
  docs/devops/BRANCHING.md: "DevOps-Branching-Strategy"

# Product Requirements Documents
prds:
  docs/prds/cch_cli_prd.md: "PRD-CCH-CLI"

# Feature specs (.speckit/features/)
feature-rulez-ui:
  .speckit/features/rulez-ui/spec.md: "Feature-RuleZ-UI-Spec"

# Checklists
checklists:
  .speckit/checklists/rulez-ui-checklist.md: "Checklist-RuleZ-UI"

# Tracking section (informational)
wiki_pages_created:
  - Home: "Description of the page"
```

---

## Naming Conventions

### Core Docs (simple names)

Standard documentation uses simple, hyphenated names:
- `User-Guide-CLI.md`
- `User-Guide-Skill.md`
- `Changelog.md`

### DevOps (DevOps- prefix)

DevOps documentation uses the `DevOps-` prefix:
- `DevOps-Branching-Strategy.md`
- `DevOps-CI-Tiers.md`
- `DevOps-Release-Process.md`

### PRDs (PRD- prefix)

Product requirements use the `PRD-` prefix:
- `PRD-CCH-CLI.md`
- `PRD-RuleZ-UI.md`

### Feature Specs (Feature- prefix)

SpecKit feature documentation uses the `Feature-` prefix:
- `Feature-RuleZ-UI-Spec.md`
- `Feature-Phase2-Governance-Plan.md`
- `Feature-CCH-Binary-v1-Tasks.md`

### Checklists (Checklist- prefix)

Completion checklists use the `Checklist-` prefix:
- `Checklist-Phase2-Governance.md`
- `Checklist-RuleZ-UI.md`

### SpecKit Core (SpecKit- prefix)

Project constitution and feature index:
- `SpecKit-Constitution.md`
- `SpecKit-Features-Index.md`

---

## Synchronization Process

### Step 1: Clone Wiki Repository

```bash
git clone https://github.com/SpillwaveSolutions/code_agent_context_hooks.wiki.git /tmp/cch-wiki
```

### Step 2: Copy Files with Renaming

For each mapping in `wiki-mapping.yml`, copy and rename:

```bash
# Core docs
cp docs/README.md /tmp/cch-wiki/Home.md
cp docs/USER_GUIDE_CLI.md /tmp/cch-wiki/User-Guide-CLI.md
cp docs/USER_GUIDE_SKILL.md /tmp/cch-wiki/User-Guide-Skill.md

# DevOps
cp docs/devops/BRANCHING.md /tmp/cch-wiki/DevOps-Branching-Strategy.md
cp docs/devops/CI_TIERS.md /tmp/cch-wiki/DevOps-CI-Tiers.md
cp docs/devops/RELEASE_PROCESS.md /tmp/cch-wiki/DevOps-Release-Process.md

# PRDs
cp docs/prds/cch_cli_prd.md /tmp/cch-wiki/PRD-CCH-CLI.md
cp docs/prds/cch_system.md /tmp/cch-wiki/PRD-CCH-System.md
cp docs/prds/rulez_ui_prd.md /tmp/cch-wiki/PRD-RuleZ-UI.md

# Feature specs
cp .speckit/features/rulez-ui/spec.md /tmp/cch-wiki/Feature-RuleZ-UI-Spec.md
cp .speckit/features/rulez-ui/plan.md /tmp/cch-wiki/Feature-RuleZ-UI-Plan.md
cp .speckit/features/rulez-ui/tasks.md /tmp/cch-wiki/Feature-RuleZ-UI-Tasks.md

# SpecKit core
cp .speckit/constitution.md /tmp/cch-wiki/SpecKit-Constitution.md
cp .speckit/features.md /tmp/cch-wiki/SpecKit-Features-Index.md
```

### Step 3: Update Home.md

Add wiki links to new pages using `[[Page-Name]]` syntax:

```markdown
## User Documentation
- [[User-Guide-CLI]] - CCH binary command reference
- [[User-Guide-Skill]] - Skill usage guide

## DevOps
- [[DevOps-Branching-Strategy]] - Git workflow
- [[DevOps-CI-Tiers]] - CI/CD configuration
- [[DevOps-Release-Process]] - Release procedures

## Product Requirements
- [[PRD-CCH-CLI]] - CLI product requirements
- [[PRD-RuleZ-UI]] - Desktop UI requirements

## Feature Documentation
- [[Feature-RuleZ-UI-Spec]] - RuleZ UI specification
- [[Feature-Phase2-Governance-Spec]] - Phase 2 governance
```

### Step 4: Commit and Push Wiki

```bash
cd /tmp/cch-wiki
git add .
git commit -m "docs: sync wiki with features X, Y, Z"
git push origin master
```

### Step 5: Update Main Repository

Update `wiki-mapping.yml` with any new mappings, then commit to main repo.

---

## File Organization Summary

```
using_hooks_plugin/
├── docs/                            # Core documentation
│   ├── README.md                   -> Home.md
│   ├── USER_GUIDE_CLI.md           -> User-Guide-CLI.md
│   ├── USER_GUIDE_SKILL.md         -> User-Guide-Skill.md
│   ├── devops/
│   │   ├── BRANCHING.md            -> DevOps-Branching-Strategy.md
│   │   ├── CI_TIERS.md             -> DevOps-CI-Tiers.md
│   │   └── RELEASE_PROCESS.md      -> DevOps-Release-Process.md
│   ├── prds/
│   │   ├── cch_cli_prd.md          -> PRD-CCH-CLI.md
│   │   └── rulez_ui_prd.md         -> PRD-RuleZ-UI.md
│   └── wiki-mapping.yml            # THE MAPPING FILE
│
├── .speckit/                        # SpecKit SDD artifacts
│   ├── constitution.md             -> SpecKit-Constitution.md
│   ├── features.md                 -> SpecKit-Features-Index.md
│   ├── features/
│   │   ├── rulez-ui/
│   │   │   ├── spec.md             -> Feature-RuleZ-UI-Spec.md
│   │   │   ├── plan.md             -> Feature-RuleZ-UI-Plan.md
│   │   │   └── tasks.md            -> Feature-RuleZ-UI-Tasks.md
│   │   └── phase2-governance/
│   │       └── ...
│   └── checklists/
│       ├── rulez-ui-checklist.md   -> Checklist-RuleZ-UI.md
│       └── ...
│
└── CHANGELOG.md                     -> Changelog.md
```

---

## Wiki Repository Structure

After sync, the wiki repo looks like:

```
code_agent_context_hooks.wiki/
├── Home.md                          # Main landing page
├── _Sidebar.md                      # Navigation sidebar
│
├── # User Documentation
├── User-Guide-CLI.md
├── User-Guide-Skill.md
├── Backlog.md
├── Changelog.md
│
├── # DevOps
├── DevOps-Branching-Strategy.md
├── DevOps-CI-Tiers.md
├── DevOps-Release-Process.md
│
├── # Product Requirements
├── PRD-CCH-CLI.md
├── PRD-CCH-System.md
├── PRD-Mastering-Hooks.md
├── PRD-Phase2-Governance.md
├── PRD-RuleZ-UI.md
│
├── # SpecKit
├── SpecKit-Constitution.md
├── SpecKit-Features-Index.md
│
├── # Feature Specs
├── Feature-CCH-Binary-v1-Spec.md
├── Feature-CCH-Binary-v1-Plan.md
├── Feature-CCH-Binary-v1-Tasks.md
├── Feature-RuleZ-UI-Spec.md
├── Feature-RuleZ-UI-Plan.md
├── Feature-RuleZ-UI-Tasks.md
├── Feature-Phase2-Governance-Spec.md
├── ...
│
└── # Checklists
└── Checklist-RuleZ-UI.md
```

---

## Key Points

1. **`wiki-mapping.yml` is the source of truth** - Always update it when adding new documentation
2. **Manual process** - No automation; files are copied manually during sync
3. **Two repositories** - Main repo and wiki repo are separate; both need commits
4. **Naming matters** - Use correct prefixes (Feature-, PRD-, DevOps-, etc.)
5. **Home.md needs updating** - Add links to new pages using `[[Page-Name]]` syntax
6. **Wiki uses master branch** - Not main

---

## When to Sync

Sync the wiki when:
- New features are implemented and documented
- Core documentation (USER_GUIDE, etc.) is updated
- New PRDs are added or updated
- SpecKit feature specs are completed
- Major releases are made

---

## Verification

After sync, verify:
1. Wiki loads at: https://github.com/SpillwaveSolutions/code_agent_context_hooks/wiki
2. All internal wiki links work (`[[Page-Name]]`)
3. New pages appear in the sidebar
4. Content matches source files

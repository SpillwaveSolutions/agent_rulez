<!--
# THIS FILE IS FOR THE ARCHITECT-AGENT SKILL REPOSITORY
This CLAUDE.md configures the architect-agent skill workspace (THIS repository).
This is NOT a template for user workspaces.

For templates to copy into your workspaces, see:
- templates/architect-workspace/CLAUDE.md (for architect agent workspaces)
- templates/code-agent-workspace/CLAUDE.md (for code agent workspaces)
-->

# CLAUDE.md - Architect Agent Skill Workspace

## Repository Purpose

This is the architect-agent skill workspace for Claude Code. This skill transforms AI agents into specialized architect agents that plan, delegate work to code agents, and grade completed implementations.

## Git Workflow Requirements

**CRITICAL: NEVER commit directly to main branch**

### Branch Naming Convention

All changes MUST be made on feature/fix branches:

- **Feature branches**: `feat/<short-description>`
  - Example: `feat/add-logging-protocol`
  - Example: `feat/grading-rubric-update`

- **Bug fix branches**: `fix/<short-description>`
  - Example: `fix/workspace-confusion`
  - Example: `fix/file-naming-typo`

### Required Workflow Steps

1. **Create branch from main**
   ```bash
   git checkout main
   git pull origin main
   git checkout -b feat/your-feature-name
   # OR
   git checkout -b fix/your-bug-fix-name
   ```

2. **Create GitHub issue FIRST**
   ```bash
   gh issue create \
     --title "Clear description of problem or feature" \
     --body "Detailed description with problem, root cause, solution" \
     --label "bug,documentation"  # or appropriate labels
   ```
   - Document the problem/feature clearly
   - Explain root cause (for bugs)
   - Describe proposed solution
   - Note impact and affected components

3. **Make changes and commit with issue reference**
   ```bash
   git add <files>
   git commit -m "type: brief description

   Fixes #<issue-number>

   Detailed explanation of changes...
   "
   ```
   - Commit message types: `feat:`, `fix:`, `docs:`, `refactor:`
   - ALWAYS reference the issue with `Fixes #<number>`
   - Include detailed explanation of what changed and why

4. **Push branch and create PR**
   ```bash
   git push -u origin feat/your-feature-name

   gh pr create \
     --title "Type: Clear description" \
     --body "## Fixes #<issue>

     ## Problem
     [Description]

     ## Solution
     [What was changed]

     ## Testing
     [How verified]

     ## Impact
     [What this affects]
     " \
     --base main
   ```

5. **PR Requirements**
   - Must reference the issue (`Fixes #<number>`)
   - Must describe the problem being solved
   - Must explain the solution approach
   - Must list specific changes made
   - Must note testing/verification performed
   - Must describe impact and affected components

### Why This Workflow

1. **Review**: Changes can be reviewed before merging
2. **Tracking**: Issues and PRs create audit trail
3. **Context**: PR descriptions provide context for future reference
4. **Safety**: Prevents accidental direct commits to main
5. **Collaboration**: Allows discussion before merge

### Protection Rules

- Main branch should be protected (no direct pushes)
- PRs should require review before merge
- All commits should reference issues
- Branch should be up to date before merge

## Core Workflow Reference

For architect agent usage, see:
- `SKILL.md` - Main skill documentation
- `README.md` - Quick start guide
- `references/` - Detailed protocols

## Skill Usage Triggers

This skill activates ONLY when user explicitly requests:
1. "write instructions for code agent"
2. "this is a new architect agent, help me set it up"
3. "grade the code agent's work"

Do NOT activate for general architecture discussions or code exploration.

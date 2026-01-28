# Contributing to Architect Agent Skill

Thank you for your interest in contributing! This guide explains the required workflow for all changes to this repository.

---

## Table of Contents

- [Git Workflow](#git-workflow)
- [Branch Naming](#branch-naming)
- [Issue Creation](#issue-creation)
- [Commit Messages](#commit-messages)
- [Pull Requests](#pull-requests)
- [Why This Workflow](#why-this-workflow)
- [Examples](#examples)

---

## Git Workflow

**CRITICAL: NEVER commit directly to main branch**

All changes MUST follow this workflow:

```
1. Create Branch          →  git checkout -b feat/description
2. Create Issue           →  gh issue create (document problem/feature)
3. Make Changes           →  Edit files
4. Commit with Reference  →  git commit -m "type: description\n\nFixes #X"
5. Push Branch            →  git push -u origin feat/description
6. Create PR              →  gh pr create (reference issue, explain changes)
7. Review & Merge         →  After approval
```

---

## Branch Naming

Use descriptive branch names with appropriate prefixes:

### Feature Branches
```bash
feat/<short-description>
```

**Examples:**
- `feat/add-logging-protocol`
- `feat/grading-rubric-update`
- `feat/opencode-integration`

### Bug Fix Branches
```bash
fix/<short-description>
```

**Examples:**
- `fix/workspace-confusion`
- `fix/file-naming-typo`
- `fix/broken-cross-reference`

### Documentation Branches
```bash
docs/<short-description>
```

**Examples:**
- `docs/update-readme`
- `docs/add-contributing-guide`

### Refactoring Branches
```bash
refactor/<short-description>
```

**Examples:**
- `refactor/reorganize-references`
- `refactor/simplify-skill-structure`

---

## Issue Creation

**Create an issue BEFORE making changes.**

### Issue Template

```bash
gh issue create \
  --title "Clear, actionable title" \
  --body "## Problem
[What's wrong or what's needed]

## Root Cause (for bugs)
[Why this issue exists]

## Proposed Solution
[How to fix it]

## Impact
[What this affects]

## Related
[Links to related issues, PRs, or docs]" \
  --label "bug" \  # or "enhancement", "documentation", etc.
```

### Good Issue Titles

✅ **Good:**
- "INSTALLATION.md should be in references/ directory"
- "Add upgrade trigger to SKILL.md"
- "Reference Documents section missing 17 files"

❌ **Bad:**
- "Fix stuff"
- "Update docs"
- "Improvements"

### Issue Body Should Include

1. **Problem**: Clear description of what's wrong or needed
2. **Root Cause**: Why this issue exists (for bugs)
3. **Proposed Solution**: How you plan to fix it
4. **Impact**: What components/workflows this affects
5. **Related**: Links to related issues or docs

---

## Commit Messages

### Format

```
type: brief summary (50 chars max)

Fixes #<issue-number>

Detailed explanation:
- Change 1
- Change 2
- Why these changes were made

Additional context...
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `refactor`: Code refactoring (no functionality change)
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples

**Feature:**
```bash
git commit -m "feat: add upgrade trigger to SKILL.md

Fixes #14

Added Trigger 6 for upgrade workflow:
- User says 'upgrade to latest architect agent protocol'
- References upgrade.md in references/
- Includes prerequisite check for version detection"
```

**Bug Fix:**
```bash
git commit -m "fix: move INSTALLATION.md to references/

Fixes #12

Moved INSTALLATION.md to references/installation.md:
- Preserves git history with git mv
- Updates all cross-references
- Aligns with file organization standards from SPEC.md"
```

**Documentation:**
```bash
git commit -m "docs: create references/README.md index

Fixes #15

Created comprehensive reference index:
- All 29 files cataloged by category
- Quick navigation section
- When-to-use guidance for each file
- File organization diagram"
```

### Rules

1. **Always include** `Fixes #<issue-number>`
2. **Use imperative mood**: "add feature" not "added feature"
3. **Brief summary**: 50 characters max
4. **Detailed body**: Explain what and why, not how
5. **Reference related issues**: If fixing multiple, list all

---

## Pull Requests

### Creating a PR

```bash
gh pr create \
  --title "Type: Clear description of change" \
  --body "## Fixes #<issue-number>

## Problem
[Brief description of what was wrong]

## Root Cause
[Why the issue existed - for bug fixes]

## Solution
[What was changed to fix it]

### Key Changes
1. Change 1 - Description
2. Change 2 - Description
3. Change 3 - Description

## Testing
[How the fix was verified]

## Impact
- **Prevents**: [What problems this prevents]
- **Improves**: [What this improves]
- **Affects**: [What components are affected]

## Files Changed
- \`file1.md\`: Description of changes (+X lines)
- \`file2.md\`: Description of changes (+X lines)" \
  --base main
```

### PR Requirements

**MUST include:**
- Issue reference (`Fixes #<number>`)
- Problem description
- Root cause (for bugs)
- Solution explanation
- List of specific changes
- Testing/verification notes
- Impact assessment
- Files changed summary

**MUST NOT:**
- Skip issue reference
- Have incomplete description
- Merge without review (if collaboration)
- Push directly to main

---

## Why This Workflow

### 1. Review and Quality Control
- Changes are reviewed before merging
- Prevents mistakes from reaching main
- Allows discussion and improvement

### 2. Audit Trail and History
- Issues document why changes were needed
- PRs document what was changed
- Commits link to both
- Full context preserved forever

### 3. Tracking and Project Management
- Issues track work items
- PRs track implementations
- Easy to see what's in progress
- Easy to see what was completed

### 4. Safety and Reversibility
- Easy to revert if needed
- Changes are isolated
- Main branch stays stable
- Less risk of breaking changes

### 5. Collaboration
- Team can see what's happening
- Discussion happens in PR comments
- Multiple reviewers can provide input
- Knowledge sharing through documentation

---

## Examples

### Example 1: Feature Addition

```bash
# 1. Create branch
git checkout main
git pull origin main
git checkout -b feat/add-spec-document

# 2. Create issue
gh issue create \
  --title "Add SPEC.md project specification document" \
  --body "## Problem
No central specification document defining project scope, architecture, and standards.

## Proposed Solution
Create SPEC.md covering:
- Purpose and scope
- Architecture (3-level progressive disclosure)
- Quality standards
- File organization standards

## Impact
Provides north star for all future development decisions." \
  --label "enhancement,documentation"

# Returns: Issue #10

# 3. Make changes
# ... create SPEC.md ...

# 4. Commit
git add SPEC.md
git commit -m "feat: add SPEC.md project specification

Fixes #10

Created comprehensive project specification:
- Purpose and vision
- Scope and boundaries
- Three-level architecture
- Key workflows
- Quality standards
- Decision framework"

# 5. Push
git push -u origin feat/add-spec-document

# 6. Create PR
gh pr create \
  --title "Feat: Add SPEC.md project specification" \
  --body "## Fixes #10

## Problem
The skill lacked a central specification document, making it difficult to maintain consistency and make architectural decisions.

## Solution
Created SPEC.md covering all aspects of the project:

### Key Sections
1. Purpose & Vision - What the skill does
2. Scope & Boundaries - What's in/out of scope
3. Architecture - 3-level progressive disclosure
4. Key Workflows - Core operations
5. Quality Standards - What good looks like
6. Decision Framework - How to evaluate changes

## Testing
Validated structure against existing documentation and references.

## Impact
- **Provides**: North star for future development
- **Improves**: Consistency in updates
- **Affects**: All future feature additions and changes

## Files Changed
- \`SPEC.md\`: New file (+500 lines)" \
  --base main

# Returns: PR #11
```

### Example 2: Bug Fix

```bash
# 1. Create branch
git checkout main
git pull origin main
git checkout -b fix/installation-location

# 2. Create issue
gh issue create \
  --title "INSTALLATION.md should be in references/ directory" \
  --body "## Problem
INSTALLATION.md is in root but should be in references/ for consistency.

## Root Cause
File was created in root initially without considering organization standards.

## Proposed Solution
Move INSTALLATION.md to references/installation.md using git mv.

## Impact
Aligns with file organization standards from SPEC.md." \
  --label "bug,refactor"

# Returns: Issue #12

# 3. Make changes
git mv INSTALLATION.md references/installation.md

# 4. Commit
git commit -m "fix: move INSTALLATION.md to references/

Fixes #12

Moved to align with file organization standards:
- Preserves git history
- Consistent with other reference docs
- Matches SPEC.md file organization"

# 5. Push
git push -u origin fix/installation-location

# 6. Create PR
gh pr create \
  --title "Fix: Move INSTALLATION.md to references/" \
  --body "## Fixes #12

## Problem
INSTALLATION.md was in root directory instead of references/.

## Root Cause
Initial file creation didn't follow organization standards.

## Solution
Moved file to correct location:

### Key Changes
1. `git mv INSTALLATION.md references/installation.md` - Preserves history
2. Updated all cross-references in README.md
3. Updated SKILL.md references

## Testing
Verified all links still work after move.

## Impact
- **Prevents**: Confusion about file organization
- **Improves**: Consistency with SPEC.md standards
- **Affects**: Documentation structure

## Files Changed
- \`INSTALLATION.md\` → \`references/installation.md\` (renamed)" \
  --base main
```

---

## Code of Conduct

- Be respectful and constructive
- Focus on the work, not the person
- Provide specific, actionable feedback
- Assume good intent
- Follow the workflow - no exceptions

---

## Questions?

If you're unsure about any aspect of the workflow:

1. Check CLAUDE.md for git workflow overview
2. Check AGENTS.md for detailed requirements
3. Check SPEC.md for architectural guidance
4. Create an issue to ask for clarification

---

## Summary Checklist

Before submitting a PR:

- [ ] Created branch with appropriate prefix
- [ ] Created issue documenting problem/feature
- [ ] Made focused changes (one issue per branch)
- [ ] Committed with issue reference (`Fixes #X`)
- [ ] Pushed branch to origin
- [ ] Created PR with complete description
- [ ] PR references issue
- [ ] PR explains problem, solution, and impact
- [ ] All cross-references updated
- [ ] Ready for review

---

**Last Updated:** 2025-01-21
**Version:** 4.0

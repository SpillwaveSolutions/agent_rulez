# Agent Specialization Protocol

## Overview

**Learned from TKT-121 Phase 8:** Improper agent usage led to incomplete work and grade deductions.

### Core Principle: Right Agent for the Job

**Each agent has a specific purpose. Do NOT use agents outside their specialization.**

Misusing agents results in:
- Incomplete work (documentation agents skipped)
- Wrong tool for the job (qa-enforcer for doc creation)
- Grade deductions (-1 to -2 points per violation)

## Agent Categories and Specializations

### Validation Agents (Technical Quality)

#### `qa-enforcer` - Technical Validation ONLY

**Purpose:** Test coverage, completeness, success criteria verification

**Use for:**
- Final validation before marking work complete
- Verifying all success criteria met
- Checking test coverage >= 60%
- Confirming all requirements completed

**Do NOT use for:**
- Creating documentation
- Writing change logs
- Syncing documentation files
- Generating reports

**Make MANDATORY:** Always required before completion

**Example proper usage:**
```markdown
## Final Validation

Before marking complete:
1. Create completion documentation manually
2. Use change-explainer to analyze changes
3. Use docs-sync-editor to update README.md
4. **Use qa-enforcer to validate all success criteria**
```

**Example improper usage (❌ WRONG):**
```markdown
## Final Validation

Use qa-enforcer to:
- Create the migration summary ❌ (qa-enforcer doesn't create docs)
- Update README.md ❌ (docs-sync-editor does this)
- Validate completion ✅ (correct use)
```

---

### Documentation Agents (Content Creation)

#### `change-explainer` - Change Documentation

**Purpose:** Analyze and document code changes, explain modifications

**Use for:**
- Creating change logs after significant work
- Explaining what changed and why
- Documenting modifications for team review
- Generating change summaries

**Make MANDATORY:** After creating significant documentation files

**Example use case:**
```markdown
After creating TKT-121-MIGRATION-COMPLETE.md:
1. ✅ Create the document manually
2. ✅ Use change-explainer to analyze all changes in the document
3. ✅ Use change-explainer output to improve the document
```

**When to use:**
- After major feature completion
- After migration or refactoring
- After creating completion documentation
- When summarizing a phase's work

#### `docs-sync-editor` - Documentation Synchronization

**Purpose:** Keep documentation in sync with code changes

**Use for:**
- Updating README.md after code changes
- Syncing CLAUDE.md with new patterns
- Updating API documentation
- Maintaining consistency across docs

**Make MANDATORY:** When migration or major changes affect project documentation

**Example use case:**
```markdown
After completing GCP project migration:
1. ✅ Create migration documentation
2. ✅ Use docs-sync-editor to update README.md with new project references
3. ✅ Use docs-sync-editor to update CLAUDE.md with new patterns
```

**When to use:**
- After significant code changes
- After infrastructure changes
- After migration completion
- When README/CLAUDE.md need updates

#### `mermaid-architect` - Architecture Diagrams

**Purpose:** Create visual architecture documentation with Mermaid diagrams

**Use for:**
- System architecture diagrams
- Data flow visualizations
- Deployment diagrams
- Sequence diagrams

**Make OPTIONAL BUT RECOMMENDED:** For completion phases, migration documentation

**Example use case:**
```markdown
For final migration documentation:
1. ✅ Create text documentation
2. ✅ Use mermaid-architect to generate architecture diagrams
3. ✅ Include diagrams in completion docs
```

#### `grammar-style-editor` - Writing Improvement

**Purpose:** Improve grammar, clarity, engagement of written content

**Use for:**
- Polishing user-facing documentation
- Improving README.md readability
- Enhancing PR descriptions
- Refining technical writing

**Make OPTIONAL:** For final polish of important docs

---

### Development Agents (Implementation)

#### `python-expert-engineer` - Python Development

**Purpose:** Python code implementation and architectural review

**Use for:**
- Complex Python features
- Refactoring Python code
- Design pattern implementation
- Python-specific best practices

**Example use case:**
```markdown
For implementing async orchestration:
1. Use python-expert-engineer for architectural guidance
2. Implement based on recommendations
3. Use qa-enforcer for final validation
```

#### `root-cause-debugger` - Debugging

**Purpose:** Systematic debugging and root cause analysis

**Use for:**
- When tests fail unexpectedly
- When bugs occur during development
- When encountering unexpected behavior
- When systematic investigation needed

**Example use case:**
```markdown
When encountering test failures:
1. Read error message
2. Use root-cause-debugger to investigate
3. Apply recommended fixes
4. Re-run tests
```

#### `code-quality-reviewer` - Code Review

**Purpose:** Code quality, security, and best practices review

**Use for:**
- After completing significant code changes
- Before creating PR
- When reviewing refactored code
- Security audits

---

## Instructions Templates for Different Phases

### Early/Middle Phases (Feature Development)

```markdown
## Required Agents

**MANDATORY Agents:**
- `qa-enforcer` - Run before marking phase complete

**RECOMMENDED Agents:**
- `root-cause-debugger` - If tests fail or bugs occur
- `python-expert-engineer` - For complex Python implementation

**Completion Checklist:**
- [ ] Feature implemented
- [ ] Tests passing
- [ ] qa-enforcer validation complete
```

### Milestone Phases (Integration Points)

```markdown
## Required Agents

**MANDATORY Agents:**
- `qa-enforcer` - Final validation
- `change-explainer` - Document changes made in this milestone

**RECOMMENDED Agents:**
- `docs-sync-editor` - If README or API docs affected
- `mermaid-architect` - For architecture changes

**Completion Checklist:**
- [ ] Milestone features complete
- [ ] change-explainer run on changes
- [ ] Documentation updated (if applicable)
- [ ] qa-enforcer validation complete
```

### Final Phases (Completion/Documentation)

```markdown
## Required Agents

**MANDATORY Agents:**
- `qa-enforcer` - Technical validation ONLY (not for doc creation)
- `change-explainer` - After creating completion docs
- `docs-sync-editor` - Update README.md, CLAUDE.md with project state

**RECOMMENDED Agents:**
- `mermaid-architect` - Create architecture diagrams for completion docs
- `grammar-style-editor` - Polish user-facing documentation

**YOU CANNOT mark this phase complete until:**
- [ ] Completion documentation created manually
- [ ] change-explainer has analyzed all documentation
- [ ] docs-sync-editor has updated README.md and CLAUDE.md
- [ ] qa-enforcer has validated all success criteria

**Completion Checklist:**
- [ ] All code complete
- [ ] Completion docs created
- [ ] change-explainer run
- [ ] README.md updated via docs-sync-editor
- [ ] CLAUDE.md updated via docs-sync-editor
- [ ] Architecture diagrams created (if applicable)
- [ ] User-facing docs polished (if applicable)
- [ ] qa-enforcer final validation complete
```

## Common Mistakes to Avoid

### Mistake #1: Using qa-enforcer for Documentation

**❌ WRONG:**
```markdown
Run qa-enforcer to create the migration summary and validation report
```

**Problem:** qa-enforcer validates technical quality, doesn't create documentation

**✅ RIGHT:**
```markdown
1. Create migration summary manually
2. Use change-explainer to analyze and document changes
3. Use docs-sync-editor to update README.md and CLAUDE.md
4. Use qa-enforcer to validate all work is complete
```

**Grade Impact:** -2 points for using qa-enforcer incorrectly

---

### Mistake #2: Making Documentation Agents "Optional"

**❌ WRONG:**
```markdown
**Recommended Agents (Optional but Helpful):**
- change-explainer - Create change documentation
- docs-sync-editor - Keep docs in sync with code changes
```

**Problem:** Code agent may skip them, leading to incomplete documentation

**✅ RIGHT:**
```markdown
**MANDATORY Agents:**
- qa-enforcer - Technical validation
- change-explainer - MUST use after creating completion docs
- docs-sync-editor - MUST use to update README.md and CLAUDE.md

YOU CANNOT mark this phase complete until:
- change-explainer has analyzed all documentation
- docs-sync-editor has updated README.md and CLAUDE.md
- qa-enforcer has validated all success criteria
```

**Grade Impact:** -2 points for making mandatory agents optional

---

### Mistake #3: Not Using change-explainer After Doc Creation

**❌ WRONG:**
```markdown
1. Create MIGRATION-COMPLETE.md
2. Use qa-enforcer to validate ❌
```

**Problem:** Missing analysis and improvement of documentation

**✅ RIGHT:**
```markdown
1. Create MIGRATION-COMPLETE.md
2. Use change-explainer to analyze changes
3. Use docs-sync-editor to update README.md
4. Use qa-enforcer to validate completion
```

**Grade Impact:** -1 point for skipping change-explainer

---

### Mistake #4: Not Using docs-sync-editor for README/CLAUDE.md

**❌ WRONG:**
```markdown
1. Complete migration
2. Create completion docs
3. Use qa-enforcer to validate ✅
4. Mark complete ❌ (README.md not updated)
```

**Problem:** Project documentation out of sync with reality

**✅ RIGHT:**
```markdown
1. Complete migration
2. Create completion docs
3. Use change-explainer to analyze
4. Use docs-sync-editor to update README.md and CLAUDE.md
5. Use qa-enforcer to validate
```

**Grade Impact:** -1 point for not updating core docs

## Success Criteria Must Include Agent Usage

When creating success criteria for phases, include agent usage:

```markdown
## Success Criteria (15 required)

**Implementation:**
- [ ] Feature X implemented
- [ ] Tests passing
- [ ] Coverage >= 60%

**Documentation:**
- [ ] Migration summary created
- [ ] change-explainer run on completion docs
- [ ] README.md updated via docs-sync-editor
- [ ] CLAUDE.md updated via docs-sync-editor

**Validation:**
- [ ] qa-enforcer validation complete
- [ ] All tests passing
- [ ] Coverage >= 60%
```

## Grading Impact

**Automatic grade deductions for improper agent usage:**

| Violation | Deduction |
|-----------|-----------|
| Using qa-enforcer for documentation creation | -2 points |
| Not using change-explainer after creating docs | -1 point |
| Not using docs-sync-editor when README/CLAUDE.md need updates | -1 point |
| Making mandatory agents "optional" | -2 points |
| Not using qa-enforcer before completion | -5 points |

## Reference Case Study: TKT-121 Phase 8

### What Happened

**Instructions said:**
```markdown
**Recommended Agents (Optional but Helpful):**
- change-explainer - Create change documentation
- docs-sync-editor - Keep docs in sync with code changes
```

**Code agent did:**
- ✅ Used qa-enforcer for validation (correct)
- ✅ Created docs manually (acceptable but less optimal)
- ❌ Did NOT use docs-sync-editor for README.md/CLAUDE.md updates

**Result:**
- README.md not updated with new GCP project references
- CLAUDE.md not updated with new patterns
- Grade deduction: -2 points

**Final Grade:** B+ (88%) instead of A (90%+)

### What Should Have Happened

**Instructions should have said:**
```markdown
**MANDATORY Agents:**
- qa-enforcer - Technical validation
- change-explainer - MUST use after creating completion docs
- docs-sync-editor - MUST use to update README.md and CLAUDE.md

YOU CANNOT mark this phase complete until:
- change-explainer has analyzed all documentation
- docs-sync-editor has updated README.md and CLAUDE.md
- qa-enforcer has validated all success criteria
```

**Code agent would have:**
- ✅ Used change-explainer after doc creation
- ✅ Used docs-sync-editor for README/CLAUDE.md
- ✅ Used qa-enforcer for validation
- ✅ All files updated

**Result:** A (95%) grade - all requirements met

## Quick Reference

### When to Use Each Agent

| Agent | When | Phase Type | Mandatory? |
|-------|------|-----------|-----------|
| qa-enforcer | Final validation | All | ✅ Yes |
| change-explainer | After doc creation | Milestone, Final | ✅ Final only |
| docs-sync-editor | When README/CLAUDE.md affected | Milestone, Final | ✅ Final only |
| mermaid-architect | Architecture changes | Final | Recommended |
| grammar-style-editor | Polish docs | Final | Optional |
| python-expert-engineer | Complex Python | Early, Middle | Recommended |
| root-cause-debugger | Bugs/test failures | Any | As needed |
| code-quality-reviewer | After major changes | Milestone | Recommended |

### Agent Usage Checklist for Instructions

When creating instructions, ask yourself:

- [ ] Is qa-enforcer marked MANDATORY for final validation?
- [ ] Is change-explainer MANDATORY if creating significant docs?
- [ ] Is docs-sync-editor MANDATORY if README/CLAUDE.md affected?
- [ ] Are doc agents NOT being asked to do technical validation?
- [ ] Is qa-enforcer NOT being asked to create documentation?
- [ ] Are success criteria explicit about which agents to use?

### Red Flags in Instructions

Watch for these patterns that lead to grade deductions:

🚩 "Use qa-enforcer to create..." (qa-enforcer doesn't create)
🚩 "Recommended (Optional)" for doc agents in final phases (should be mandatory)
🚩 No mention of change-explainer after doc creation (will be skipped)
🚩 No mention of docs-sync-editor when README affected (docs won't sync)
🚩 Success criteria missing agent usage requirements (agents may be skipped)

## Key Takeaways

1. **qa-enforcer = validation only** - Never for creation
2. **change-explainer = after doc creation** - Mandatory in final phases
3. **docs-sync-editor = keep docs in sync** - Mandatory when README/CLAUDE.md affected
4. **Make it explicit** - "Recommended" = optional = may be skipped
5. **Success criteria should list agents** - Makes requirements crystal clear
6. **Wrong agent = wrong results** - Each agent has specific expertise

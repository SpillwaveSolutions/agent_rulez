# Architect Agent Guides

This directory contains workflow guides loaded on-demand based on user intent.

## Directory Structure

```
guides/
├── README.md                    # This file
└── workflows/                   # Step-by-step workflow guides
    ├── create-instructions.md   # Creating delegation instructions
    ├── grade-work.md           # Grading code agent work
    ├── send-instructions.md    # Sending instructions to code agent
    └── initialize-workspace.md # Setting up new workspace
```

## Usage

Guides are loaded by the skill's decision tree based on user intent:

| User Intent | Guide Loaded |
|-------------|--------------|
| "write instructions" | `workflows/create-instructions.md` |
| "grade the work" | `workflows/grade-work.md` |
| "send instructions" | `workflows/send-instructions.md` |
| "initialize workspace" | `workflows/initialize-workspace.md` |

## Adding New Guides

1. Create markdown file in appropriate subdirectory
2. Include trigger phrase in header
3. Add prerequisite checks
4. Document step-by-step workflow
5. Include quick checklist
6. Update SKILL.md intent classification table

## Guide Format

Each guide should follow this structure:

```markdown
# Workflow: [Name]

**Trigger:** User says "[trigger phrase]"

## Prerequisites
[What must be true before starting]

## Workflow Steps
### 1. First Step
[Details]

### 2. Second Step
[Details]

## Quick Checklist
- [ ] Item 1
- [ ] Item 2
```
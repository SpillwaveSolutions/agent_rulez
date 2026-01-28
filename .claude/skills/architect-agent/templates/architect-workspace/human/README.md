# Human Summaries Directory

This directory contains human-readable summaries of instruction files (10-25 bullet points) for quick review.

## File Naming Convention

```
human-YYYYMMDD_HHMMSS-brief_description.md
```

**Must match corresponding instruction file timestamp.**

**Example:** `human-20251120_143045-implement_api_refactor.md`

## Summary Format

Each human summary should contain **10-25 bullet points** covering:

### Main Objectives (2-4 bullets)
- Primary goal 1
- Primary goal 2

### Key Requirements (3-6 bullets)
- Technical requirement 1
- Technical requirement 2

### Critical Constraints (2-4 bullets)
- Constraint 1
- Constraint 2

### Success Criteria (2-4 bullets)
- Verification method 1
- Verification method 2

### Testing Requirements (2-4 bullets)
- Test 1
- Test 2

## Purpose

- **Quick Review** - Scan summary before sending instructions
- **User Communication** - Show user what code agent will do
- **Context Tracking** - Maintain history of what was requested

## Workflow

1. After creating instruction file, create matching human summary
2. When using `/project.send`, display this summary to user
3. Summary shows user exactly what code agent will execute

## See Also

- `instructions/` directory - Full detailed instruction files
- `/project.send` command - Uses these summaries for user communication

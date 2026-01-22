# Instructions Directory

This directory contains instruction files created by the architect agent for delegation to the code agent.

## File Naming Convention

```
instruct-YYYYMMDD_HHMMSS-brief_description.md
```

**Example:** `instruct-20251120_143045-implement_api_refactor.md`

## Instruction File Structure

Each instruction file should include:

1. **Title** - Clear, concise description of task
2. **Context** - Background and relevant information
3. **Objectives** - What needs to be accomplished
4. **Requirements** - Specific technical requirements
5. **Constraints** - Limitations and gotchas
6. **Success Criteria** - How to verify completion
7. **Testing** - Required tests and validation
8. **References** - Links to relevant documentation

## Workflow

1. Architect creates instruction file HERE in this directory
2. When ready to send, architect copies to code agent's `debugging/instructions/current_instructions.md`
3. Code agent reads from `current_instructions.md` and executes
4. Instruction file remains here for record-keeping

## See Also

- `human/` directory - Human-readable summaries (10-25 bullets)
- `grades/` directory - Grading reports for completed work

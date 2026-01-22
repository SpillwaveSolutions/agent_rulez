# Human Instruction Examples

This directory contains example human instructions demonstrating the difference between vague summaries (BAD) and executable documentation (GOOD).

## Files

| File | Purpose |
|------|---------|
| `GOOD-database-migration.md` | Complete example of database schema migration |
| `GOOD-parallel-worktrees.md` | Complete example of git worktree setup |
| `BAD-vague-summary.md` | Anti-pattern showing what NOT to do |

## Key Principles

Human instructions must be **executable documentation**, not summaries. Every instruction file should include:

1. **Copy-pasteable commands** - Exact bash commands, not descriptions
2. **Expected output** - What success looks like
3. **Verification steps** - How to confirm each step worked
4. **Why explanations** - Context for human decision-making
5. **Troubleshooting tables** - Common errors and solutions

## Using These Examples

When creating new human instructions:

1. Start from the template in `references/human_instruction_structure.md`
2. Use `GOOD-database-migration.md` or `GOOD-parallel-worktrees.md` as reference
3. Review `BAD-vague-summary.md` to avoid common mistakes

## Template Quick Reference

```markdown
## Step N: [Step Name]

### Why This Step
[1-2 sentences explaining purpose]

### Commands
```bash
exact-command --with --flags
```

### Expected Output
```
What the user should see
```

### Verification
```bash
how-to-verify-success
```

### If This Fails

| Error | Cause | Solution |
|-------|-------|----------|
| error-message | root-cause | fix-command |
```

## Checklist Before Publishing

- [ ] Can a new team member follow this without asking questions?
- [ ] Are all commands copy-pasteable?
- [ ] Is expected output shown for every command?
- [ ] Are at least 2-3 common errors documented per step?
- [ ] Is there a rollback procedure if something goes wrong?

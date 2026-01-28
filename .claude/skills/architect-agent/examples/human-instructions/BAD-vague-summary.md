# BAD EXAMPLE - Do Not Follow This Pattern

**This file demonstrates what NOT to do when creating human instructions.**

---

## The Problematic "Summary" Approach

Below is an example of human instructions that fail because they're summaries, not executable documentation:

```markdown
# Database Migration Summary

## Main Objectives
- Add last_login_at column to users table
- Create index for performance

## Key Requirements
- PostgreSQL database
- Admin access

## Execution Flow
1. Back up the database
2. Run the migration
3. Verify it worked

## Success Criteria
- Column exists
- Index exists
```

---

## Why This Fails

### Problem 1: No Copy-Pasteable Commands

The instruction says "Back up the database" but doesn't provide:

```bash
# What command? pg_dump? Which flags? Where does it save?
pg_dump $DATABASE_URL --table=users --data-only --file="backup.sql"
```

A human reading "back up the database" has to:
1. Figure out which backup tool to use
2. Look up the syntax
3. Decide what options to use
4. Hope they got it right

### Problem 2: No Expected Output

"Verify it worked" gives no indication of what success looks like:

- What query should they run?
- What result should they see?
- How do they know if something went wrong?

### Problem 3: No Troubleshooting

When (not if) something fails, the human has no guidance:

- What are common errors?
- What causes each error?
- How do they fix it?

### Problem 4: No Context/Why

Humans make better decisions when they understand purpose:

- Why are we adding this column?
- Why does the index need to be descending?
- Why do we use CONCURRENTLY?

### Problem 5: Missing Prerequisites

"PostgreSQL database" and "Admin access" are vague:

- Which PostgreSQL version?
- What specific privileges are needed?
- What environment variables should be set?

---

## The Fix

Compare with the GOOD example: `GOOD-database-migration.md`

Each step in the good example includes:
1. **Commands** - Exact bash commands to copy and run
2. **Expected Output** - What success looks like
3. **Verification** - How to confirm it worked
4. **Why This Step** - Context for decision-making
5. **If This Fails** - Troubleshooting table

---

## Quick Checklist: Is This Executable?

Ask yourself these questions:

| Question | Summary (BAD) | Executable (GOOD) |
|----------|---------------|-------------------|
| Can I copy-paste a command? | No | Yes |
| Do I know what success looks like? | No | Yes |
| If it fails, do I know what to try? | No | Yes |
| Do I understand why I'm doing this? | Vaguely | Yes |
| Can a new team member follow this? | No | Yes |

If you answer "No" to any of these, the instructions need more detail.

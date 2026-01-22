# Git Commit and PR Management

## ⚠️ CRITICAL: No AI Attribution

**NEVER include AI attribution in commits, PRs, tickets, or comments.**

**FORBIDDEN:**
- ❌ AI, AI agent, Claude, Claude Code, Anthropic
- ❌ "Generated with", "Assisted by"
- ❌ 🤖 emoji or AI-related emojis
- ❌ `Co-Authored-By: Claude <noreply@anthropic.com>`
- ❌ Any mention of automation tools

**WHY:** Professional client requirements - they use AI but don't want it publicly visible to stakeholders.

**GRADING:** Including AI attribution = -2 points (professionalism violation)

---

## Commit Messages

### Format

Combine recent instruction descriptions separated by commas:

```bash
git commit -m "implement contact api, add validation, update tests"
```

### Guidelines

- Use imperative mood ("add" not "added")
- Be concise but descriptive
- Reference multiple tasks if applicable
- No need to mention ticket ID (tracked separately)

## Pull Request Creation

### PR Message Structure

```markdown
# PR Title: [Ticket-ID]: Summary of changes

## Ticket
- TKT-123: Description
- Status: Complete / In Progress

## Changes Made
• instruction_description_1 - What was done
• instruction_description_2 - What was done
• instruction_description_3 - What was done

## Technical Decisions
- Decision 1 with rationale
- Decision 2 with rationale

## Testing
- Unit tests: X/X passing
- Integration tests: X/X passing
- Coverage: X%

## Related Logs
- log-YYYY_MM_DD-HH_MM-description_1.md
- log-YYYY_MM_DD-HH_MM-description_2.md
```

### PR Creation Process

1. **Summarize completed work:**
   - List all instruction descriptions as bullet points
   - Note ticket number and status
   - Highlight key technical decisions

2. **Instruct code agent to:**
   ```bash
   # Create branch if needed
   git checkout -b feat/TKT-123

   # Stage and commit changes
   git add -A
   git commit -m "descriptive message"

   # Push to remote
   git push origin feat/TKT-123

   # Create PR via CLI
   gh pr create --title "[TKT-123]: Description" --body-file pr-message.md
   ```

3. **Document PR in ticket:**
   - Add PR URL to ticket file
   - Update status in current_ticket.md
   - Note PR number and link

### Example PR Workflow

```bash
# After multiple instruction/grade cycles:

# 1. Simple commit
git add -A
git commit -m "fix zsh compatibility, update aws scripts, implement security"

# 2. Or detailed PR
gh pr create \
  --title "[TKT-123]: Fix authentication scripts and implement security" \
  --body-file pr-message.md \
  --base main \
  --head feat/TKT-123
```

## Instruct Code Agent Template

```markdown
## Git Commit Instructions

Create commit with message combining recent work:
- Description from instruction 1
- Description from instruction 2
- Description from instruction 3

Example commit message:
```
implement contact api, add validation tests, update documentation
```

## PR Creation Instructions

Create PR with:
- Title: [TICKET-ID]: Summary
- Body: Use template above
- Base: main
- Head: feat/TICKET-ID

Document PR URL in ticket file.
```

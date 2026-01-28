# Common Review Scenarios

Detailed workflows for specific review use cases.

## Scenario 1: Quick Review Request

**Trigger**: User provides PR URL and requests review.

**Workflow**:
1. Run `fetch_pr_data.py` to collect data
2. Read `SUMMARY.txt` and `metadata.json`
3. Scan `diff.patch` for obvious issues
4. Apply critical criteria (security, bugs, tests)
5. Create findings JSON with analysis
6. Run `generate_review_files.py` to create review files
7. Direct user to review `pr/review.md` and `pr/human.md`
8. Remind user to use `/show` to edit, then `/send` or `/send-decline`

## Scenario 2: Thorough Review with Inline Comments

**Trigger**: User requests comprehensive review with inline comments.

**Workflow**:
1. Run `fetch_pr_data.py` with cloning enabled
2. Read all collected files (metadata, diff, comments, commits)
3. Apply full `review_criteria.md` checklist
4. Identify critical issues, important issues, and nits
5. Create findings JSON with `inline_comments` array
6. Run `generate_review_files.py` to create all files
7. Direct user to:
   - Review `pr/review.md` for detailed analysis
   - Edit `pr/human.md` if needed
   - Check `pr/inline.md` for proposed comments
   - Use `/show` to open in VS Code
   - Use `/send` or `/send-decline` when ready
   - Optionally post inline comments from `pr/inline.md`

## Scenario 3: Security-Focused Review

**Trigger**: User requests security-specific review.

**Workflow**:
1. Fetch PR data
2. Focus on `review_criteria.md` Section 5 (Security)
3. Check for: SQL injection, XSS, CSRF, secrets exposure
4. Examine dependencies in metadata
5. Review authentication/authorization changes
6. Report security findings with severity ratings

## Scenario 4: Review with Related Tickets

**Trigger**: User requests review against linked JIRA/GitHub ticket.

**Workflow**:
1. Fetch PR data (captures ticket references)
2. Read `related_issues.json`
3. Compare PR changes against ticket requirements
4. Verify all acceptance criteria met
5. Note any missing functionality
6. Suggest additional tests if needed

## Scenario 5: Large PR Review (>400 lines)

**Trigger**: PR contains more than 400 lines of changes.

**Workflow**:
1. Suggest splitting into smaller PRs if feasible
2. Review in logical chunks by file or feature
3. Focus on architecture and design first
4. Document structural concerns before line-level issues
5. Prioritize security and correctness over style

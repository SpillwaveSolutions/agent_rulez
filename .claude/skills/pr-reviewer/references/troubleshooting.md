# Troubleshooting Guide

Common issues and solutions for the PR Reviewer skill.

## gh CLI Not Found

Install GitHub CLI: https://cli.github.com/

```bash
# macOS
brew install gh

# Linux
sudo apt install gh  # or yum, dnf, etc.

# Authenticate
gh auth login
```

## Permission Denied Errors

Check authentication:

```bash
gh auth status
gh auth refresh -s repo
```

## Invalid PR URL

Ensure URL format: `https://github.com/owner/repo/pull/NUMBER`

## Line Number Mismatch in Diff

Inline comment line numbers are **relative to the diff**, not absolute file positions.
Use `gh pr diff <number>` to see diff line numbers.

## Rate Limit Errors

```bash
# Check rate limit
gh api /rate_limit

# Authenticated users get higher limits
gh auth login
```

## Common Error Patterns

| Error | Cause | Solution |
|-------|-------|----------|
| 401 Unauthorized | Token expired | Run `gh auth refresh` |
| 403 Forbidden | Missing scope | Run `gh auth refresh -s repo` |
| 404 Not Found | Private repo access | Verify repo permissions |
| 422 Unprocessable | Invalid request | Check command arguments |

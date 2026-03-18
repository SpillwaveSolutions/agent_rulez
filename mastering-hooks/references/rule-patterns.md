---
last_modified: 2026-03-16
last_validated: 2026-03-16
---

# Hook Rule Patterns and Recipes

Common patterns for solving real-world problems with RuleZ.

## Table of Contents

1. [Context Injection Patterns](#context-injection-patterns)
2. [Security and Safety Patterns](#security-and-safety-patterns)
3. [Workflow Automation Patterns](#workflow-automation-patterns)
4. [Validation Patterns](#validation-patterns)
5. [Conditional Logic Patterns](#conditional-logic-patterns)
6. [Agent Lifecycle Patterns](#agent-lifecycle-patterns)
7. [Cross-Platform Patterns](#cross-platform-patterns)
8. [Optimization Patterns](#optimization-patterns)

---

## Context Injection Patterns

### Language-Specific Standards

Inject coding standards based on file type.

```yaml
# Python standards
- name: python-standards
  matchers:
    operations: [PreToolUse]
    tools: [Write, Edit]
    extensions: [.py, .pyi]
  actions:
    inject: .claude/context/python-standards.md

# TypeScript standards
- name: typescript-standards
  matchers:
    operations: [PreToolUse]
    tools: [Write, Edit]
    extensions: [.ts, .tsx]
  actions:
    inject: .claude/context/typescript-standards.md
```

### Directory-Based Context

Different context for different parts of the codebase.

```yaml
# API layer context
- name: api-context
  matchers:
    operations: [PreToolUse]
    tools: [Write, Edit]
    directories: [src/api/, src/routes/]
  actions:
    inject: .claude/context/api-guidelines.md

# Database layer context
- name: db-context
  matchers:
    operations: [PreToolUse]
    tools: [Write, Edit]
    directories: [src/models/, src/repositories/]
  actions:
    inject: .claude/context/database-patterns.md
```

### Dynamic Context from Commands

Generate context at runtime.

```yaml
# Include current git branch
- name: git-context
  matchers:
    operations: [SessionStart]
  actions:
    inject_command: |
      echo "## Current Branch"
      echo "Branch: $(git branch --show-current)"
      echo "Last commit: $(git log -1 --oneline)"

# Include dependency versions
- name: dependency-context
  matchers:
    operations: [SessionStart]
  actions:
    inject_command: |
      echo "## Dependencies"
      cat package.json | jq '{name, version, dependencies}'
```

### Project Overview on Session Start

Load comprehensive project context.

```yaml
- name: project-overview
  matchers:
    operations: [SessionStart]
  actions:
    inject: .claude/context/project-overview.md
```

**Example project-overview.md**:
```markdown
## Project: MyApp

### Tech Stack
- Backend: Python 3.11, FastAPI, SQLAlchemy
- Frontend: React 18, TypeScript
- Database: PostgreSQL 15

### Key Conventions
- Use Pydantic for all data models
- Tests in tests/ mirror src/ structure
- API versioning: /api/v1/

### Current Sprint
Focus: Performance optimization for search
```

---

## Security and Safety Patterns

### Block Dangerous Git Commands

```yaml
# Block force push
- name: block-force-push
  priority: 10
  matchers:
    operations: [PreToolUse]
    tools: [Bash]
    command_match: "git push.*(--force|-f)"
  actions:
    block: true
  governance:
    reason: "Force push is dangerous. Use --force-with-lease or get approval."

# Block main branch commits
- name: block-main-commit
  priority: 10
  matchers:
    operations: [PreToolUse]
    tools: [Bash]
    command_match: "git commit.*--(amend|fixup)"
  actions:
    run: |
      BRANCH=$(git branch --show-current)
      if [ "$BRANCH" = "main" ] || [ "$BRANCH" = "master" ]; then
        echo '{"continue": false, "reason": "Cannot amend commits on main/master branch"}'
      else
        echo '{"continue": true}'
      fi
```

### Secret Detection

```yaml
- name: detect-secrets
  priority: 5
  matchers:
    operations: [PreToolUse]
    tools: [Write, Edit]
    extensions: [.py, .js, .ts, .env, .yaml, .json]
  actions:
    run: .claude/validators/check-secrets.sh
```

**check-secrets.sh**:
```bash
#!/bin/bash
# Check for potential secrets in file content

CONTENT="$RULEZ_TOOL_INPUT_CONTENT"

# Patterns that might indicate secrets
PATTERNS=(
  "api[_-]?key\s*[:=]"
  "password\s*[:=]"
  "secret\s*[:=]"
  "token\s*[:=]"
  "-----BEGIN .* KEY-----"
  "aws_access_key_id"
  "aws_secret_access_key"
)

for pattern in "${PATTERNS[@]}"; do
  if echo "$CONTENT" | grep -qiE "$pattern"; then
    echo '{"continue": false, "reason": "Potential secret detected. Please use environment variables or a secrets manager."}'
    exit 0
  fi
done

echo '{"continue": true}'
```

### Prevent Destructive File Operations

```yaml
- name: block-rm-rf
  priority: 1
  matchers:
    operations: [PreToolUse]
    tools: [Bash]
    command_match: "rm\\s+(-rf|-fr|--recursive.*--force|--force.*--recursive)\\s+/"
  actions:
    block: true
  governance:
    reason: "Recursive force delete from root is blocked for safety."

- name: warn-rm-rf
  priority: 20
  matchers:
    operations: [PreToolUse]
    tools: [Bash]
    command_match: "rm\\s+(-rf|-fr)"
  actions:
    inject_inline: |
      **Warning**: Recursive delete detected. Please verify:
      - Target path is correct
      - No important files will be deleted
      - You have backups if needed
```

---

## Workflow Automation Patterns

### Pre-Commit Checks

```yaml
- name: pre-commit-lint
  matchers:
    operations: [PreToolUse]
    tools: [Bash]
    command_match: "git commit"
  actions:
    run: |
      # Run linting
      npm run lint 2>&1
      LINT_EXIT=$?

      if [ $LINT_EXIT -ne 0 ]; then
        echo '{"continue": false, "reason": "Linting failed. Please fix errors before committing."}'
      else
        echo '{"continue": true, "context": "All linting checks passed."}'
      fi
```

### Auto-Format on Save

```yaml
- name: format-python
  matchers:
    operations: [PostToolUse]
    tools: [Write]
    extensions: [.py]
  actions:
    run: |
      FILE="$RULEZ_TOOL_INPUT_PATH"
      black "$FILE" 2>&1
      isort "$FILE" 2>&1
      echo '{"continue": true, "context": "File formatted with black and isort."}'
```

### Test Reminder

```yaml
- name: test-reminder
  matchers:
    operations: [PostToolUse]
    tools: [Write, Edit]
    directories: [src/]
    extensions: [.py, .ts, .js]
  actions:
    inject_inline: |
      **Reminder**: You modified source code. Consider:
      - Running related tests: `pytest tests/`
      - Adding tests for new functionality
      - Checking test coverage
```

---

## Validation Patterns

### Require Commit Message Format

```yaml
- name: conventional-commits
  matchers:
    operations: [PreToolUse]
    tools: [Bash]
    command_match: 'git commit -m'
  actions:
    run: |
      # Extract commit message
      MSG=$(echo "$RULEZ_TOOL_INPUT_COMMAND" | grep -oP '(?<=-m\s?["\x27])[^"\x27]+')

      # Check conventional commit format
      if echo "$MSG" | grep -qE '^(feat|fix|docs|style|refactor|test|chore)(\(.+\))?: .+'; then
        echo '{"continue": true}'
      else
        echo '{"continue": false, "reason": "Commit message must follow Conventional Commits format: type(scope): description"}'
      fi
```

### Validate JSON/YAML Files

```yaml
- name: validate-json
  matchers:
    operations: [PreToolUse]
    tools: [Write]
    extensions: [.json]
  actions:
    run: |
      echo "$RULEZ_TOOL_INPUT_CONTENT" | jq . > /dev/null 2>&1
      if [ $? -eq 0 ]; then
        echo '{"continue": true}'
      else
        echo '{"continue": false, "reason": "Invalid JSON syntax. Please fix before saving."}'
      fi

- name: validate-yaml
  matchers:
    operations: [PreToolUse]
    tools: [Write]
    extensions: [.yaml, .yml]
  actions:
    run: |
      echo "$RULEZ_TOOL_INPUT_CONTENT" | python -c "import sys, yaml; yaml.safe_load(sys.stdin)" 2>&1
      if [ $? -eq 0 ]; then
        echo '{"continue": true}'
      else
        echo '{"continue": false, "reason": "Invalid YAML syntax. Please fix before saving."}'
      fi
```

---

## Conditional Logic Patterns

### Environment-Based Rules

```yaml
# Stricter rules in CI
- name: ci-strict-mode
  enabled_when: 'env_CI == "true"'
  matchers:
    operations: [PreToolUse]
    tools: [Bash]
  actions:
    inject_inline: |
      **CI Mode Active**: All commands are logged and audited.

# Development shortcuts
- name: dev-shortcuts
  enabled_when: 'env_CI != "true"'
  matchers:
    operations: [PreToolUse]
    tools: [Bash]
  actions:
    inject_inline: |
      Development mode: Using local configurations.
```

### Branch-Based Rules

```yaml
- name: production-branch-warning
  enabled_when: 'env_GIT_BRANCH == "main" || env_GIT_BRANCH == "master" || env_GIT_BRANCH == "production"'
  matchers:
    operations: [PreToolUse]
    tools: [Write, Edit, Bash]
  actions:
    inject_inline: |
      **Warning**: You are on a protected branch.
      All changes require code review.
```

### File Pattern Conditions

```yaml
# Extra care for test files
- name: test-file-guidance
  matchers:
    operations: [PreToolUse]
    tools: [Write, Edit]
    extensions: [.test.js, .test.ts, .spec.js, .spec.ts]
  actions:
    inject: .claude/context/testing-guidelines.md
```

---

## Agent Lifecycle Patterns

### Inject Policy Before Agent Tasks

Control what subagents/agents can do by injecting policy context.

```yaml
# Inject project conventions before any agent runs
- name: agent-policy
  description: Ensure agents follow project conventions
  matchers:
    operations: [BeforeAgent]
  actions:
    inject: .claude/context/agent-policy.md

# Log when agents complete
- name: agent-completed
  description: Track agent completion for audit
  matchers:
    operations: [AfterAgent]
  actions:
    run: |
      echo '{"continue": true, "context": "Agent task completed. Review changes before proceeding."}'
```

### Restrict Agent Scope

```yaml
# Block agents from modifying production configs
- name: agent-no-prod-config
  priority: 10
  matchers:
    operations: [BeforeAgent]
    prompt_match: "(?i)(production|prod)"
  actions:
    block: true
  governance:
    reason: "Agents cannot modify production configuration files."
```

---

## Cross-Platform Patterns

### Rules That Work Everywhere

These patterns use only events available on all platforms:

```yaml
# Works on Claude Code, Gemini, Copilot, and OpenCode
- name: universal-safety
  matchers:
    operations: [PreToolUse]
    tools: [Bash]
    command_match: "rm -rf /"
  actions:
    block: true
  governance:
    reason: "Dangerous operation blocked."

# Session context works on all platforms
- name: session-context
  matchers:
    operations: [SessionStart]
  actions:
    inject: .claude/context/project-overview.md
```

### Dual-Fire Aware Rules

On Gemini, `BeforeAgent` also fires `UserPromptSubmit`. Write rules knowing both may trigger:

```yaml
# This fires on Gemini's BeforeAgent AND as a dual-fire UserPromptSubmit
- name: prompt-policy
  matchers:
    operations: [UserPromptSubmit]
    prompt_match: "(?i)deploy"
  actions:
    inject_inline: |
      **Deploy detected**: Follow the deployment checklist.
```

---

## Optimization Patterns

### Consolidate Similar Rules

**Before** (3 rules):
```yaml
- name: python-lint
  matchers: { extensions: [.py] }
  actions: { inject: lint.md }

- name: js-lint
  matchers: { extensions: [.js] }
  actions: { inject: lint.md }

- name: ts-lint
  matchers: { extensions: [.ts] }
  actions: { inject: lint.md }
```

**After** (1 rule):
```yaml
- name: code-lint
  matchers:
    extensions: [.py, .js, .ts]
  actions:
    inject: .claude/context/lint-standards.md
```

### Priority-Based Short-Circuiting

Block rules first, context injection later.

```yaml
# Priority 1-10: Blockers (highest priority)
- name: security-block
  priority: 5
  actions: { block: true }

# Priority 50-70: Context injection
- name: code-standards
  priority: 50
  actions: { inject: standards.md }

# Priority 90-100: Logging/telemetry (lowest priority)
- name: action-log
  priority: 100
  actions: { run: log.sh }
```

### Lazy Evaluation with enabled_when

Avoid expensive checks when not needed.

```yaml
# Only run Python checks for Python files
- name: python-security
  matchers:
    tools: [Write]
    extensions: [.py]
  actions:
    run: python-security-check.sh
```

---

## Pattern Index

| Pattern | Use Case | Key Technique |
|---------|----------|---------------|
| Language standards | Consistent code style | extensions + inject |
| Directory context | Layer-specific guidance | directories + inject |
| Dynamic context | Runtime information | inject_command |
| Block dangerous | Safety guardrails | command_match + block |
| Secret detection | Security | run + validation script |
| Pre-commit | Quality gates | command_match + run |
| Format on save | Automation | PostToolUse + run |
| Conventional commits | Consistency | run + validation |
| CI-specific | Environment awareness | enabled_when |
| Branch protection | Workflow enforcement | enabled_when + regex |
| Agent policy | Agent governance | BeforeAgent + inject |
| Agent completion | Audit trail | AfterAgent + run |
| Cross-platform safety | Universal rules | PreToolUse (all platforms) |

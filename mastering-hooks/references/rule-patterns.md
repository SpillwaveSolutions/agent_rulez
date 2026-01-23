# Hook Rule Patterns and Recipes

Common patterns for solving real-world problems with CCH.

## Table of Contents

1. [Context Injection Patterns](#context-injection-patterns)
2. [Security and Safety Patterns](#security-and-safety-patterns)
3. [Workflow Automation Patterns](#workflow-automation-patterns)
4. [Validation Patterns](#validation-patterns)
5. [Conditional Logic Patterns](#conditional-logic-patterns)
6. [Optimization Patterns](#optimization-patterns)

---

## Context Injection Patterns

### Language-Specific Standards

Inject coding standards based on file type.

```yaml
# Python standards
- name: python-standards
  event: PreToolUse
  match:
    tools: [Write, Edit]
    extensions: [.py, .pyi]
  action:
    type: inject
    source: file
    path: .claude/context/python-standards.md

# TypeScript standards
- name: typescript-standards
  event: PreToolUse
  match:
    tools: [Write, Edit]
    extensions: [.ts, .tsx]
  action:
    type: inject
    source: file
    path: .claude/context/typescript-standards.md
```

### Directory-Based Context

Different context for different parts of the codebase.

```yaml
# API layer context
- name: api-context
  event: PreToolUse
  match:
    tools: [Write, Edit]
    directories: [src/api/, src/routes/]
  action:
    type: inject
    source: file
    path: .claude/context/api-guidelines.md

# Database layer context
- name: db-context
  event: PreToolUse
  match:
    tools: [Write, Edit]
    directories: [src/models/, src/repositories/]
  action:
    type: inject
    source: file
    path: .claude/context/database-patterns.md
```

### Dynamic Context from Commands

Generate context at runtime.

```yaml
# Include current git branch
- name: git-context
  event: SessionStart
  match: {}
  action:
    type: inject
    source: command
    command: |
      echo "## Current Branch"
      echo "Branch: $(git branch --show-current)"
      echo "Last commit: $(git log -1 --oneline)"

# Include dependency versions
- name: dependency-context
  event: SessionStart
  match: {}
  action:
    type: inject
    source: command
    command: |
      echo "## Dependencies"
      cat package.json | jq '{name, version, dependencies}'
```

### Project Overview on Session Start

Load comprehensive project context.

```yaml
- name: project-overview
  event: SessionStart
  match: {}
  action:
    type: inject
    source: file
    path: .claude/context/project-overview.md
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
  event: PreToolUse
  priority: 10
  match:
    tools: [Bash]
    command_match: "git push.*(--force|-f)"
  action:
    type: block
    reason: "Force push is dangerous. Use --force-with-lease or get approval."

# Block main branch commits
- name: block-main-commit
  event: PreToolUse
  priority: 10
  match:
    tools: [Bash]
    command_match: "git commit.*--(amend|fixup)"
  action:
    type: run
    command: |
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
  event: PreToolUse
  priority: 5
  match:
    tools: [Write, Edit]
    extensions: [.py, .js, .ts, .env, .yaml, .json]
  action:
    type: run
    command: .claude/validators/check-secrets.sh
    timeout: 10
```

**check-secrets.sh**:
```bash
#!/bin/bash
# Check for potential secrets in file content

CONTENT="$CCH_TOOL_INPUT_CONTENT"

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
  event: PreToolUse
  priority: 1
  match:
    tools: [Bash]
    command_match: "rm\\s+(-rf|-fr|--recursive.*--force|--force.*--recursive)\\s+/"
  action:
    type: block
    reason: "Recursive force delete from root is blocked for safety."

- name: warn-rm-rf
  event: PreToolUse
  priority: 20
  match:
    tools: [Bash]
    command_match: "rm\\s+(-rf|-fr)"
  action:
    type: inject
    source: inline
    content: |
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
  event: PreToolUse
  match:
    tools: [Bash]
    command_match: "git commit"
  action:
    type: run
    command: |
      # Run linting
      npm run lint 2>&1
      LINT_EXIT=$?
      
      if [ $LINT_EXIT -ne 0 ]; then
        echo '{"continue": false, "reason": "Linting failed. Please fix errors before committing."}'
      else
        echo '{"continue": true, "context": "All linting checks passed."}'
      fi
    timeout: 60
```

### Auto-Format on Save

```yaml
- name: format-python
  event: PostToolUse
  match:
    tools: [Write]
    extensions: [.py]
  action:
    type: run
    command: |
      FILE="$CCH_TOOL_INPUT_PATH"
      black "$FILE" 2>&1
      isort "$FILE" 2>&1
      echo '{"continue": true, "context": "File formatted with black and isort."}'
```

### Test Reminder

```yaml
- name: test-reminder
  event: PostToolUse
  match:
    tools: [Write, Edit]
    directories: [src/]
    extensions: [.py, .ts, .js]
  action:
    type: inject
    source: inline
    content: |
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
  event: PreToolUse
  match:
    tools: [Bash]
    command_match: 'git commit -m'
  action:
    type: run
    command: |
      # Extract commit message
      MSG=$(echo "$CCH_TOOL_INPUT_COMMAND" | grep -oP '(?<=-m\s?["\x27])[^"\x27]+')
      
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
  event: PreToolUse
  match:
    tools: [Write]
    extensions: [.json]
  action:
    type: run
    command: |
      echo "$CCH_TOOL_INPUT_CONTENT" | jq . > /dev/null 2>&1
      if [ $? -eq 0 ]; then
        echo '{"continue": true}'
      else
        echo '{"continue": false, "reason": "Invalid JSON syntax. Please fix before saving."}'
      fi

- name: validate-yaml
  event: PreToolUse
  match:
    tools: [Write]
    extensions: [.yaml, .yml]
  action:
    type: run
    command: |
      echo "$CCH_TOOL_INPUT_CONTENT" | python -c "import sys, yaml; yaml.safe_load(sys.stdin)" 2>&1
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
  event: PreToolUse
  match:
    tools: [Bash]
    enabled_when: "env.CI == 'true'"
  action:
    type: inject
    source: inline
    content: |
      **CI Mode Active**: All commands are logged and audited.

# Development shortcuts
- name: dev-shortcuts
  event: PreToolUse
  match:
    tools: [Bash]
    enabled_when: "env.CI != 'true'"
  action:
    type: inject
    source: inline
    content: |
      Development mode: Using local configurations.
```

### Branch-Based Rules

```yaml
- name: production-branch-warning
  event: PreToolUse
  match:
    tools: [Write, Edit, Bash]
    enabled_when: "env.GIT_BRANCH =~ '(main|master|production)'"
  action:
    type: inject
    source: inline
    content: |
      **Warning**: You are on a protected branch. 
      All changes require code review.
```

### File Pattern Conditions

```yaml
# Extra care for test files
- name: test-file-guidance
  event: PreToolUse
  match:
    tools: [Write, Edit]
    enabled_when: "tool.input.path =~ '(test_|_test\\.|\\.test\\.|spec\\.)'"
  action:
    type: inject
    source: file
    path: .claude/context/testing-guidelines.md
```

---

## Optimization Patterns

### Consolidate Similar Rules

**Before** (3 rules):
```yaml
- name: python-lint
  match: { extensions: [.py] }
  action: { type: inject, path: lint.md }
  
- name: js-lint
  match: { extensions: [.js] }
  action: { type: inject, path: lint.md }
  
- name: ts-lint
  match: { extensions: [.ts] }
  action: { type: inject, path: lint.md }
```

**After** (1 rule):
```yaml
- name: code-lint
  match:
    extensions: [.py, .js, .ts]
  action:
    type: inject
    source: file
    path: .claude/context/lint-standards.md
```

### Priority-Based Short-Circuiting

Block rules first, context injection later.

```yaml
# Priority 1-10: Blockers (highest priority)
- name: security-block
  priority: 5
  action: { type: block }

# Priority 50-70: Context injection
- name: code-standards
  priority: 50
  action: { type: inject }

# Priority 90-100: Logging/telemetry (lowest priority)
- name: action-log
  priority: 100
  action: { type: run, command: log.sh }
```

### Lazy Evaluation with enabled_when

Avoid expensive checks when not needed.

```yaml
# Only run Python checks for Python files
- name: python-security
  match:
    tools: [Write]
    enabled_when: "tool.input.path =~ '\\.py$'"
  action:
    type: run
    command: python-security-check.sh
```

---

## Pattern Index

| Pattern | Use Case | Key Technique |
|---------|----------|---------------|
| Language standards | Consistent code style | extensions + inject |
| Directory context | Layer-specific guidance | directories + inject |
| Dynamic context | Runtime information | command source |
| Block dangerous | Safety guardrails | command_match + block |
| Secret detection | Security | run + validation script |
| Pre-commit | Quality gates | command_match + run |
| Format on save | Automation | PostToolUse + run |
| Conventional commits | Consistency | run + validation |
| CI-specific | Environment awareness | enabled_when |
| Branch protection | Workflow enforcement | enabled_when + regex |

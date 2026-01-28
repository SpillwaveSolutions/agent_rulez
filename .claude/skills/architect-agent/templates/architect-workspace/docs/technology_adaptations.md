# Technology Adaptations

**Project:** [PROJECT_NAME]
**Technology Stack:** [TECH_STACK]
**Last Updated:** [DATE]

---

## Overview

This document describes project-specific technology adaptations and conventions that differ from standard architect-agent workflows.

**Template Note:** Replace [PLACEHOLDERS] with project-specific information.

---

## Technology Stack

### Primary Technologies
- **Build System:** [Gradle / Maven / npm / etc.]
- **Languages:** [Java / Kotlin / TypeScript / Python / etc.]
- **Framework:** [Spring Boot / FastAPI / Next.js / etc.]
- **Infrastructure:** [AWS CDK / Pulumi / Terraform / etc.]
- **Database:** [PostgreSQL / MySQL / MongoDB / etc.]

### Testing
- **Unit Tests:** [JUnit / pytest / Jest / etc.]
- **Integration Tests:** [Testcontainers / etc.]
- **E2E Tests:** [Playwright / Cypress / etc.]

---

## Build and Test Commands

### Build
```bash
[BUILD_COMMAND]
# Example: ./gradlew clean build
# Example: npm run build
# Example: poetry run task build
```

### Test
```bash
[TEST_COMMAND]
# Example: ./gradlew test
# Example: npm test
# Example: pytest
```

### Clean
```bash
[CLEAN_COMMAND]
# Example: ./gradlew clean
# Example: npm run clean
```

---

## Project Structure

```
[PROJECT_ROOT]/
├── [SOURCE_DIR]/           # Source code
├── [TEST_DIR]/             # Tests
├── [CONFIG_DIR]/           # Configuration
├── [DOCS_DIR]/             # Documentation
└── [BUILD_DIR]/            # Build output
```

**Example (Gradle/Java):**
```
platform-perimeter-data-router/
├── src/main/java/          # Source code
├── src/test/java/          # Tests
├── config/                 # Configuration
├── docs/                   # Documentation
└── build/                  # Build output
```

---

## JIRA Integration

**Project Key:** [JIRA_PROJECT_KEY]
**Board:** [BOARD_NAME]

### Ticket Format
```
[KEY]-[NUMBER]: [Title]
Example: PROJ-123: Implement Database Migration
```

### Epic Tracking
- Epic field: `customfield_[NUMBER]`
- Use `gh` CLI or JIRA API for epic queries

### Status Workflow
[PROJECT_STATUS_WORKFLOW]
Example:
- Backlog → Ready for Development → In Progress → In Review → Done

---

## GitHub Configuration

### Repository
**URL:** [GITHUB_REPO_URL]
**Default Branch:** [main / master / develop]

### Branch Naming
```
[BRANCH_PREFIX]/[TICKET]-[DESCRIPTION]
Example: feature/PROJ-123-database-migration
```

### GitHub Auth
```bash
# Verify auth
gh auth status

# Should show: [GITHUB_ORG]_[GITHUB_USER]
```

---

## Testing Conventions

### Test Organization
[PROJECT_TEST_ORGANIZATION]

Example (Gradle/Java):
```
src/test/java/
├── unit/           # Unit tests (fast)
├── integration/    # Integration tests (slower)
└── e2e/            # End-to-end tests (slowest)
```

### Test Naming
```
[NAMING_CONVENTION]
Example: test_[feature]_[scenario]_[expected_result]
```

### Coverage Requirements
- **Minimum:** [COVERAGE_MIN]%
- **Target:** [COVERAGE_TARGET]%

---

## Code Agent Specific Adaptations

### Quality Assurance Protocol

**After ANY code changes, code agent MUST:**
1. Run tests: `[TEST_COMMAND]`
2. Run build: `[BUILD_COMMAND]`
3. Verify coverage if applicable
4. Only mark task complete after all pass

### Common Commands

**Start Logging:**
```bash
/log-start "description"
```

**Check Status:**
```bash
[STATUS_COMMAND]
# Example: ./gradlew check
# Example: npm run lint && npm test
```

**Run Specific Tests:**
```bash
[SPECIFIC_TEST_COMMAND]
# Example: ./gradlew test --tests ClassName
# Example: pytest tests/test_specific.py
```

---

## Environment Variables

### Required
```bash
[ENV_VAR_1]=[DESCRIPTION]
[ENV_VAR_2]=[DESCRIPTION]
```

### Optional
```bash
[OPTIONAL_ENV_VAR_1]=[DESCRIPTION]
```

---

## Dependencies

### Adding Dependencies

**Gradle:**
```groovy
dependencies {
    implementation '[GROUP]:[ARTIFACT]:[VERSION]'
}
```

**npm:**
```bash
npm install [PACKAGE]@[VERSION]
```

**Poetry:**
```bash
poetry add [PACKAGE]
```

### Updating Dependencies
```bash
[UPDATE_COMMAND]
# Example: ./gradlew dependencies --refresh-dependencies
# Example: npm update
# Example: poetry update
```

---

## Code Quality Tools

### Linting
```bash
[LINT_COMMAND]
# Example: ./gradlew checkstyleMain
# Example: npm run lint
# Example: poetry run ruff check
```

### Formatting
```bash
[FORMAT_COMMAND]
# Example: ./gradlew spotlessApply
# Example: npm run format
# Example: poetry run black .
```

### Static Analysis
```bash
[STATIC_ANALYSIS_COMMAND]
# Example: ./gradlew spotbugsMain
# Example: npm run type-check
# Example: poetry run mypy .
```

---

## Deployment

### Development
```bash
[DEV_DEPLOY_COMMAND]
```

### Staging
```bash
[STAGING_DEPLOY_COMMAND]
```

### Production
```bash
[PROD_DEPLOY_COMMAND]
```

**Note:** Production deployments typically require architect approval

---

## Common Issues and Solutions

### Issue 1: [COMMON_ISSUE]
**Solution:** [SOLUTION]

### Issue 2: [COMMON_ISSUE]
**Solution:** [SOLUTION]

---

## References

### Project Documentation
- [LINK_TO_MAIN_DOCS]
- [LINK_TO_API_DOCS]
- [LINK_TO_ARCHITECTURE_DOCS]

### External Resources
- [RELEVANT_EXTERNAL_DOCS]

---

**Last Updated:** [DATE]
**Maintainer:** [TEAM / PERSON]

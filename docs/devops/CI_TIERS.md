# CI Tiers

## Overview

CCH uses a two-tier CI system to balance development velocity with release quality:

| Tier | When | Duration | Purpose |
|------|------|----------|---------|
| **Fast CI** | PRs to `develop`, feature pushes | ~2-3 min | Rapid feedback |
| **Full Validation** | PRs to `main`, releases | ~10-15 min | Release gate |

---

## Fast CI

**Workflow:** `.github/workflows/ci.yml`

### Triggers
- Push to `develop` branch
- Push to `feature/*` branches
- Pull requests targeting `develop`

### Jobs

| Job | Description | Duration |
|-----|-------------|----------|
| `fmt` | Check code formatting | ~30s |
| `clippy` | Lint with clippy | ~1 min |
| `test-unit` | Run unit tests | ~1 min |
| `test-iq-smoke` | Linux IQ smoke test | ~1 min |
| `coverage` | Generate coverage report | ~2 min |

### What It Validates
- Code compiles without errors
- Code follows formatting standards
- No clippy warnings
- Unit tests pass
- Basic IQ installation works on Linux

### What It Skips
- Multi-platform builds
- Full OQ test suite
- PQ performance tests
- Evidence collection

### When to Use
- Daily development
- Quick iterations
- Feature development
- Bug fixes

---

## Full Validation

**Workflow:** `.github/workflows/validation.yml`

### Triggers
- Pull requests targeting `main`
- Release tags (`v*`)
- Manual dispatch (`workflow_dispatch`)

### Jobs

| Phase | Jobs | Duration |
|-------|------|----------|
| IQ | 4 platform builds (macOS ARM64, Intel, Linux, Windows) | ~5 min |
| OQ | US1-US5 test suites | ~3 min |
| PQ | Performance and memory tests | ~3 min |
| Report | Generate validation report | ~1 min |

### What It Validates
- Installation works on all 4 platforms
- All operational features work correctly
- Performance meets requirements
- Memory usage is acceptable
- No regressions from previous release

### Evidence Collected
- IQ evidence per platform
- OQ test results (JSON)
- PQ benchmark data
- Combined validation report

### When to Use
- Merging to production (`main`)
- Creating releases
- Formal validation audits

---

## Workflow Files

### Fast CI (`.github/workflows/ci.yml`)
```yaml
on:
  push:
    branches: [develop, "feature/**"]
  pull_request:
    branches: [develop]
```

### Full Validation (`.github/workflows/validation.yml`)
```yaml
on:
  pull_request:
    branches: [main]
  push:
    tags: ['v*']
  workflow_dispatch:
```

### IQ Validation (`.github/workflows/iq-validation.yml`)
```yaml
on:
  workflow_dispatch:  # Manual only
```

---

## Running Locally

### Fast CI Equivalent
```bash
cd cch_cli
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --lib
cargo test iq_
```

### Full Validation Equivalent
```bash
# Fast CI checks
cd cch_cli
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test

# Evidence collection
cd ..
./scripts/collect-iq-evidence.sh --release
./scripts/collect-oq-evidence.sh --release
./scripts/collect-pq-evidence.sh --release
./scripts/generate-validation-report.sh
```

---

## Interpreting Failures

### Fast CI Failures

| Job | Failure Meaning | Fix |
|-----|-----------------|-----|
| `fmt` | Code not formatted | Run `cargo fmt` |
| `clippy` | Lint warnings | Fix warnings or add `#[allow(...)]` |
| `test-unit` | Unit test failed | Fix test or code |
| `test-iq-smoke` | Installation broken | Check build/install logic |

### Full Validation Failures

| Phase | Failure Meaning | Action |
|-------|-----------------|--------|
| IQ platform failure | Build/install broken on that platform | Check platform-specific code |
| OQ failure | Feature regression | Review test failure details |
| PQ failure | Performance regression | Profile and optimize |

---

## Coverage

Coverage runs in **both** tiers:
- **Fast CI:** Generates report, non-blocking warning if < 80%
- **Full Validation:** Same behavior, artifacts uploaded

Coverage is informational - it doesn't block PRs, but low coverage generates a warning.

---

## Manual Validation

For formal validation runs (compliance, audits):

```bash
# Trigger IQ validation manually
gh workflow run iq-validation.yml

# Or run full validation
gh workflow run validation.yml
```

Evidence artifacts will be available in the GitHub Actions run.

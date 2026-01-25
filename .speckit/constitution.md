# Project Constitution

## Project Vision

### Strategic Positioning
CCH (Claude Context Hooks) is evolving from a "powerful local hook system" into a **first-class, auditable, local AI policy engine** suitable for real organizational governance.

The project encompasses:
1. **CCH Core** (v1.0.0 Released) - Rust-based policy engine binary
2. **Phase 2 Governance** (Planned) - Policy modes, metadata, priorities, enterprise features
3. **RuleZ UI** (Planned) - Tauri desktop application for visual configuration

### Design Philosophy
**LLMs do not enforce policy. LLMs are subject to policy.**

- CCH is the policy authority
- Skills are policy authors
- Claude is policy-constrained execution

This positions CCH as comparable to:
- OPA (but human-readable)
- Terraform Sentinel (but local)
- Kubernetes admission controllers (but for agents)

---

## Git Workflow Principles

### Branching Model

```
main (protected)          <- Production-ready, fully validated
  ^
  |
develop (default)         <- Integration branch, fast CI
  ^
  |
feature/* | fix/*         <- Short-lived working branches
```

| Branch | Purpose | CI Level | Protection |
|--------|---------|----------|------------|
| `main` | Production-ready releases | Full Validation | Protected, requires IQ/OQ/PQ |
| `develop` | Integration branch (default) | Fast CI | Protected, requires Fast CI |
| `feature/*` | Active development | Fast CI | None |
| `fix/*` | Bug fixes | Fast CI | None |
| `release/*` | Release candidates | Full Validation | None |
| `hotfix/*` | Emergency fixes to main | Full Validation | None |

### Feature Branch Requirement
- **NEVER commit directly to `main` or `develop`** - This is a non-negotiable principle
- All feature work MUST be done in a dedicated feature branch
- Pull Requests are REQUIRED for all changes
- Code review via PR ensures quality and knowledge sharing

### Branch Naming Convention
- Features: `feature/<feature-name>` (e.g., `feature/add-debug-command`)
- Bugfixes: `fix/<bug-description>` (e.g., `fix/config-parsing-error`)
- Documentation: `docs/<doc-topic>` (e.g., `docs/update-readme`)
- Releases: `release/<version>` (e.g., `release/v1.0.0`)
- Hotfixes: `hotfix/<issue>` (e.g., `hotfix/critical-security-fix`)

### Standard PR Workflow (Daily Development)
1. Create feature branch from `develop`
2. Implement changes with atomic, conventional commits
3. **Run pre-commit checks locally** (see below)
4. Push branch and create Pull Request **targeting `develop`**
5. Fast CI runs (~2-3 minutes)
6. Request review and address feedback
7. Merge to `develop` via GitHub
8. Delete feature branch after merge

### Release Workflow (Production Deployment)
1. Create PR from `develop` to `main`
2. Full IQ/OQ/PQ validation runs (~10-15 minutes)
3. All 4 platforms tested (macOS ARM64, Intel, Linux, Windows)
4. Evidence artifacts collected
5. Merge to `main` only after all validation passes
6. Tag release from `main`

### Hotfix Workflow (Emergency Fixes)
1. Create `hotfix/*` branch from `main`
2. Implement fix with minimal changes
3. Create PR to `main` (triggers full validation)
4. After merge to `main`, backport to `develop`

### Pre-Commit Checks (MANDATORY)

**For CCH Core (Rust):**
```bash
cd cch_cli
cargo fmt --check                                          # Formatting
cargo clippy --all-targets --all-features -- -D warnings   # Linting
cargo test                                                 # All tests
```

**For RuleZ UI (TypeScript/React):**
```bash
cd rulez_ui
bun run lint                                               # ESLint
bun run typecheck                                          # TypeScript
bun run test                                               # Bun tests
```

**NEVER commit if any check fails.** This is non-negotiable. CI will reject PRs that fail these checks, wasting time and creating noise.

Quick one-liner for CCH Core:
```bash
cd cch_cli && cargo fmt && cargo clippy --all-targets --all-features -- -D warnings && cargo test
```

### Rationale
- **Two-branch model** enables fast iteration on `develop` while maintaining production stability on `main`
- **Fast CI on develop** provides rapid feedback (~2-3 min) during active development
- **Full validation on main** ensures releases are thoroughly tested across all platforms
- Direct commits bypass code review, risk introducing bugs, and make it difficult to revert changes

---

## CI/CD Policy

### CI Tiers

| Tier | Trigger | Duration | What Runs |
|------|---------|----------|-----------|
| **Fast CI** | Push to `develop`, `feature/*`; PRs to `develop` | ~2-3 min | fmt, clippy, unit tests, Linux IQ smoke test |
| **Full Validation** | PRs to `main`, release tags, manual dispatch | ~10-15 min | Fast CI + IQ (4 platforms) + OQ + PQ + evidence |

### Fast CI (~2-3 minutes)
**Purpose:** Rapid feedback during active development

**Jobs:**
- Format check (`cargo fmt --check`)
- Linting (`cargo clippy`)
- Unit tests (`cargo test --lib`)
- Linux IQ smoke test (`cargo test iq_`)
- Code coverage (report only, non-blocking)

**When it runs:**
- Every push to `develop` or `feature/*` branches
- Every PR targeting `develop`

### Full Validation (~10-15 minutes)
**Purpose:** Release gate validation ensuring production readiness

**Jobs:**
- All Fast CI jobs
- IQ on 4 platforms (macOS ARM64, macOS Intel, Linux, Windows)
- Full OQ test suite (US1-US5)
- PQ benchmarks (performance, memory)
- Evidence collection and artifact upload

**When it runs:**
- PRs targeting `main`
- Release tags (`v*`)
- Manual workflow dispatch

### Validation Gates

| Event | Required Checks | Blocking |
|-------|-----------------|----------|
| PR to `develop` | Fast CI passes | Yes |
| PR to `main` | Full IQ/OQ/PQ Validation passes | Yes |
| Release tag | Full Validation already passed on `main` | Yes |

### Evidence Collection
Full validation automatically collects and uploads:
- IQ evidence per platform
- OQ test results
- PQ benchmark data
- Combined validation report

Evidence is stored as GitHub Actions artifacts and can be downloaded for compliance audits.

Reference: [CI Tiers Documentation](docs/devops/CI_TIERS.md)

---

## Core Principles

### Safety First
- **Zero unsafe code blocks**: All Rust code must be memory-safe using Rust's ownership system
- **Fail-open design**: System continues operating even when individual components fail
- **Comprehensive error handling**: All error paths must be handled gracefully
- **No network access**: CCH operates purely locally for security
- **No telemetry**: User privacy is paramount; no analytics or data collection

### Performance Critical
- **Sub-10ms processing**: Hook events must be processed in under 10ms
- **Minimal dependencies**: Only essential crates to minimize binary size and startup time
- **Async efficiency**: Use tokio with minimal features for optimal performance
- **UI responsiveness**: RuleZ UI must maintain 60fps (< 16ms input latency)
- **Fast startup**: CCH cold start <5ms, RuleZ UI launch <2s

### Configuration-Driven Architecture
- **YAML-based rules**: All behavior defined by user configuration, not hardcoded logic
- **Flexible matching**: Support tools, extensions, directories, operations, and command patterns
- **Pluggable actions**: Support inject, run, block, and block_if_match actions
- **Backward compatible**: New features (metadata, modes, priority) are always optional

### Observability & Debugging
- **Complete audit trail**: All decisions logged in JSON Lines format
- **Debug configuration**: Optional detailed logging for troubleshooting
- **CLI tools**: Commands for log querying and rule explanation
- **Visual debugging**: RuleZ UI provides simulation and trace visualization

## Technology Choices

### CCH Core (Rust Binary)

**Language & Runtime:**
- **Rust 2024 edition**: Modern Rust with stable features
- **No unsafe code**: Memory safety guaranteed by compiler
- **Tokio async runtime**: For efficient async operations

**Core Dependencies:**
- **serde**: JSON/YAML serialization (no other serialization crates)
- **clap**: CLI argument parsing (derive API)
- **regex**: Pattern matching for rule conditions
- **tokio**: Async runtime (minimal features for performance)
- **tracing**: Structured logging (not println!)
- **chrono**: Time handling with serde support
- **dirs**: Cross-platform directory handling

**Project Structure:**
- **Workspace layout**: Separate binary crate for clean separation
- **Module organization**: Clear separation of concerns (cli, config, hooks, logging, models)
- **Test organization**: Unit, integration, and contract tests

### RuleZ UI (Desktop Application)

**Frontend Stack:**
- **Runtime**: Bun (all TypeScript/React operations)
- **Framework**: React 18 + TypeScript
- **Styling**: Tailwind CSS 4
- **Editor**: Monaco Editor + monaco-yaml
- **State**: Zustand + TanStack Query

**Desktop Framework:**
- **Tauri 2.0**: Rust backend with WebView frontend
- **IPC**: Type-safe command invocation between frontend and backend
- **File I/O**: Secure filesystem access via Tauri APIs

**Testing:**
- **Unit Tests**: Bun test (80%+ coverage for utilities)
- **E2E Tests**: Playwright (critical user flows)

## Coding Standards

### Rust (CCH Core)

**Error Handling:**
- Use `anyhow::Result` for application errors
- Use `thiserror` for library crate error types
- Log errors with context, don't panic

**Async Patterns:**
- Use `tokio::main` with current_thread flavor for minimal overhead
- Prefer async functions over blocking operations
- Use tokio::process for external command execution

**Configuration:**
- Load from `.claude/hooks.yaml` (project) with fallback to `~/.claude/hooks.yaml` (user)
- Validate configuration on startup
- Support environment variable overrides

**Logging:**
- Use tracing macros (info!, error!, warn!, debug!)
- Structure logs as JSON for machine readability
- Include session_id and event context in all log entries

### TypeScript (RuleZ UI)

**Type Safety:**
- Strict TypeScript configuration
- No `any` types without explicit justification
- Prefer interfaces over type aliases for objects

**React Patterns:**
- Functional components with hooks
- Zustand for global state management
- TanStack Query for async operations

**Styling:**
- Tailwind CSS utility classes
- Dark/light theme support via CSS variables
- System preference detection with manual override

## Architectural Decisions

### Event Processing Pipeline (CCH Core)
1. Parse JSON event from stdin
2. Load and validate configuration
3. Match rules against event
4. Execute matching rule actions
5. Log decision with full provenance
6. Output JSON response to stdout

### Rule Matching Logic
- AND conditions within matchers (all must match)
- OR across rules (first matching rule wins)
- Actions executed in rule definition order
- Block actions terminate processing immediately

### Phase 2 Governance Extensions

**Policy Modes** (enforce | warn | audit):
- `enforce` (default): Normal blocking behavior
- `warn`: Never blocks, injects warning context
- `audit`: No injection, no blocking, logs only

**Rule Priority:**
- Higher numbers run first (default = 0)
- Enables explicit control over policy ordering
- Prevents emergent policy bugs

**Rule Metadata (Provenance):**
- `author`, `created_by`, `reason`, `confidence`
- `last_reviewed`, `ticket`, `tags`
- Included in logs and debug output for auditability

**Conflict Resolution:**
- enforce + warn = enforce wins
- audit + enforce = enforce wins
- Multiple enforce = highest priority wins

### Security Model
- No network access (pure local processing)
- File system access limited to configuration and log directories
- External script execution with timeout and controlled environment
- Input validation on all event data
- RuleZ UI: No arbitrary code execution, respects filesystem permissions

## Quality Gates

### CCH Core Performance
- Cold start: <5ms p95, <10ms p99
- Rule matching: <1ms for 100 rules
- Memory usage: <50MB resident
- No memory leaks in 24-hour stress test

### RuleZ UI Performance
- App launch: <2 seconds
- File load (10KB YAML): <100ms
- Validation response: <200ms
- Editor input latency: <16ms (60fps)
- Memory usage (idle): <150MB

### Reliability
- Zero crashes in normal operation
- Graceful degradation on configuration errors
- Fail-open behavior for non-critical failures
- Comprehensive test coverage
- RuleZ UI: Graceful handling of missing configs and CCH binary

### Maintainability
- Clear module boundaries
- Comprehensive documentation
- Automated testing and linting
- Simple deployment (single binary for CCH, installers for RuleZ UI)

---

## Validation Framework (IQ/OQ/PQ)

### Philosophy
**Governance tools must embody the same rigor they enforce.** CCH, as an AI policy engine, requires mathematical certainty that rules execute consistently across all platforms and conditions. This is achieved through the 3Q validation framework.

Reference: [IQ/OQ/PQ Integration Testing Guide](../docs/IQ_OQ_PQ_IntegrationTesting.md)

### Installation Qualification (IQ)

**Purpose:** Verify software installs correctly per documentation across all target platforms.

**Scope:**
| Platform | Architecture | Validation Required |
|----------|--------------|---------------------|
| macOS | Apple Silicon (ARM64) | Yes |
| macOS | Intel/AMD (x86_64) | Yes |
| Windows | x86_64 | Yes |
| Linux | x86_64, aarch64 | Yes |

**IQ Checklist:**
- [ ] Binary installs via documented method (cargo, binary download)
- [ ] `cch --version` returns correct version
- [ ] `cch init` creates `~/.claude/hooks.yaml`
- [ ] Log directory exists: `~/.claude/logs/`
- [ ] Platform-specific checks pass (code signing, permissions)
- [ ] Claude CLI integration verified

**Evidence Required:**
- Installation logs (`install.log`)
- Version verification output
- Configuration file creation proof
- Platform-specific verification (codesign on macOS, etc.)

### Operational Qualification (OQ)

**Purpose:** Verify features function correctly in operational environments.

**OQ Test Categories:**

| Category | Test Focus | Minimum Coverage |
|----------|------------|------------------|
| Rule Matching | Tool, directory, regex, extension patterns | 100% pattern types |
| Actions | block, inject, warn, run | 100% action types |
| Event Types | PreToolUse, PostToolUse, PermissionRequest | 100% event types |
| Modes | enforce, warn, audit | 100% mode types |
| Edge Cases | Invalid YAML, missing files, concurrent access | Critical paths |

**OQ Scenarios (Required):**
1. **Block Force Push** - Verify `git push --force` blocked with audit log
2. **Context Injection** - Verify context injected for file patterns
3. **Session Logging** - Verify JSON Lines audit log creation
4. **Permission Context** - Verify context provided during permission requests
5. **Mode Behavior** - Verify warn mode logs but allows, audit mode logs only

**Evidence Required:**
- Test execution reports (pass/fail per scenario)
- Event payloads that triggered rules (JSON)
- Log entries showing decisions
- Screenshots/output showing expected behavior

### Performance Qualification (PQ)

**Purpose:** Verify system meets performance requirements under realistic load.

**PQ Requirements:**

| Metric | Requirement | Measurement |
|--------|-------------|-------------|
| Latency (simple rule) | <5ms p95 | Benchmark suite |
| Latency (complex regex) | <10ms p95 | Benchmark suite |
| Throughput | >1000 events/sec | Load test |
| Memory (sustained) | <50MB RSS | 24-hour test |
| Memory leaks | None detected | 7-day stress test |

**PQ Test Protocol:**
1. Run latency benchmarks across all platforms
2. Execute sustained load test (1000 events/sec for 1 hour)
3. Run 7-day stress test with realistic workload
4. Monitor and record resource utilization
5. Compare results against platform baselines

**Evidence Required:**
- Benchmark results with percentile distributions
- Resource utilization graphs
- Stability metrics over extended periods
- Platform comparison data

### Evidence Collection Standards

**Directory Structure:**
```
docs/validation/
├── iq/
│   └── {date}/
│       ├── macos-arm64/
│       ├── macos-intel/
│       ├── windows/
│       └── linux/
├── oq/
│   └── {date}/
│       ├── test-results.json
│       ├── scenarios/
│       └── evidence/
├── pq/
│   └── {date}/
│       ├── benchmarks/
│       ├── load-tests/
│       └── stability/
└── sign-off/
    └── v{version}-validation-report.md
```

**Evidence Naming Convention:**
- `iq-{platform}-{date}.md` - IQ reports
- `oq-{test-id}-{date}.json` - OQ test results
- `pq-benchmark-{platform}-{date}.csv` - PQ metrics

**Retention Policy:**
- Major releases: Indefinite retention
- Minor releases: Minimum 2 years
- Patch releases: Minimum 1 year

### Validation Workflow

**Pre-Release Validation (Required):**
1. Run IQ on all target platforms
2. Execute full OQ test suite
3. Complete PQ benchmarks
4. Generate validation report
5. Obtain sign-off before release

**Continuous Validation (Automated):**
- Integration tests run on every PR (OQ subset)
- Nightly full OQ suite on main branch
- Weekly PQ benchmarks to detect regression
- Platform-specific IQ on release candidates

**Validation Gates:**
- **PR Merge:** Integration tests pass (OQ subset)
- **Release Candidate:** Full IQ + OQ pass on all platforms
- **Production Release:** IQ + OQ + PQ pass with signed evidence

### Integration Test Requirements

**Before Any Release:**
```bash
# Run integration tests (OQ subset)
task integration-test

# All 4 test cases must pass:
# - 01-block-force-push
# - 02-context-injection
# - 03-session-logging
# - 04-permission-explanations
```

**Integration tests are mandatory gate checks.** The release preflight script (`preflight-check.sh`) will fail if integration tests do not pass.

### Sign-Off Template

```markdown
## Validation Sign-Off - CCH v{VERSION}

**Validation Date:** {DATE}
**Product:** Claude Context Hooks (CCH)
**Version:** {VERSION}

### IQ Results
- [ ] macOS ARM64: PASSED/FAILED (evidence: docs/validation/iq/{date}/macos-arm64/)
- [ ] macOS Intel: PASSED/FAILED
- [ ] Windows: PASSED/FAILED
- [ ] Linux: PASSED/FAILED

**IQ Approved By:** _______________ Date: ___________

### OQ Results
- [ ] All test scenarios passed ({count}/{total})
- Evidence: docs/validation/oq/{date}/

**OQ Approved By:** _______________ Date: ___________

### PQ Results
- [ ] Latency requirements met on all platforms
- [ ] Stability test passed ({duration})
- Evidence: docs/validation/pq/{date}/

**PQ Approved By:** _______________ Date: ___________

**Overall Validation Status:** APPROVED / NOT APPROVED FOR RELEASE
```

---

## Platform Support

### CCH Core
| Platform | Architecture | Status |
|----------|--------------|--------|
| Linux | x86_64, aarch64 | Supported |
| macOS | Intel, Apple Silicon | Supported |
| Windows | x86_64 | Supported |

### RuleZ UI
| Platform | Format | Status |
|----------|--------|--------|
| macOS | .dmg, .app | Planned |
| Windows | .msi, .exe | Planned |
| Linux | .deb, .AppImage | Planned |

---

## Roadmap Summary

### v1.0.0 (Released)
- Core policy engine with blocking, injection, validation
- CLI commands: init, install, uninstall, validate, logs, explain, debug, repl
- 64+ tests, comprehensive logging

### Phase 2 Governance (Planned)
- Policy modes (enforce/warn/audit)
- Rule priority and metadata
- Enhanced `cch explain rule` command
- Trust levels for validators (informational)
- Policy packs concept (future-proof)

### RuleZ UI (Planned)
- Visual YAML editor with Monaco
- Real-time schema validation
- Debug simulator for testing rules
- Multi-file support (global + project configs)
- Dark/light theme support
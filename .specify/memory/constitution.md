# Claude Context Hooks (CCH) Constitution
<!-- A deterministic, auditable, local-first AI policy engine for Claude Code -->

## Core Principles

### I. Hook-First Architecture
Every policy must be enforceable via hooks, not just documented. LLMs are subject to policy, not policy enforcers. Hook violations must be blocked by default, with explicit opt-outs requiring justification.

### II. CLI Interface Standard
Every component must expose functionality via CLI. Text in/out protocol: stdin/args → stdout, errors → stderr. Support JSON + human-readable formats. No GUI-only features.

### III. Test-First Development (NON-NEGOTIABLE)
TDD mandatory: Tests written → User approved → Tests fail → Then implement. Red-Green-Refactor cycle strictly enforced. Unit, integration, and end-to-end tests required. Test coverage minimum 80%.

### IV. Rust Binary Reliability
The CCH binary must be deterministic, fast (<10ms cold start), and secure. No runtime dependencies. Cross-platform compatibility required. Memory-safe with comprehensive error handling.

### V. Skill-Driven Configuration
Skills must auto-discover project context and generate optimal configurations. Manual configuration is fallback only. Configurations must be versioned, auditable, and reproducible.

### VI. Policy Observability
Every policy decision must be logged in structured JSON. Logs must include rule provenance, confidence levels, and execution traces. No silent failures or unlogged actions.

### VII. Security by Default
Dangerous operations (force push, rm -rf /, credential exposure) must be blocked by default. Safety guards cannot be disabled without explicit justification and audit trail.

### VIII. TypeScript/React Excellence
Modern TypeScript 5.9+ with strict mode. React hooks must follow best practices. No legacy patterns. Type safety is non-negotiable.

## Additional Constraints

### Technology Stack Requirements
- **Frontend/Skill:** React 18+, TypeScript 5.9+, Vite for bundling
- **Backend/Binary:** Rust 2024 edition, no unsafe code blocks
- **Testing:** Jest/Vitest for JS, cargo test + IQ/OQ/PQ for Rust
- **CI/CD:** GitHub Actions with matrix builds
- **Package Management:** npm workspaces for monorepo

### Security Requirements
- No secrets in code or logs
- Input validation on all user inputs
- Sandboxed script execution
- Regular security audits
- OWASP Top 10 compliance

### Performance Standards
- Binary cold start: <15ms (AMENDED: realistic target; <5ms deferred to BACKLOG)
- Hot execution: <1ms per rule evaluation
- Memory usage: <50MB peak
- Bundle size: <2MB gzipped

## Development Workflow

### Code Review Requirements
- All PRs must pass CI checks
- Constitution compliance verification required
- Security review for hook-related changes
- Performance regression testing
- Cross-platform testing confirmation

### Quality Gates
- TypeScript strict mode compilation
- Rust clippy and rustfmt
- Test coverage minimum 80% (enforced in CI)
- Security scanning (SAST/DAST)
- Performance benchmarking
- IQ/OQ/PQ qualification testing with evidence generation

### Deployment Approval Process
- Binary releases require security review
- Major versions require architectural review
- Breaking changes require migration plan
- Rollback plan required for production deployments

## Governance

Constitution supersedes all other practices. Amendments require:
1. Proposal with rationale and impact analysis
2. Technical review for feasibility
3. Security review for policy changes
4. Implementation plan with migration strategy
5. Ratification by project maintainers

All PRs/reviews must verify constitution compliance. Complexity must be justified with performance metrics. Use CLAUDE.md for runtime development guidance. Hook configurations must be auditable and version-controlled.

**Version**: 1.1.0 | **Ratified**: 2025-01-21 | **Last Amended**: 2025-01-22

---

## Amendment History

### Amendment 1.1.0 (2025-01-22)
**Rationale**: Align constitution with implementation reality and add qualification testing requirements.

**Changes**:
1. **Rust Edition**: Updated from 2021 to 2024 (stable as of Dec 2025)
2. **Cold Start Target**: Adjusted from <5ms to <15ms (realistic without removing async runtime)
3. **Testing Requirements**: Added IQ/OQ/PQ qualification testing framework
4. **Coverage Enforcement**: Added "enforced in CI" to clarify automation

**Impact Analysis**:
- <5ms cold start requires removing tokio, which breaks async validators (deferred to BACKLOG)
- Rust 2024 provides async closures and improved ergonomics
- IQ/OQ/PQ testing provides auditable compliance evidence

# CCH Backlog

This document tracks deferred work items that are out of scope for the current release but may be addressed in future versions.

## Performance Improvements

### PERF-001: Achieve <5ms Cold Start
**Priority**: Medium
**Deferred From**: Constitution Amendment 1.1.0 (2025-01-22)

**Current State**: Binary cold start is ~15-40ms due to tokio async runtime initialization.

**Proposed Solution**:
1. Remove tokio dependency entirely
2. Use synchronous I/O for file operations
3. Use `std::process::Command` instead of `tokio::process::Command` for validators
4. Consider blocking I/O with manual timeout implementation

**Trade-offs**:
- Loss of async validator execution (must run sequentially)
- More complex timeout handling
- Potential blocking if validator scripts hang

**Acceptance Criteria**:
- [ ] Cold start (--version) < 5ms average over 100 runs
- [ ] All IQ/OQ/PQ tests still pass
- [ ] No regression in functionality

---

## Integration Features

### LOG-001: External Logging Integration
**Priority**: Low
**Status**: Not Started

**Description**: Support external logging backends for enterprise environments.

**Proposed Features**:
- Datadog integration via dogstatsd
- Splunk HTTP Event Collector (HEC)
- OpenTelemetry (OTLP) exporter
- Prometheus metrics endpoint

**Acceptance Criteria**:
- [ ] At least one external backend implemented
- [ ] Configuration via hooks.yaml settings
- [ ] Fallback to local JSON Lines on connection failure

---

### LANG-001: Additional Validator Languages
**Priority**: Low
**Status**: Not Started

**Description**: Support validators written in languages other than Python.

**Proposed Languages**:
- Node.js/JavaScript
- Ruby
- Go (compiled)
- Rust (compiled)

**Acceptance Criteria**:
- [ ] Language detection based on file extension or shebang
- [ ] Consistent stdin/stdout/stderr protocol
- [ ] Timeout handling for all languages

---

## Architecture Improvements

### ARCH-001: Plugin System
**Priority**: Low
**Status**: Conceptual

**Description**: Allow third-party extensions via a plugin architecture.

**Proposed Features**:
- Dynamic loading of action handlers
- Custom matcher types
- Pre/post processing hooks

**Considerations**:
- Security implications of dynamic code loading
- Performance impact
- API stability requirements

---

## Documentation

### DOCS-001: API Documentation
**Priority**: Medium
**Status**: Not Started

**Description**: Generate comprehensive API documentation.

**Deliverables**:
- [ ] Rustdoc for library crate
- [ ] OpenAPI/JSON Schema for event format
- [ ] Configuration schema with JSON Schema validation

---

## Notes

- Items are prioritized as High/Medium/Low
- Move items to Issues when work begins
- Reference this document in PRs that address backlog items

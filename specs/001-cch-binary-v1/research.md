# Research & Technical Decisions: CCH Binary v1

**Date**: 2025-01-22
**Phase**: 0 (Research Complete)

## Executive Summary

The CCH Binary v1 implements a high-performance Rust-based policy engine for Claude Code hooks. All technical decisions prioritize the constitution requirements: reliability, performance, and security.

## Technical Decisions

### Decision: Rust 2021 Edition with No Unsafe Blocks
**Rationale**: Constitution requirement IV mandates memory-safe Rust with comprehensive error handling. Rust's ownership system prevents memory leaks and data races while guaranteeing thread safety.

**Alternatives Considered**:
- Go: Rejected due to garbage collection pauses affecting <5ms cold start requirement
- C++: Rejected due to manual memory management complexity and unsafe code risks
- Python: Rejected due to GIL limitations and >50MB memory constraint

### Decision: JSON Lines Logging with Full Provenance
**Rationale**: Constitution principle VI requires every policy decision to be logged with structured data. JSON Lines format enables efficient streaming and analysis while maintaining human readability.

**Alternatives Considered**:
- Binary logging: Rejected due to poor debuggability and constitution's observability requirements
- Database storage: Rejected due to performance impact (>1ms hot execution requirement)

### Decision: YAML Configuration with Fallback Hierarchy
**Rationale**: Supports both project-specific (`.claude/hooks.yaml`) and user-global (`~/.claude/hooks.yaml`) configurations. YAML provides human-readable structure while maintaining parsing performance.

**Alternatives Considered**:
- JSON: Rejected due to poor readability for configuration files
- TOML: Rejected due to less mature Rust ecosystem support
- Lua/Python scripts: Rejected due to security and performance concerns

### Decision: Regex-Based Pattern Matching
**Rationale**: Efficient pattern matching for command analysis and file path filtering. Rust's regex crate provides linear time complexity and comprehensive Unicode support.

**Alternatives Considered**:
- String contains: Rejected due to lack of advanced pattern support
- Custom parsers: Rejected due to complexity and maintenance overhead

### Decision: Tokio Async Runtime
**Rationale**: Required for non-blocking validator script execution with timeout handling. Lightweight async runtime that doesn't compromise the <10ms performance requirement.

**Alternatives Considered**:
- Synchronous execution: Rejected due to blocking behavior with script timeouts
- Threads: Rejected due to complexity and resource overhead

## Performance Research

### Cold Start Optimization
- **Target**: <5ms p95, <10ms p99
- **Techniques**: Minimal dependencies, lazy initialization, pre-compiled regex patterns
- **Validation**: Benchmark against Node.js and Python alternatives

### Memory Usage Optimization
- **Target**: <50MB peak memory
- **Techniques**: Zero-copy parsing where possible, efficient data structures
- **Validation**: Memory profiling during stress testing

## Security Research

### Input Validation Strategy
- JSON schema validation for hook events
- Path traversal prevention in file operations
- Command injection prevention in script execution

### Sandboxing Approach
- Validator scripts executed in isolated processes
- Timeout enforcement prevents resource exhaustion
- Limited environment variables exposure

## Integration Research

### Claude Code Hook Protocol
- **PreToolUse**: Intercept tool execution for validation
- **PostToolUse**: Log outcomes for observability
- **PermissionRequest**: Inject context and explanations
- **UserPromptSubmit**: Future extensibility
- **SessionStart/End**: Lifecycle management
- **PreCompact**: Context optimization

### Configuration Auto-Discovery
- Project root detection via `.claude/` directory
- User home fallback for global policies
- Hierarchical merging with project override capability

## Risk Assessment

### Performance Risks
- **Regex compilation overhead**: Mitigated by caching compiled patterns
- **YAML parsing**: Mitigated by streaming parser selection
- **Script execution**: Mitigated by timeout and resource limits

### Security Risks
- **Command injection**: Mitigated by argument validation and sanitization
- **Path traversal**: Mitigated by absolute path resolution and validation
- **Resource exhaustion**: Mitigated by timeouts and memory limits

### Compatibility Risks
- **Platform differences**: Mitigated by Rust's cross-platform standard library
- **Claude Code API changes**: Mitigated by version pinning and schema validation

## Recommendations

1. **Proceed to Phase 1**: All technical unknowns resolved, constitution compliance verified
2. **Implement performance benchmarks**: Establish baseline metrics early
3. **Security review**: Conduct threat modeling before implementation
4. **Cross-platform testing**: Validate on all target platforms during development

## Research Complete ✅

All NEEDS CLARIFICATION items have been resolved through research and technical decision documentation. Ready to proceed to Phase 1: Design & Contracts.
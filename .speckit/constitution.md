# Project Constitution

## Core Principles

### Safety First
- **Zero unsafe code blocks**: All code must be memory-safe using Rust's ownership system
- **Fail-open design**: System continues operating even when individual components fail
- **Comprehensive error handling**: All error paths must be handled gracefully

### Performance Critical
- **Sub-10ms processing**: Hook events must be processed in under 10ms
- **Minimal dependencies**: Only essential crates to minimize binary size and startup time
- **Async efficiency**: Use tokio with minimal features for optimal performance

### Configuration-Driven Architecture
- **YAML-based rules**: All behavior defined by user configuration, not hardcoded logic
- **Flexible matching**: Support tools, extensions, directories, operations, and command patterns
- **Pluggable actions**: Support inject, run, block, and block_if_match actions

### Observability & Debugging
- **Complete audit trail**: All decisions logged in JSON Lines format
- **Debug configuration**: Optional detailed logging for troubleshooting
- **CLI tools**: Commands for log querying and rule explanation

## Technology Choices

### Language & Runtime
- **Rust 2024 edition**: Modern Rust with stable features
- **No unsafe code**: Memory safety guaranteed by compiler
- **Tokio async runtime**: For efficient async operations

### Core Dependencies
- **serde**: JSON/YAML serialization (no other serialization crates)
- **clap**: CLI argument parsing (derive API)
- **regex**: Pattern matching for rule conditions
- **tokio**: Async runtime (minimal features for performance)
- **tracing**: Structured logging (not println!)
- **chrono**: Time handling with serde support
- **dirs**: Cross-platform directory handling

### Project Structure
- **Workspace layout**: Separate binary crate for clean separation
- **Module organization**: Clear separation of concerns (cli, config, hooks, logging, models)
- **Test organization**: Unit, integration, and contract tests

## Coding Standards

### Error Handling
- Use `anyhow::Result` for application errors
- Use `thiserror` for library crate error types
- Log errors with context, don't panic

### Async Patterns
- Use `tokio::main` with current_thread flavor for minimal overhead
- Prefer async functions over blocking operations
- Use tokio::process for external command execution

### Configuration
- Load from `.claude/hooks.yaml` (project) with fallback to `~/.claude/hooks.yaml` (user)
- Validate configuration on startup
- Support environment variable overrides

### Logging
- Use tracing macros (info!, error!, warn!, debug!)
- Structure logs as JSON for machine readability
- Include session_id and event context in all log entries

## Architectural Decisions

### Event Processing Pipeline
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

### Security Model
- No network access (pure local processing)
- File system access limited to configuration and log directories
- External script execution with timeout and controlled environment
- Input validation on all event data

## Quality Gates

### Performance
- Cold start: <5ms p95, <10ms p99
- Rule matching: <1ms for 100 rules
- Memory usage: <50MB resident
- No memory leaks in 24-hour stress test

### Reliability
- Zero crashes in normal operation
- Graceful degradation on configuration errors
- Fail-open behavior for non-critical failures
- Comprehensive test coverage

### Maintainability
- Clear module boundaries
- Comprehensive documentation
- Automated testing and linting
- Simple deployment (single binary)
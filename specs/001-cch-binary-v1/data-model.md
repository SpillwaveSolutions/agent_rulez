# Data Model: CCH Binary v1

**Date**: 2025-01-22
**Phase**: 1 (Design Complete)

## Core Entities

### Rule
Configuration entry defining policy enforcement logic.

**Fields**:
- `name` (string): Unique identifier for the rule
- `description` (string, optional): Human-readable explanation
- `matchers` (object): Conditions that trigger the rule
  - `tools` (array of strings, optional): Tool names to match (e.g., ["Bash", "Edit"])
  - `extensions` (array of strings, optional): File extensions to match (e.g., [".rs", ".ts"])
  - `directories` (array of strings, optional): Directory patterns to match (e.g., ["src/**", "tests/**"])
  - `operations` (array of strings, optional): Operation types to match
  - `command_match` (string, optional): Regex pattern for command matching
- `actions` (object): Actions to take when rule matches
  - `inject` (string, optional): Path to context file to inject
  - `run` (string, optional): Path to validator script to execute
  - `block` (boolean, optional): Whether to block the operation
  - `block_if_match` (string, optional): Regex pattern for conditional blocking
- `metadata` (object, optional): Additional rule information
  - `priority` (number): Rule evaluation order
  - `timeout` (number): Script execution timeout in seconds

**Validation Rules**:
- `name` must be unique across all rules
- At least one matcher must be specified
- At least one action must be specified
- `inject` and `run` paths must be relative to project root
- `priority` defaults to 0 (higher numbers = higher priority)

**State Transitions**: Rules are immutable at runtime, reloaded on configuration change.

### Event
Claude Code hook event data structure.

**Fields**:
- `event_type` (enum): Hook event type
  - `PreToolUse`, `PostToolUse`, `PermissionRequest`, `UserPromptSubmit`, `SessionStart`, `SessionEnd`, `PreCompact`
- `tool_name` (string, optional): Name of the tool being used
- `tool_input` (object, optional): Tool parameters and arguments
- `session_id` (string): Unique session identifier
- `timestamp` (string): ISO 8601 timestamp
- `user_id` (string, optional): User identifier if available

**Validation Rules**:
- `event_type` must be valid enum value
- `session_id` must be present for all events
- `timestamp` must be valid ISO 8601 format

### Response
Binary output structure for hook responses.

**Fields**:
- `continue` (boolean): Whether the operation should proceed
- `context` (string, optional): Additional context to inject
- `reason` (string, optional): Explanation for blocking or context injection
- `timing` (object, optional): Performance metrics
  - `processing_ms` (number): Total processing time
  - `rules_evaluated` (number): Number of rules checked

**Validation Rules**:
- `continue` is always required
- `reason` must be present if `continue` is false
- `context` size limited to prevent performance issues

### LogEntry
Structured audit log record.

**Fields**:
- `timestamp` (string): ISO 8601 timestamp with microsecond precision
- `event_type` (string): Hook event type
- `session_id` (string): Session identifier
- `tool_name` (string, optional): Tool being used
- `rules_matched` (array of strings): Names of rules that matched
- `outcome` (enum): Result of evaluation
  - `allow`, `block`, `inject`
- `timing` (object): Performance data
  - `processing_ms` (number): Processing time
  - `rules_evaluated` (number): Rules checked
- `metadata` (object, optional): Additional context
  - `injected_files` (array): Files injected as context
  - `validator_output` (string): Script execution results

**Validation Rules**:
- All timestamps in UTC
- `outcome` must reflect actual decision made
- Sensitive data must be redacted from logs

## Relationships

### Rule → Event (Many-to-One)
Rules are evaluated against each incoming event. Multiple rules can match a single event.

### Event → Response (One-to-One)
Each event produces exactly one response from the binary.

### Event → LogEntry (One-to-One)
Each event is recorded as exactly one log entry for audit purposes.

### Rule → LogEntry (Many-to-Many)
Rules that matched an event are recorded in the corresponding log entry.

## Data Flow

1. **Input**: JSON Event → Binary
2. **Processing**: Load Configuration → Match Rules → Execute Actions → Generate Response
3. **Output**: JSON Response → Claude Code + Log Entry → File System

## Storage Strategy

### Configuration
- **Format**: YAML files
- **Location**: `.claude/hooks.yaml` (project) or `~/.claude/hooks.yaml` (user)
- **Loading**: Hierarchical merge with project overriding user settings
- **Validation**: Schema validation on load with detailed error messages

### Logs
- **Format**: JSON Lines (.jsonl)
- **Location**: `~/.claude/logs/cch.log`
- **Rotation**: Size-based rotation with configurable limits
- **Retention**: Configurable retention period with automatic cleanup

### Runtime State
- **Storage**: None (stateless design)
- **Caching**: Compiled regex patterns cached in memory
- **Concurrency**: Single-threaded processing for deterministic behavior

## Error Handling

### Configuration Errors
- Invalid YAML: Log error, use default empty configuration
- Schema violations: Log detailed errors, skip invalid rules
- Missing files: Log warning, continue with available configuration

### Runtime Errors
- Script execution failures: Log error, continue operation (fail-open)
- Timeout exceeded: Log warning, continue operation
- Memory limits: Log error, terminate gracefully

### Validation Strategy
- Input validation on all external data
- Type safety through Rust's type system
- Comprehensive error messages for debugging
- Graceful degradation on non-critical failures
# Integration Test: Permission Explanations

## Purpose

Verify that CCH (Claude Context Hooks) provides helpful context during permission request events, helping users make informed decisions.

## What This Tests

1. **PermissionRequest event handling** - CCH should process permission request events
2. **Context injection for permissions** - Relevant context files are injected
3. **Tool-specific explanations** - Different tools get different context

## Configuration

The test uses this `hooks.yaml` configuration:

```yaml
version: "1.0"

rules:
  - name: explain-bash-permissions
    event_types: ["PermissionRequest"]
    matchers:
      tools: ["Bash"]
    actions:
      inject_context:
        - ".claude/context/bash-permission-context.md"

  - name: explain-write-permissions
    event_types: ["PermissionRequest"]
    matchers:
      tools: ["Write", "Edit"]
    actions:
      inject_context:
        - ".claude/context/write-permission-context.md"
```

## Context Files

- `bash-permission-context.md` - Explains Bash command permissions and security considerations
- `write-permission-context.md` - Explains file write permissions and what to verify

## Expected Behavior

When Claude requests permission for a tool:

1. Claude sends a PermissionRequest event
2. CCH intercepts the event
3. CCH matches the appropriate explanation rule
4. CCH injects the context file content
5. The user sees helpful context alongside the permission prompt

## Running the Test

```bash
# From the integration test directory
./use-cases/04-permission-explanations/test.sh

# Or run all tests
./run-all.sh
```

## Success Criteria

- CCH logs show PermissionRequest events being processed
- Permission explanation rules are matched
- Context injection occurs for permission events

## Test Limitations

This test runs Claude without pre-approved tools to trigger permission requests. However:

- Claude may timeout waiting for permission approval
- The test uses a short timeout and expects this behavior
- The key verification is that CCH logs show the event was processed

## Notes

- Permission explanations help users understand what they're approving
- Different tools can have different explanation contexts
- This feature is especially useful for security-sensitive operations

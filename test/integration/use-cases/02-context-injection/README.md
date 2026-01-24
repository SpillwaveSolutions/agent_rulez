# Integration Test: Context Injection

## Purpose

Verify that CCH (Claude Context Hooks) correctly injects context files when Claude CLI operates on specific file types.

## What This Tests

1. **CDK context injection** - When Claude reads/edits a `.cdk.ts` file, CCH should inject CDK best practices
2. **Path-based matching** - Only files matching the pattern trigger injection
3. **Non-matching files** - Regular files should not trigger CDK context injection

## Configuration

The test uses this `hooks.yaml` configuration:

```yaml
version: "1.0"

rules:
  - name: cdk-context-injection
    matchers:
      tools: ["Write", "Edit", "Read"]
      path_match: ".*\\.cdk\\.ts$"
    actions:
      inject_context:
        - ".claude/context/cdk-best-practices.md"
```

## Test Files

- `sample.cdk.ts` - A sample CDK stack file that triggers context injection
- `.claude/context/cdk-best-practices.md` - The context file to be injected

## Expected Behavior

When Claude CLI reads or edits `sample.cdk.ts`:

1. Claude calls the Read tool with the file path
2. CCH intercepts the `PreToolUse` event
3. CCH matches the `cdk-context-injection` rule based on the path
4. CCH injects the context file content into the response
5. Claude receives the additional context alongside its normal operation

## Running the Test

```bash
# From the integration test directory
./use-cases/02-context-injection/test.sh

# Or run all tests
./run-all.sh
```

## Success Criteria

- CCH log contains entry with `rules_matched: ["cdk-context-injection"]`
- CCH log shows `injected_files` in the response metadata
- Non-CDK files do not trigger CDK context injection

## Notes

- Context injection adds helpful information without blocking operations
- The injected context appears in Claude's tool response
- Multiple context files can be injected for a single rule

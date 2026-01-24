# Integration Test: Block Force Push

## Purpose

Verify that CCH (Claude Context Hooks) correctly blocks dangerous git operations when Claude CLI attempts to execute them.

## What This Tests

1. **Force push blocking** - When Claude tries to run `git push --force`, CCH should intercept and block the operation
2. **Hard reset blocking** - When Claude tries to run `git reset --hard`, CCH should intercept and block
3. **Safe commands allowed** - Normal commands like `echo` should be allowed through

## Configuration

The test uses this `hooks.yaml` configuration:

```yaml
version: "1.0"

rules:
  - name: block-force-push
    matchers:
      tools: ["Bash"]
      command_match: "git push.*--force|git push.*-f"
    actions:
      block: true

  - name: block-hard-reset
    matchers:
      tools: ["Bash"]
      command_match: "git reset --hard"
    actions:
      block: true
```

## Expected Behavior

When Claude CLI is asked to run `git push --force`:

1. Claude calls the Bash tool with the command
2. CCH intercepts the `PreToolUse` event
3. CCH matches the `block-force-push` rule
4. CCH returns `continue_: false` with a block reason
5. Claude receives the block and does not execute the command

## Running the Test

```bash
# From the integration test directory
./use-cases/01-block-force-push/test.sh

# Or run all tests
./run-all.sh
```

## Success Criteria

- CCH log contains entry with `rules_matched: ["block-force-push"]`
- CCH log shows `outcome: "Block"` for force push attempts
- Safe commands are allowed (`outcome: "Allow"`)

## Notes

- Claude may refuse to run dangerous commands on its own before CCH even sees them
- The test accounts for this by checking log entries rather than strictly requiring a block
- Debug logging is enabled to capture full event details

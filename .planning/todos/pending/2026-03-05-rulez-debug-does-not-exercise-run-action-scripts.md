---
created: 2026-03-05T21:28:15.168Z
title: rulez debug does not exercise run action scripts
area: tooling
github_issue: https://github.com/SpillwaveSolutions/agent_rulez/issues/104
files:
  - rulez/src/cli/debug.rs
  - rulez/src/hooks.rs
---

## Problem

`rulez debug PreToolUse --tool Bash --command '...'` does not fully exercise `run:` action scripts the way the live hook does. During debug mode, rules with `run:` scripts return `{"continue": true}` even for commands that should be blocked.

When the same script is tested directly with piped JSON (`echo '...' | ./script.sh`), it correctly blocks. This means `rulez debug` either:
- Doesn't pipe the event JSON to the script's stdin, or
- Doesn't execute `run:` scripts at all in debug mode, or
- Handles the script output differently than the live hook path

This makes it impossible to test validator rules without a live Claude Code session, defeating the purpose of the debug command.

## Solution

Ensure `rulez debug` executes `run:` action scripts identically to the live hook path:
1. Pipe the full event JSON to the script's stdin
2. Capture the script's stdout JSON response
3. Use the script's `continue`/`reason` fields in the debug output
4. Show the script's actual response in the debug summary (not just the rule engine's default)

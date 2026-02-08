#!/usr/bin/env python3
"""
Test validator: Detect console.log statements in code.

This validator reads a CCH event from stdin and checks if the code
being written contains console.log statements.

Exit codes:
  0 = Allow (no console.log found)
  1 = Block (console.log found)

Stdout: Context to inject (when allowing)
Stderr: Reason for blocking (when blocking)
"""
import sys
import json
import re

def main():
    # Read event from stdin
    try:
        event_json = sys.stdin.read()
        event = json.loads(event_json)
    except json.JSONDecodeError as e:
        print(f"Invalid JSON input: {e}", file=sys.stderr)
        sys.exit(1)
    
    # Get the content being written
    tool_input = event.get("tool_input", {})
    content = tool_input.get("newString") or tool_input.get("content") or ""
    
    # Check for console.log patterns
    console_pattern = re.compile(r'\bconsole\.(log|warn|error|debug|info)\s*\(')
    
    matches = console_pattern.findall(content)
    
    if matches:
        # Block with reason
        methods = ", ".join(set(matches))
        print(f"Found console.{methods} statements in code. "
              f"Use proper logging instead.", file=sys.stderr)
        sys.exit(1)
    else:
        # Allow with optional context
        print("Code review passed: No console statements found.")
        sys.exit(0)

if __name__ == "__main__":
    main()

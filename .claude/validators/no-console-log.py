#!/usr/bin/env python3
"""
Validator script that blocks console.log statements in code.
Exit 0 = allow, Exit 1 = block
"""
import sys
import json

def main():
    # Read event from stdin
    event_json = sys.stdin.read()
    event = json.loads(event_json)
    
    # Get the content being written/edited
    tool_input = event.get("tool_input", {})
    content = tool_input.get("newString") or tool_input.get("content") or ""
    
    # Check for console.log
    if "console.log" in content:
        print("console.log statements are not allowed in production code", file=sys.stderr)
        sys.exit(1)
    
    # Allow the operation
    sys.exit(0)

if __name__ == "__main__":
    main()

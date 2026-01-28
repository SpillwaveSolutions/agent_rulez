# Bash Command Permission Context

## Why This Permission is Needed

The assistant is requesting permission to execute a shell command. This allows it to:
- Run system utilities
- Execute build tools
- Interact with external services

## Security Considerations

Before approving, verify:
- The command does not modify critical system files
- The command does not expose sensitive data
- The command is appropriate for the current task

## Common Safe Commands

- `echo`, `cat`, `ls`, `pwd` - Information display
- `npm install`, `cargo build` - Package management
- `git status`, `git log` - Version control queries

## Commands Requiring Caution

- `rm`, `mv` - File manipulation
- `chmod`, `chown` - Permission changes
- `curl`, `wget` - Network operations

---
*This context provided by CCH to help with permission decisions.*

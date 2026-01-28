# File Write Permission Context

## Why This Permission is Needed

The assistant is requesting permission to write or edit a file. This allows it to:
- Create new files
- Modify existing content
- Update configuration

## Security Considerations

Before approving, verify:
- The file path is within the expected project directory
- The changes align with the requested task
- No sensitive files are being modified

## Safe File Operations

- Creating source code files
- Updating configuration files
- Writing documentation

## Operations Requiring Caution

- Modifying `.env` files (may contain secrets)
- Changing system configuration
- Overwriting existing important files

---
*This context provided by CCH to help with permission decisions.*

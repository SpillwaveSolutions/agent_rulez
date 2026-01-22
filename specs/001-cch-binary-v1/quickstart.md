# Quickstart: CCH Binary v1

**Date**: 2025-01-22
**Phase**: 1 (Design Complete)

## Overview

The CCH (Claude Context Hooks) Binary is a high-performance Rust-based policy engine that enforces development policies through Claude Code hooks. It provides security-by-default behavior, automatic context injection, and comprehensive observability.

## Installation

### Pre-built Binaries
```bash
# Download from releases page
curl -L https://github.com/yourorg/cch/releases/download/v1.0.0/cch-$(uname -s)-$(uname -m) -o cch
chmod +x cch
sudo mv cch /usr/local/bin/
```

### Build from Source
```bash
git clone https://github.com/yourorg/cch-binary.git
cd cch-binary
cargo build --release
cp target/release/cch /usr/local/bin/
```

## Basic Usage

### Test Installation
```bash
cch --version
cch --help
```

### Validate Configuration
```bash
cch validate .claude/hooks.yaml
```

### View Logs
```bash
cch logs
cch logs --tail 10
```

## Configuration

### Project Configuration (.claude/hooks.yaml)
```yaml
version: "1.0"
rules:
  - name: block-force-push
    description: "Block dangerous git force push operations"
    matchers:
      tools: ["Bash"]
      command_match: "git push.*--force"
    actions:
      block: true

  - name: inject-cdk-context
    description: "Inject AWS CDK context for CDK files"
    matchers:
      tools: ["Edit", "Write"]
      directories: ["cdk/**"]
      extensions: [".ts", ".js"]
    actions:
      inject: ".claude/skills/aws-cdk/SKILL.md"

  - name: validate-no-console
    description: "Run custom validator for console.log statements"
    matchers:
      tools: ["Edit", "Write"]
      extensions: [".js", ".ts"]
    actions:
      run: ".claude/validators/no-console.py"
```

### Global Configuration (~/.claude/hooks.yaml)
```yaml
version: "1.0"
rules:
  - name: global-security
    description: "Global security policies"
    matchers:
      command_match: "rm -rf /"
    actions:
      block: true
```

## Hook Events

### PreToolUse (Security & Context Injection)
- **Purpose**: Validate operations before execution
- **Input**: Tool name and parameters
- **Actions**: Block dangerous operations, inject context
- **Example**: Block `git push --force`, inject CDK documentation

### PostToolUse (Observability)
- **Purpose**: Log outcomes for analysis
- **Input**: Tool execution results
- **Actions**: Record metrics and decisions

### PermissionRequest (User Interaction)
- **Purpose**: Provide explanations before permission prompts
- **Input**: Operation requiring user approval
- **Actions**: Inject explanation templates

## Custom Validators

### Python Validator Example (.claude/validators/no-console.py)
```python
#!/usr/bin/env python3
import sys
import json
import re

# Read event from stdin
event = json.load(sys.stdin)

# Check if this is a file edit/write operation
if event.get("tool_name") in ["Edit", "Write"]:
    file_path = event.get("tool_input", {}).get("filePath", "")
    old_string = event.get("tool_input", {}).get("oldString", "")
    new_string = event.get("tool_input", {}).get("newString", "")

    # Check for console.log in the new content
    if re.search(r'console\.log', new_string):
        print("console.log found - consider using proper logging", file=sys.stderr)
        sys.exit(1)  # Block the operation

# Allow the operation
sys.exit(0)
```

### Usage in Configuration
```yaml
actions:
  run: ".claude/validators/no-console.py"
```

## Integration with Claude Code

### Automatic Discovery
CCH automatically discovers configuration in this order:
1. `.claude/hooks.yaml` (project-specific)
2. `~/.claude/hooks.yaml` (user-global)
3. Default empty configuration

### Hook Registration
Claude Code automatically calls CCH for all supported hook events when the binary is in PATH.

### Context Injection
Injected context appears in Claude's responses with clear source attribution:
```
## Injected Context: AWS CDK Best Practices

Use CDK v2 constructs for new projects...
```

## Troubleshooting

### Configuration Validation
```bash
# Validate YAML syntax and schema
cch validate .claude/hooks.yaml

# Check for common issues
cch validate --strict .claude/hooks.yaml
```

### Log Analysis
```bash
# View recent decisions
cch logs --since "1 hour ago"

# Search for specific rules
cch logs --rule block-force-push

# Analyze performance
cch logs --stats
```

### Debug Mode
```bash
# Enable verbose logging
export CCH_LOG_LEVEL=debug
cch [command]
```

## Performance Expectations

- **Cold Start**: <5ms (first request)
- **Hot Execution**: <1ms (subsequent requests)
- **Memory Usage**: <50MB peak
- **Rule Evaluation**: <1ms for 100 rules

## Security Features

- **Input Validation**: All inputs validated against schemas
- **Path Safety**: No path traversal vulnerabilities
- **Script Sandboxing**: Validators run in isolated processes
- **Fail-Open Design**: Errors don't break development workflow
- **Audit Logging**: All decisions recorded for compliance

## Examples

### Block Dangerous Operations
```yaml
rules:
  - name: no-force-push
    matchers:
      command_match: "git push.*--force|-f"
    actions:
      block: true
```

### Inject Documentation Context
```yaml
rules:
  - name: react-context
    matchers:
      extensions: [".jsx", ".tsx"]
    actions:
      inject: ".claude/docs/react-hooks.md"
```

### Custom Business Logic
```yaml
rules:
  - name: code-quality
    matchers:
      tools: ["Edit", "Write"]
    actions:
      run: "scripts/check-code-quality.py"
```

## Next Steps

1. **Install CCH Binary** in your development environment
2. **Create `.claude/hooks.yaml`** with your project policies
3. **Test Configuration** with `cch validate`
4. **Monitor Logs** with `cch logs` to understand rule firing
5. **Customize Validators** for your specific needs

For detailed API documentation, see the contracts/ directory.
# Claude Code CLI Reference

Complete reference for Claude Code command-line interface commands and flags.

## Contents

- [CLI Commands](#cli-commands)
- [Essential Flags for Automation](#essential-flags-for-automation)
- [All CLI Flags](#all-cli-flags)
- [Agents Flag Format](#agents-flag-format)
- [System Prompt Flags](#system-prompt-flags)
- [Output Formats](#output-formats)

---

## CLI Commands

| Command | Description | Example |
|---------|-------------|---------|
| `claude` | Start interactive REPL | `claude` |
| `claude "query"` | Start REPL with initial prompt | `claude "explain this project"` |
| `claude -p "query"` | Query via SDK, then exit | `claude -p "explain this function"` |
| `cat file \| claude -p "query"` | Process piped content | `cat logs.txt \| claude -p "explain"` |
| `claude -c` | Continue most recent conversation | `claude -c` |
| `claude -c -p "query"` | Continue via SDK | `claude -c -p "Check for type errors"` |
| `claude -r "<session>" "query"` | Resume session by ID or name | `claude -r "auth-refactor" "Finish this PR"` |
| `claude update` | Update to latest version | `claude update` |
| `claude mcp` | Configure MCP servers | `claude mcp` |

---

## Essential Flags for Automation

These are the most commonly used flags for subprocess automation:

| Flag | Purpose | Example |
|------|---------|---------|
| `-p`, `--print` | Non-interactive mode (required for automation) | `claude -p "query"` |
| `--allowedTools` | Pre-approve tools (skip permission prompts) | `--allowedTools Write Read Bash` |
| `--add-dir` | Grant access to additional directories | `--add-dir ../apps ../lib` |
| `--settings` | Pass settings JSON or file path | `--settings ./settings.json` |
| `--model` | Select model (sonnet, opus, or full name) | `--model sonnet` |
| `--output-format` | Output format: text, json, stream-json | `--output-format json` |
| `--max-turns` | Limit agentic turns | `--max-turns 3` |

**Minimal automation command:**
```bash
claude -p "Your prompt" --allowedTools Write Read Edit
```

**Full automation command:**
```bash
claude \
  --model sonnet \
  --add-dir /path/to/skills \
  --allowedTools Write Read Edit Bash \
  --output-format json \
  --max-turns 10 \
  -p "Your automation prompt"
```

---

## All CLI Flags

| Flag | Description | Example |
|------|-------------|---------|
| `--add-dir` | Add additional working directories | `--add-dir ../apps ../lib` |
| `--agent` | Specify an agent for the session | `--agent my-custom-agent` |
| `--agents` | Define custom subagents via JSON | See [Agents Flag Format](#agents-flag-format) |
| `--allowedTools` | Tools that run without permission prompts | `--allowedTools "Bash(git log:*)" Read` |
| `--append-system-prompt` | Append text to default system prompt | `--append-system-prompt "Use TypeScript"` |
| `--betas` | Beta headers for API requests | `--betas interleaved-thinking` |
| `--chrome` | Enable Chrome browser integration | `--chrome` |
| `--continue`, `-c` | Load most recent conversation | `--continue` |
| `--dangerously-skip-permissions` | Skip all permission prompts (use with caution) | `--dangerously-skip-permissions` |
| `--debug` | Enable debug mode with optional filtering | `--debug "api,mcp"` |
| `--disallowedTools` | Remove tools from model's context | `--disallowedTools Edit` |
| `--enable-lsp-logging` | Enable verbose LSP logging | `--enable-lsp-logging` |
| `--fallback-model` | Fallback model when default is overloaded | `--fallback-model sonnet` |
| `--fork-session` | Create new session ID when resuming | `--resume abc --fork-session` |
| `--ide` | Auto-connect to IDE on startup | `--ide` |
| `--include-partial-messages` | Include partial streaming events | `--include-partial-messages` |
| `--input-format` | Input format: text, stream-json | `--input-format stream-json` |
| `--json-schema` | Get validated JSON output matching schema | `--json-schema '{"type":"object",...}'` |
| `--max-turns` | Limit agentic turns in non-interactive mode | `--max-turns 3` |
| `--mcp-config` | Load MCP servers from JSON files | `--mcp-config ./mcp.json` |
| `--model` | Set model: sonnet, opus, or full name | `--model claude-sonnet-4-5-20250929` |
| `--no-chrome` | Disable Chrome integration | `--no-chrome` |
| `--output-format` | Output format: text, json, stream-json | `--output-format json` |
| `--permission-mode` | Start in specified permission mode | `--permission-mode plan` |
| `--permission-prompt-tool` | MCP tool for permission prompts | `--permission-prompt-tool mcp_auth` |
| `--plugin-dir` | Load plugins from directories | `--plugin-dir ./my-plugins` |
| `--print`, `-p` | Print response without interactive mode | `-p "query"` |
| `--resume`, `-r` | Resume session by ID or name | `--resume auth-refactor` |
| `--session-id` | Use specific session ID (UUID) | `--session-id "550e8400-..."` |
| `--setting-sources` | Setting sources: user, project, local | `--setting-sources user,project` |
| `--settings` | Path to settings JSON or JSON string | `--settings ./settings.json` |
| `--strict-mcp-config` | Only use MCP servers from --mcp-config | `--strict-mcp-config` |
| `--system-prompt` | Replace entire system prompt | `--system-prompt "You are a Python expert"` |
| `--system-prompt-file` | Load system prompt from file | `--system-prompt-file ./prompt.txt` |
| `--tools` | Specify available tools | `--tools "Bash,Edit,Read"` |
| `--verbose` | Enable verbose logging | `--verbose` |
| `--version`, `-v` | Output version number | `-v` |

---

## Agents Flag Format

Define custom subagents with JSON:

```bash
claude --agents '{
  "code-reviewer": {
    "description": "Expert code reviewer. Use proactively after code changes.",
    "prompt": "You are a senior code reviewer. Focus on code quality and security.",
    "tools": ["Read", "Grep", "Glob", "Bash"],
    "model": "sonnet"
  },
  "debugger": {
    "description": "Debugging specialist for errors and test failures.",
    "prompt": "You are an expert debugger. Analyze errors and provide fixes."
  }
}'
```

| Field | Required | Description |
|-------|----------|-------------|
| `description` | Yes | When the subagent should be invoked |
| `prompt` | Yes | System prompt guiding behavior |
| `tools` | No | Array of tools (inherits all if omitted) |
| `model` | No | Model: sonnet, opus, or haiku |

---

## System Prompt Flags

| Flag | Behavior | Modes | Use Case |
|------|----------|-------|----------|
| `--system-prompt` | **Replaces** entire default prompt | Both | Complete control over instructions |
| `--system-prompt-file` | **Replaces** with file contents | Print only | Load from versioned files |
| `--append-system-prompt` | **Appends** to default prompt | Both | Add instructions, keep defaults |

**Recommendation:** Use `--append-system-prompt` for most cases to preserve Claude Code's built-in capabilities.

```bash
# Keep defaults, add specific instructions
claude --append-system-prompt "Always use TypeScript and include JSDoc comments"

# Replace completely (loses default capabilities)
claude --system-prompt "You are a Python expert who only writes type-annotated code"

# Load from file (print mode only)
claude -p --system-prompt-file ./prompts/code-review.txt "Review this PR"
```

---

## Output Formats

For automation, use `--output-format` to get structured output:

| Format | Description | Use Case |
|--------|-------------|----------|
| `text` | Plain text (default) | Human-readable output |
| `json` | Complete JSON response | Programmatic parsing |
| `stream-json` | Streaming JSON events | Real-time processing |

**JSON output parsing:**
```bash
claude -p "List 3 Python tips" --output-format json | jq '.result'
```

**Stream JSON for real-time:**
```bash
claude -p "Explain async/await" --output-format stream-json
```

---

## See Also

- [subprocess-patterns.md](subprocess-patterns.md) - Python invocation patterns
- [hooks-examples.md](hooks-examples.md) - Hook configuration
- [json-extraction.md](json-extraction.md) - Parsing JSON from output

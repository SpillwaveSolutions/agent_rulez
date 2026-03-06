# RuleZ Plugin for OpenCode CLI

A TypeScript plugin that integrates the RuleZ policy engine with OpenCode CLI. Intercepts tool execution events and enforces YAML-defined rules (allow, deny, inject).

## Installation

### Option A: Plugin (TypeScript — recommended for OpenCode)

1. Copy the `rulez-plugin/` folder to your project or global plugin directory:

   ```bash
   # Project-level (per-repo)
   cp -r rulez-plugin/ .opencode/plugins/rulez-plugin/

   # Global (all repos)
   cp -r rulez-plugin/ ~/.config/opencode/plugins/rulez-plugin/
   ```

2. Ensure the `rulez` binary is in your PATH, or set:
   ```bash
   export RULEZ_BINARY_PATH=/path/to/rulez
   ```

3. Create `.claude/hooks.yaml` with your policy rules (RuleZ uses the same config path across all CLIs).

4. Restart OpenCode — the plugin auto-loads from the plugins directory.

### Option B: Command-based hooks (no TypeScript needed)

```bash
rulez opencode install --scope project
```

This writes hook entries directly into `.opencode/settings.json`. No plugin folder needed.

## How It Works

The plugin registers handlers for:

- **`tool.execute.before`** — Before any tool runs, RuleZ evaluates policy rules. If a rule denies, the tool call is blocked.
- **`tool.execute.after`** — After tool execution, RuleZ logs the event for audit purposes.

On each event, the plugin:
1. Builds a JSON payload with session ID, tool name, tool input, and working directory
2. Spawns `rulez opencode hook` and pipes the JSON on stdin
3. Reads the JSON response from stdout
4. If `continue: false`, throws an error to block the tool call
5. If `context` is present, injects it into the event

The plugin is **fail-open**: if the `rulez` subprocess fails or times out (5s), the tool call is allowed.

## Configuration

Policy rules are defined in `.claude/hooks.yaml` (same format across Claude Code, Gemini, Copilot, and OpenCode):

```yaml
version: "1.0"
rules:
  - name: block-force-push
    matchers:
      tools: ["Bash"]
      command_match: "git push.*--force"
    actions:
      block: true
```

See the [RuleZ documentation](../docs/) for full rule syntax.

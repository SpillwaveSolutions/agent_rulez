# Phase 26 Research: OpenCode CLI E2E Testing

## OpenCode Plugin Architecture

OpenCode uses a **plugin system** with TypeScript/JavaScript lifecycle callbacks loaded via Bun runtime embedded in the Go binary.

### Plugin Discovery
- **Local plugins:** `.opencode/plugins/<name>/index.ts` (auto-discovered per project)
- **Global plugins:** `~/.config/opencode/plugins/<name>/`
- **NPM plugins:** listed in `opencode.json` `"plugins"` section
- **Load order:** global config -> project config -> global plugins -> project plugins
- **Hooks execute sequentially** (not parallel)

### Plugin File Structure
```
.opencode/plugins/rulez-plugin/
├── index.ts          # Main plugin (default export)
├── package.json      # { "type": "module", "main": "./index.ts" }
└── tsconfig.json     # Optional
```

### Plugin Function Signature
```typescript
export default async function rulezPlugin({ project, directory, worktree, client, $ }) {
  return {
    async 'tool.execute.before'(ctx) { ... },
    async 'tool.execute.after'(ctx) { ... },
    async 'permission.asked'(ctx) { ... }
  };
}
```

## Available Lifecycle Events

| Event | RuleZ EventType | Description |
|-------|----------------|-------------|
| `tool.execute.before` | PreToolUse | Before tool invocation |
| `tool.execute.after` | PostToolUse | After tool execution |
| `session.created` | SessionStart | New session |
| `session.deleted` | SessionEnd | Session ended |
| `session.updated` | UserPromptSubmit | Session changed |
| `session.compacted` | PreCompact | Context compaction |
| `file.edited` | (mapped in adapter) | File changed |
| `permission.asked` | (custom) | Permission gate |

## How RuleZ Integrates

### Command-Based Hook Approach (simpler)
- `rulez opencode install --scope project` writes hook entries into `.opencode/settings.json`
- Each hook entry: `{ "type": "command", "command": "rulez opencode hook", "timeout": 5 }`
- At runtime: OpenCode pipes JSON to `rulez opencode hook` stdin
- RuleZ parses event via `adapters/opencode.rs`, evaluates rules, outputs JSON response on stdout
- Exit code 2 = deny (blocks the tool call)

### Plugin Approach (richer integration)
- TypeScript plugin in `.opencode/plugins/rulez-plugin/`
- Spawns `rulez opencode hook` via `Bun.spawn`
- Handles response: `continue: false` -> throw Error to block
- Fail-open on subprocess errors

### Hook Events Registered
From `opencode_install.rs` `OPENCODE_HOOK_EVENTS`:
- `file.edited`
- `tool.execute.before`
- `tool.execute.after`
- `session.updated`

## Tool Name Mapping

From `rulez/src/adapters/opencode.rs` `map_tool_name()`:

| OpenCode Name | Canonical (RuleZ) Name |
|--------------|----------------------|
| `bash` | `Bash` |
| `write` | `Write` |
| `edit` | `Edit` |
| `read` | `Read` |
| `glob` | `Glob` |
| `grep` | `Grep` |
| `task` | `Task` |
| `webfetch`/`fetch` | `WebFetch` |

## OpenCode Headless Mode
- `opencode --prompt "..." --format json` — headless with JSON output
- `opencode --session <id>` — attach to session
- `opencode --model provider/model` — select model
- No equivalent of `--dangerously-skip-permissions`

## RuleZ Codebase Files

| File | Purpose |
|------|---------|
| `rulez/src/adapters/opencode.rs` | Event parsing, tool name mapping, response translation |
| `rulez/src/opencode/dispatcher.rs` | Event dispatch through policy engine + audit |
| `rulez/src/opencode/config.rs` | Plugin config from settings.json |
| `rulez/src/cli/opencode_hook.rs` | Reads stdin JSON, calls adapter, emits response |
| `rulez/src/cli/opencode_install.rs` | Writes hook entries into .opencode/settings.json |

## Key Differences from Other CLIs

| Aspect | Claude Code | Copilot | Gemini | OpenCode |
|--------|------------|---------|--------|----------|
| Hook mechanism | Shell hooks | JSON hook config | Extensions | Plugin system / command hooks |
| Config path | `.claude/settings.json` | `.github/hooks/` | `.gemini/settings.json` | `.opencode/settings.json` |
| Headless flag | `-p` | `-p` | `--yolo` | `--prompt` |
| Auth | API key | GitHub OAuth | Google OAuth | Provider-specific |
| Deny signal | exit 2 | exit 2 | exit 2 | exit 2 (same) |

## E2E Test Strategy

Follows the established pattern from Phases 23-25:
1. **01-install** — `rulez opencode install` creates valid `.opencode/settings.json` (no live CLI needed)
2. **02-hook-fire** — Headless invocation triggers hook, verified via audit log
3. **03-deny** — Force push prompt blocked, verified via audit log
4. **04-inject** — Inject command writes marker file, verified via filesystem + audit log

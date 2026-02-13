# Phase 21: Copilot CLI Support and Copilot Hooks Support - Research

**Researched:** 2026-02-12
**Domain:** GitHub Copilot CLI, Copilot hooks, VS Code Chat/LM API
**Confidence:** MEDIUM

## Summary

Copilot CLI and Copilot hooks are official GitHub Copilot agent surfaces with a documented hooks system. Hooks are JSON files in `.github/hooks/*.json`, loaded from the repo default branch for coding agent and from the current working directory for Copilot CLI. Hook types include session start/end, prompt submission, pre/post tool use, and errors. The `preToolUse` hook can deny tool execution by returning a JSON decision, which is the key enforcement point for policy gating in CLI/agent workflows. Hooks run synchronously and must stay fast, emit valid JSON, and avoid logging secrets.

For VS Code integration, the standard approach is to ship a VS Code extension that contributes a Chat Participant. Chat participants support slash commands and can use the Language Model API (`vscode.lm`) for policy checks. The Language Model API supports only user/assistant messages, requires user consent, and should be called from a user-initiated action. Publishing to Marketplace is done with `vsce` and standard VS Code extension packaging practices.

There is no documented pre-acceptance hook for GitHub Copilot inline suggestions in the public VS Code API. The only confirmed interception point for policy enforcement is Copilot hooks (CLI/coding agent) via `preToolUse`. For editor completions, you may need to scope enforcement to chat and agent workflows unless a new API or GitHub Copilot-specific extension point is available.

**Primary recommendation:** Use Copilot hooks for CLI policy enforcement (preToolUse), and implement a VS Code Chat Participant that calls RuleZ via the Language Model API or backend for policy decisions; treat inline suggestion pre-acceptance as an open capability to validate.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| VS Code Chat Participant API (`vscode.chat`) | VS Code API | Implement Copilot Chat participant and slash commands | Official VS Code extensibility path for chat participants |
| VS Code Language Model API (`vscode.lm`) | VS Code API | Run policy checks or summarize policy in chat | Official API for Copilot models in extensions |
| GitHub Copilot CLI hooks | v1 schema | Enforce policy via `preToolUse` and audit via `postToolUse` | Official hook system for agents/CLI |
| GitHub Copilot CLI (`copilot` / `gh copilot`) | Public preview | Terminal agent surface for policy enforcement | Official CLI integration |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `@vscode/chat-extension-utils` | Latest | Simplify tool calling in chat participants | If tool-calling is needed for agentic flows |
| `@vscode/prompt-tsx` | Latest | Compose prompts with token-aware structure | If prompt construction needs dynamic sizing |
| `@vscode/vsce` | Latest | Package/publish VS Code extension | Required for Marketplace publishing |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| VS Code chat participant | GitHub App Copilot extension | Works across surfaces but lacks VS Code API access |
| Copilot hooks | Custom shell wrappers around `copilot` | More brittle, less integrated, no formal hook types |

**Installation:**
```bash
npm install -g @vscode/vsce
npm install @vscode/chat-extension-utils @vscode/prompt-tsx
```

## Architecture Patterns

### Recommended Project Structure
```
extensions/
├── copilot-rulez-vscode/     # VS Code extension
│   ├── package.json          # chatParticipants, commands
│   ├── src/                  # extension activation + handlers
│   └── resources/            # icons, bundled binaries
hooks/
├── .github/hooks/            # Copilot hooks JSON
└── scripts/                  # hook scripts calling RuleZ
```

### Pattern 1: Chat Participant + Policy Service
**What:** Chat participant request handler calls RuleZ (local binary or service) to validate or explain policy decisions and responds with markdown, buttons, or follow-ups.
**When to use:** COPILOT-01/02/03/04/06 (chat participant, LM checks, slash commands, context injection, audit).
**Example:**
```typescript
// Source: https://code.visualstudio.com/api/extension-guides/ai/chat
export function activate(context: vscode.ExtensionContext) {
  const participant = vscode.chat.createChatParticipant('rulez.participant', handler);
  participant.iconPath = vscode.Uri.joinPath(context.extensionUri, 'icon.png');
}

const handler: vscode.ChatRequestHandler = async (request, context, stream, token) => {
  if (request.command === 'validate') {
    // call RuleZ policy checker
  } else {
    // handle general policy queries
  }
};
```

### Pattern 2: Language Model API for Policy Checks
**What:** Use `vscode.lm.selectChatModels` and `sendRequest` to run a policy check prompt, respecting user-selected model and consent.
**When to use:** COPILOT-02 (inline chat policy checks), COPILOT-04 (inject context into prompt).
**Example:**
```typescript
// Source: https://code.visualstudio.com/api/extension-guides/ai/language-model
const [model] = await vscode.lm.selectChatModels({ vendor: 'copilot', family: 'gpt-4o' });
const messages = [
  vscode.LanguageModelChatMessage.User('Apply RuleZ policy checks to this request.'),
  vscode.LanguageModelChatMessage.User(request.prompt)
];
const response = await model.sendRequest(messages, {}, token);
```

### Pattern 3: Copilot Hooks for CLI Enforcement
**What:** Define `.github/hooks/*.json` with `preToolUse` to deny dangerous commands and `postToolUse` for audit logging; hook scripts call RuleZ with JSON input.
**When to use:** CLI and coding agent enforcement (COPILOT-05/06), especially where pre-acceptance is needed.
**Example:**
```json
// Source: https://docs.github.com/en/copilot/how-tos/copilot-cli/use-hooks
{
  "version": 1,
  "hooks": {
    "preToolUse": [
      {
        "type": "command",
        "bash": "./scripts/rulez-pretool.sh",
        "powershell": "./scripts/rulez-pretool.ps1",
        "cwd": "scripts",
        "timeoutSec": 10
      }
    ]
  }
}
```

### Anti-Patterns to Avoid
- **Hooking outside `.github/hooks/`**: Copilot only loads hooks from `.github/hooks/*.json` (CLI uses CWD).
- **Long-running hooks**: Hooks block agent execution; timeouts default to 30s and slow hooks degrade UX.
- **Assuming system messages in LM API**: VS Code LM API supports only user/assistant messages.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| CLI policy enforcement | Custom wrapper around `copilot` | Copilot hooks (`preToolUse`) | Official interception with allow/deny decisions |
| Chat participant plumbing | Manual tool-calling orchestration | `@vscode/chat-extension-utils` | Standard, tested chat tool flow |
| Extension publishing pipeline | Custom packaging scripts | `vsce` | Required tool for Marketplace publishing |

**Key insight:** Hooks and VS Code APIs are the sanctioned enforcement and integration points; custom wrappers are fragile and miss built-in lifecycle events.

## Common Pitfalls

### Pitfall 1: Hooks not executing
**What goes wrong:** Hooks never run in CLI/agent sessions.
**Why it happens:** JSON file not in `.github/hooks`, invalid JSON, or missing `version: 1`.
**How to avoid:** Place `*.json` in `.github/hooks/`, validate with `jq .`, include `"version": 1`.
**Warning signs:** No hook side effects, no log entries.

### Pitfall 2: Invalid hook output
**What goes wrong:** Hook output ignored or hook fails.
**Why it happens:** Output is multi-line or not valid JSON.
**How to avoid:** Use `jq -c` (Bash) or `ConvertTo-Json -Compress` (PowerShell).
**Warning signs:** "Invalid JSON output" warnings.

### Pitfall 3: Slow or blocking hooks
**What goes wrong:** Agent UI stalls; timeouts triggered.
**Why it happens:** Heavy work or network calls in hook scripts.
**How to avoid:** Keep hooks under ~5s, offload heavy work, set `timeoutSec` where needed.
**Warning signs:** Frequent hook timeouts, slow CLI response.

### Pitfall 4: Missing user consent for LM API
**What goes wrong:** LM requests fail or return no models.
**Why it happens:** `selectChatModels` invoked outside user action.
**How to avoid:** Trigger LM calls from explicit commands or chat requests.
**Warning signs:** `selectChatModels` returns empty list.

## Code Examples

Verified patterns from official sources:

### Copilot CLI tool permissions
```bash
# Source: https://docs.github.com/en/copilot/concepts/agents/about-copilot-cli
copilot --allow-all-tools --deny-tool 'shell(rm)' --deny-tool 'shell(git push)'
```

### Copilot CLI hooks pre-tool decision
```json
// Source: https://docs.github.com/en/copilot/reference/hooks-configuration
{
  "permissionDecision": "deny",
  "permissionDecisionReason": "Dangerous command detected"
}
```

### VS Code chat participant slash commands
```json
// Source: https://code.visualstudio.com/api/extension-guides/ai/chat
"contributes": {
  "chatParticipants": [
    {
      "id": "rulez.participant",
      "name": "rulez",
      "fullName": "RuleZ",
      "description": "Policy checks for Copilot",
      "commands": [
        { "name": "explain", "description": "Explain a policy decision" },
        { "name": "validate", "description": "Validate a proposed change" }
      ]
    }
  ]
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Ad-hoc CLI wrappers | Copilot hooks JSON + `preToolUse` | 2025-2026 | Standard, enforceable policy gating |
| Chat-only prompts | Chat Participant + LM API | 2025+ | Richer integration, tool calling, context control |

**Deprecated/outdated:**
- Custom CLI wrappers for policy enforcement in place of hooks: less reliable and missing official lifecycle events.

## Open Questions

1. **Pre-acceptance hook for Copilot inline suggestions**
   - What we know: VS Code exposes Chat Participant and LM APIs; Copilot hooks enforce tool usage in CLI/agent flows.
   - What's unclear: Any official VS Code or Copilot API to intercept acceptance of inline suggestions before insertion.
   - Recommendation: Validate with current VS Code/Copilot extension APIs and GitHub Copilot extensibility docs; treat as a risk item.

2. **Chat API "message attachments" for context injection**
   - What we know: Chat Participant can construct prompts and add references in responses.
   - What's unclear: Whether there is a formal attachment mechanism in chat requests beyond prompt construction.
   - Recommendation: Confirm via VS Code Chat API reference or sample code before planning COPILOT-04.

## Sources

### Primary (HIGH confidence)
- https://code.visualstudio.com/api/extension-guides/ai/chat - Chat Participant API (slash commands, handlers)
- https://code.visualstudio.com/api/extension-guides/ai/language-model - Language Model API (consent, models)
- https://docs.github.com/en/copilot/concepts/agents/coding-agent/about-hooks - Hook types and format
- https://docs.github.com/en/copilot/how-tos/copilot-cli/use-hooks - Hook locations and CLI behavior
- https://docs.github.com/en/copilot/reference/hooks-configuration - Hook input/output details
- https://docs.github.com/en/copilot/concepts/agents/about-copilot-cli - CLI modes, tool approval flags
- https://cli.github.com/manual/gh_copilot - `gh copilot` command behavior and options
- https://code.visualstudio.com/api/working-with-extensions/publishing-extension - Publishing and `vsce`

### Secondary (MEDIUM confidence)
- https://github.blog/changelog/2026-01-21-github-copilot-cli-plan-before-you-build-steer-as-you-go/ - CLI release notes

### Tertiary (LOW confidence)
- None

## Metadata

**Confidence breakdown:**
- Standard stack: MEDIUM - Official docs, but version details are not explicit
- Architecture: MEDIUM - Patterns derived from official APIs and hooks
- Pitfalls: HIGH - Directly documented in hook references and CLI docs

**Research date:** 2026-02-12
**Valid until:** 2026-03-12

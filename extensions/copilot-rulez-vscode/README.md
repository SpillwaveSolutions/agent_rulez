# RuleZ Copilot Chat Extension

Run RuleZ policy checks from GitHub Copilot chat using slash commands.

## Quick Start

1. Open this folder in VS Code.
2. Install dependencies:
   - `npm install`
3. Build the extension:
   - `npm run compile`
4. Press `F5` to launch the Extension Development Host.
5. Open Copilot Chat and run:
   - `/validate <your prompt>`
   - `/explain <your prompt>`

## Configuration

Configure via VS Code settings:

- `rulez.binaryPath`: Absolute path to the RuleZ binary. Leave empty to use `rulez` from PATH.
- `rulez.useLmSummary`: If enabled and a Copilot model is available, RuleZ decisions are summarized using the Language Model API.

## Behavior

- The participant sends a `UserPromptSubmit` event to the RuleZ binary via stdin and reads the JSON response from stdout.
- If `rulez` is not found and no custom `rulez.binaryPath` is configured, the extension falls back to `cch`.
- When the Language Model API is unavailable or no model is selected, the raw RuleZ decision and reason are returned.

## Known Limitations

- No pre-acceptance hooks for inline Copilot suggestions are available in the public VS Code API.
- RuleZ checks are performed only for Copilot chat requests via `/validate` and `/explain`.

## Troubleshooting

- If you see "RuleZ binary not found", set `rulez.binaryPath` or ensure `rulez` is on PATH.
- Use `/validate` to confirm your RuleZ configuration is reachable from VS Code.

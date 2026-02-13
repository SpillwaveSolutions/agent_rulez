# RuleZ VS Code Copilot Chat Extension

This extension adds a RuleZ chat participant to GitHub Copilot Chat so you can validate or explain policy decisions from inside VS Code.

## Installation (Development)

1. Open `extensions/copilot-rulez-vscode` in VS Code.
2. Install dependencies:
   - `npm install`
3. Compile:
   - `npm run compile`
4. Press `F5` to launch the Extension Development Host.

## Configuration

Settings are available under **Settings → Extensions → RuleZ Copilot**:

- `rulez.binaryPath`: Absolute path to the RuleZ binary. Leave empty to use `rulez` from PATH.
- `rulez.useLmSummary`: If enabled, the Language Model API is used to summarize RuleZ decisions (only when a model is available and the request is user-initiated).

## Usage

Open Copilot Chat and run either command:

- `/validate <prompt>`: Validate a proposed change against RuleZ policies.
- `/explain <prompt>`: Explain the RuleZ policy decision.

The extension sends a `UserPromptSubmit` event to the RuleZ binary via stdin and renders the JSON response in chat.

## Example

```
/validate Add a new script that copies files into /tmp before deployment.
```

## Limitations

- There is no public VS Code API for intercepting Copilot inline suggestions before acceptance.
- Policy enforcement is currently scoped to chat commands (Copilot CLI hooks handle CLI/agent enforcement separately).

## Troubleshooting

- If the chat response says the RuleZ binary is missing, set `rulez.binaryPath` or ensure `rulez` is on PATH.
- If `rulez.useLmSummary` is enabled but no summary appears, Copilot model consent may be required or no model is available.

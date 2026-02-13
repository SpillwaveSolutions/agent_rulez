# Copilot CLI Hooks

This guide documents how RuleZ integrates with GitHub Copilot CLI hooks and how to diagnose hook installation issues.

## Hook File Locations

Copilot CLI loads hooks from `.github/hooks/*.json` in the project root. Each JSON file must include `"version": 1` and a `hooks` object mapping event names to handler arrays.

Supported events:

- `preToolUse` — called before a tool executes; can allow or deny
- `postToolUse` — called after a tool executes; used for audit logging

## Install Copilot Hooks

Use the install command to generate hook files that invoke the RuleZ Copilot hook runner:

```
cch copilot install
```

This creates `.github/hooks/rulez.json` with entries for `preToolUse` and `postToolUse`.

To specify a custom binary path:

```
cch copilot install --binary /path/to/cch
```

To print the JSON snippet without writing:

```
cch copilot install --print
```

## Diagnostics: `cch copilot doctor`

Run the doctor command to validate hook installation:

```
cch copilot doctor
```

For machine-readable output:

```
cch copilot doctor --json
```

### Output Interpretation

- **OK**: CCH hook commands were found in the hook file.
- **MISSING**: The hooks directory or hook file was not found.
- **WARN**: Hooks were found, but none reference `cch copilot hook` (likely misconfigured or outdated).
- **ERROR**: The file exists but could not be read or parsed as JSON.

## Hook Response Format

The Copilot hook runner reads JSON from stdin and emits a single-line JSON response:

### Allow

```json
{"permissionDecision":"allow"}
```

### Deny

```json
{"permissionDecision":"deny","permissionDecisionReason":"Blocked by policy: dangerous command detected"}
```

## Troubleshooting

### Missing hooks directory

If `cch copilot doctor` reports no hooks directory, run `cch copilot install` to create `.github/hooks/rulez.json`.

### Hooks present but misconfigured

If hooks exist but do not reference `cch copilot hook`, the hook entries may be pointing at a different tool or an outdated binary. Update the `bash` or `powershell` field to invoke `cch copilot hook`.

### Outdated CCH binary

If diagnostics warn that hook commands reference `cch` without `copilot hook`, the binary may be outdated.

1. Confirm Copilot subcommands are available:
   ```
   cch --help
   ```
   Look for `copilot` in the subcommand list.
2. If missing, update your PATH or reinstall the latest binary:
   ```
   cargo install --path .
   ```
3. Re-run the installer to refresh hooks:
   ```
   cch copilot install
   ```

### Hook not executing

Ensure:
1. The hook file is in `.github/hooks/` (not a subdirectory).
2. The file is valid JSON with `"version": 1`.
3. The `type` is `"command"`.
4. The `bash` or `powershell` field points to the correct `cch` binary.

## Wrapper Scripts

The installer also creates wrapper scripts in `scripts/copilot/`:

- `rulez-pretool.sh` — Bash wrapper that pipes stdin to `cch copilot hook`
- `rulez-pretool.ps1` — PowerShell wrapper for Windows

These can be referenced directly in hook files if you prefer script-based invocation over direct binary calls.

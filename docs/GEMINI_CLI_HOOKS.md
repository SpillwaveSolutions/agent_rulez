# Gemini CLI Hooks

This guide documents how RuleZ integrates with Gemini CLI hooks and how to diagnose hook installation issues.

## Settings Locations

Gemini CLI loads hook settings from multiple scopes, in precedence order:

- Project: `.gemini/settings.json`
- User: `~/.gemini/settings.json`
- System (platform-dependent):
  - macOS: `/Library/Application Support/Gemini/settings.json` or `/etc/gemini/settings.json`
  - Linux: `/etc/gemini/settings.json`
  - Windows: `%ProgramData%\Gemini\settings.json`

Gemini extensions can also provide hooks in:

- `~/.gemini/extensions/<extension>/hooks/hooks.json`
- `~/.gemini/hooks/*.json`

## Install Gemini Hooks

Use the install command to generate or merge hook settings that invoke the RuleZ Gemini hook runner:

```
cch gemini install
```

To target a different scope:

```
cch gemini install --scope user
cch gemini install --scope system
```

To print the JSON snippet without writing:

```
cch gemini install --print
```

## Diagnostics: `cch gemini doctor`

Run the doctor command to validate settings and hook installation across all scopes:

```
cch gemini doctor
```

For machine-readable output:

```
cch gemini doctor --json
```

### Output Interpretation

- **OK**: CCH hook commands were found in the scope or hook file.
- **MISSING**: The settings file or hook file was not found.
- **WARN**: Hooks were found, but none reference `cch` (likely misconfigured).
- **ERROR**: The file exists but could not be read or parsed as JSON.

## Troubleshooting

### Missing hooks in a scope

If a scope is marked **MISSING**, ensure the settings file exists and contains Gemini hook entries.
Example (trimmed to essentials):

```json
{
  "hooks": {
    "BeforeTool": [
      {
        "matcher": ".*",
        "hooks": [
          { "type": "command", "command": "/path/to/cch gemini hook", "timeout": 5 }
        ]
      }
    ]
  }
}
```

Re-run the installation process you used previously (project settings, user settings, or extension install) to restore the hook entries.

### Hooks present but misconfigured

If a scope is **WARN**, hooks exist but do not reference the `cch` command. Verify the hook command path and that the `type` is `command`. Update the entry to point at the correct `cch` binary.

### Outdated CCH binary

If diagnostics warn that hook commands reference `cch` without `gemini hook`, the binary on your PATH may be outdated.

1. Confirm Gemini subcommands are available:
   ```
   cch --help
   ```
   Look for `gemini` in the subcommand list.
2. If missing, update your PATH or reinstall the latest binary:
   ```
   cargo install --path .
   ```
3. Re-run the installer to refresh settings:
   ```
   cch gemini install
   ```

### Extension hook issues

If extension hook files are missing or misconfigured:

1. Confirm the extension is installed under `~/.gemini/extensions`.
2. Check for `hooks/hooks.json` inside the extension folder.
3. If you are using shared hooks, ensure `~/.gemini/hooks` contains valid JSON hook files.

After updating any hook files, re-run `cch gemini doctor` to verify.

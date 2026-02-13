use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const COPILOT_HOOK_EVENTS: [&str; 2] = ["preToolUse", "postToolUse"];

#[derive(Debug, Serialize, Deserialize, Default)]
struct CopilotHooksFile {
    #[serde(default)]
    version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hooks: Option<std::collections::HashMap<String, Vec<CopilotHookEntry>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct CopilotHookEntry {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    hook_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    powershell: Option<String>,
    #[serde(rename = "timeoutSec", skip_serializing_if = "Option::is_none")]
    timeout_sec: Option<u32>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}

pub async fn run(binary_path: Option<String>, print: bool) -> Result<()> {
    let cch_path = resolve_binary_path(binary_path)?;
    let hook_command = format!("{} copilot hook", cch_path.display());

    if print {
        let snippet = build_snippet(&hook_command);
        let output = serde_json::to_string_pretty(&snippet)?;
        println!("{}", output);
        return Ok(());
    }

    let hooks_dir = hooks_dir()?;
    let hooks_path = hooks_dir.join("rulez.json");

    println!("Installing Copilot hooks...\n");
    println!("  Binary: {}", cch_path.display());
    println!("  Hooks file: {}", hooks_path.display());
    println!();

    let mut hooks_file = load_hooks_file(&hooks_path)?;
    hooks_file.version = 1;
    let hooks = hooks_file
        .hooks
        .get_or_insert_with(std::collections::HashMap::new);
    let new_entry = build_hook_entry(&hook_command);

    for event in &COPILOT_HOOK_EVENTS {
        let entries = hooks.entry((*event).to_string()).or_default();
        let cleaned = remove_cch_hooks(entries);
        *entries = cleaned;
        entries.push(new_entry.clone());
    }

    save_hooks_file(&hooks_path, &hooks_file)?;

    // Create wrapper scripts
    create_wrapper_scripts(&hooks_dir, &cch_path)?;

    println!("✓ Copilot hooks installed successfully!\n");
    println!("Hook registered for events:");
    for event in &COPILOT_HOOK_EVENTS {
        println!("  • {}", event);
    }
    println!();
    println!("To verify installation:");
    println!("  cch copilot doctor");
    println!();
    println!("To print snippet for docs:");
    println!("  cch copilot install --print");

    Ok(())
}

fn build_snippet(command: &str) -> CopilotHooksFile {
    let mut hooks = std::collections::HashMap::new();
    let entry = build_hook_entry(command);

    for event in &COPILOT_HOOK_EVENTS {
        hooks.insert((*event).to_string(), vec![entry.clone()]);
    }

    CopilotHooksFile {
        version: 1,
        hooks: Some(hooks),
    }
}

fn build_hook_entry(command: &str) -> CopilotHookEntry {
    // The bash/powershell scripts pipe stdin to the cch binary
    CopilotHookEntry {
        hook_type: Some("command".to_string()),
        bash: Some(command.to_string()),
        powershell: Some(command.to_string()),
        timeout_sec: Some(10),
        extra: std::collections::HashMap::new(),
    }
}

fn remove_cch_hooks(entries: &[CopilotHookEntry]) -> Vec<CopilotHookEntry> {
    entries
        .iter()
        .filter(|entry| !is_cch_hook(entry))
        .cloned()
        .collect()
}

fn is_cch_hook(hook: &CopilotHookEntry) -> bool {
    let bash_is_cch = hook
        .bash
        .as_deref()
        .map(|cmd| cmd.contains("cch"))
        .unwrap_or(false);
    let ps_is_cch = hook
        .powershell
        .as_deref()
        .map(|cmd| cmd.contains("cch"))
        .unwrap_or(false);
    bash_is_cch || ps_is_cch
}

fn hooks_dir() -> Result<PathBuf> {
    let cwd = std::env::current_dir().context("Failed to determine current directory")?;
    Ok(cwd.join(".github").join("hooks"))
}

fn load_hooks_file(path: &Path) -> Result<CopilotHooksFile> {
    if path.exists() {
        let content = fs::read_to_string(path).context("Failed to read hooks file")?;
        let hooks_file: CopilotHooksFile =
            serde_json::from_str(&content).context("Failed to parse hooks file")?;
        Ok(hooks_file)
    } else {
        Ok(CopilotHooksFile::default())
    }
}

fn save_hooks_file(path: &Path, hooks_file: &CopilotHooksFile) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).context("Failed to create hooks directory")?;
        }
    }

    let content =
        serde_json::to_string_pretty(hooks_file).context("Failed to serialize hooks file")?;
    fs::write(path, content).context("Failed to write hooks file")?;
    Ok(())
}

fn create_wrapper_scripts(hooks_dir: &Path, _cch_path: &Path) -> Result<()> {
    let scripts_dir = hooks_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("scripts")
        .join("copilot");
    if !scripts_dir.exists() {
        fs::create_dir_all(&scripts_dir).context("Failed to create scripts directory")?;
    }

    let bash_script = r"#!/usr/bin/env bash
# RuleZ Copilot hook wrapper — forwards stdin to cch copilot hook
set -euo pipefail
exec cch copilot hook
";

    let ps_script = r"# RuleZ Copilot hook wrapper — forwards stdin to cch copilot hook
$input = [Console]::In.ReadToEnd()
$input | cch copilot hook
";

    let bash_path = scripts_dir.join("rulez-pretool.sh");
    let ps_path = scripts_dir.join("rulez-pretool.ps1");

    fs::write(&bash_path, bash_script).context("Failed to write bash script")?;
    fs::write(&ps_path, ps_script).context("Failed to write PowerShell script")?;

    // Make bash script executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&bash_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&bash_path, perms)?;
    }

    Ok(())
}

fn resolve_binary_path(explicit_path: Option<String>) -> Result<PathBuf> {
    if let Some(path) = explicit_path {
        let path_buf = PathBuf::from(&path);
        if path_buf.exists() {
            return path_buf
                .canonicalize()
                .context("Failed to resolve binary path");
        }
        anyhow::bail!("Specified binary not found: {}", path);
    }

    if let Ok(output) = std::process::Command::new("which").arg("cch").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Ok(PathBuf::from(path));
            }
        }
    }

    let local = PathBuf::from("./target/release/cch");
    if local.exists() {
        return Ok(local.canonicalize()?);
    }

    let debug = PathBuf::from("./target/debug/cch");
    if debug.exists() {
        return Ok(debug.canonicalize()?);
    }

    anyhow::bail!(
        "Could not find CCH binary. Either:\n  \
        1. Install globally: cargo install --path .\n  \
        2. Build locally: cargo build --release\n  \
        3. Specify path: cch copilot install --binary /path/to/cch"
    );
}

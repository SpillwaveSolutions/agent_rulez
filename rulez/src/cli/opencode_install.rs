use anyhow::{Context, Result};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const OPENCODE_HOOK_EVENTS: [&str; 4] = [
    "file.edited",
    "tool.execute.before",
    "tool.execute.after",
    "session.updated",
];

#[derive(Debug, Clone, Copy, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum Scope {
    Project,
    User,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct OpenCodeConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hooks: Option<HashMap<String, Vec<OpenCodeHookEntry>>>,
    #[serde(flatten)]
    other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct OpenCodeHookEntry {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    hook_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<u32>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

pub async fn run(scope: Scope, binary_path: Option<String>, print: bool) -> Result<()> {
    let cch_path = resolve_binary_path(binary_path)?;
    let hook_command = format!("{} opencode hook", cch_path.display());

    if print {
        let snippet = build_snippet(&hook_command);
        let output = serde_json::to_string_pretty(&snippet)?;
        println!("{}", output);
        return Ok(());
    }

    let config_path = config_path(scope)?;

    println!(
        "Installing OpenCode hooks...
"
    );
    println!("  Binary: {}", cch_path.display());
    println!("  Config: {}", config_path.display());
    println!("  Scope: {}", scope_name(scope));
    println!();

    let mut config = load_config(&config_path)?;
    let hooks = config.hooks.get_or_insert_with(HashMap::new);
    let new_entry = OpenCodeHookEntry {
        hook_type: Some("command".to_string()),
        command: Some(hook_command.clone()),
        timeout: Some(5),
        extra: HashMap::new(),
    };

    for event in &OPENCODE_HOOK_EVENTS {
        let entries = hooks.entry((*event).to_string()).or_default();
        let cleaned = remove_cch_hooks(entries);
        *entries = cleaned;
        entries.push(new_entry.clone());
    }

    save_config(&config_path, &config)?;

    println!(
        "✓ OpenCode hooks installed successfully!
"
    );
    println!("Hook registered for events:");
    for event in &OPENCODE_HOOK_EVENTS {
        println!("  • {}", event);
    }
    println!();
    println!("To print snippet for docs:");
    println!("  cch opencode install --print");

    Ok(())
}

fn build_snippet(command: &str) -> OpenCodeConfig {
    let mut hooks = HashMap::new();
    let entry = OpenCodeHookEntry {
        hook_type: Some("command".to_string()),
        command: Some(command.to_string()),
        timeout: Some(5),
        extra: HashMap::new(),
    };

    for event in &OPENCODE_HOOK_EVENTS {
        hooks.insert((*event).to_string(), vec![entry.clone()]);
    }

    OpenCodeConfig {
        hooks: Some(hooks),
        other: HashMap::new(),
    }
}

fn remove_cch_hooks(entries: &[OpenCodeHookEntry]) -> Vec<OpenCodeHookEntry> {
    entries
        .iter()
        .filter(|hook| !is_cch_hook(hook))
        .cloned()
        .collect()
}

fn is_cch_hook(hook: &OpenCodeHookEntry) -> bool {
    if let Some(hook_type) = hook.hook_type.as_deref() {
        if hook_type != "command" {
            return false;
        }
    }

    hook.command
        .as_deref()
        .map(|command| command.contains("cch"))
        .unwrap_or(false)
}

fn config_path(scope: Scope) -> Result<PathBuf> {
    match scope {
        Scope::Project => {
            let cwd = std::env::current_dir().context("Failed to determine current directory")?;
            Ok(cwd.join(".opencode").join("settings.json"))
        }
        Scope::User => {
            let home = dirs::home_dir().context("Could not determine home directory")?;
            Ok(home
                .join(".config")
                .join("opencode")
                .join("plugins")
                .join("rulez-plugin")
                .join("settings.json"))
        }
    }
}

fn scope_name(scope: Scope) -> &'static str {
    match scope {
        Scope::Project => "project",
        Scope::User => "user",
    }
}

fn load_config(path: &Path) -> Result<OpenCodeConfig> {
    if path.exists() {
        let content = fs::read_to_string(path).context("Failed to read config file")?;
        let config: OpenCodeConfig =
            serde_json::from_str(&content).context("Failed to parse config file")?;
        Ok(config)
    } else {
        Ok(OpenCodeConfig::default())
    }
}

fn save_config(path: &Path, config: &OpenCodeConfig) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }
    }

    let content = serde_json::to_string_pretty(config).context("Failed to serialize config")?;
    fs::write(path, content).context("Failed to write config file")?;
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

    anyhow::bail!("Could not find CCH binary. Build locally or specify path with --binary");
}

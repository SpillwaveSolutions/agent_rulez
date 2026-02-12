use anyhow::{Context, Result};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const GEMINI_HOOK_EVENTS: [&str; 11] = [
    "BeforeTool",
    "AfterTool",
    "BeforeAgent",
    "AfterAgent",
    "BeforeModel",
    "AfterModel",
    "BeforeToolSelection",
    "SessionStart",
    "SessionEnd",
    "Notification",
    "PreCompact",
];

#[derive(Debug, Clone, Copy, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum Scope {
    Project,
    User,
    System,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct GeminiSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hooks: Option<HashMap<String, Vec<GeminiMatcherEntry>>>,
    #[serde(flatten)]
    other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct GeminiMatcherEntry {
    #[serde(rename = "matcher", skip_serializing_if = "Option::is_none")]
    matcher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hooks: Option<Vec<GeminiHookCommand>>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct GeminiHookCommand {
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
    let hook_command = format!("{} gemini hook", cch_path.display());

    if print {
        let snippet = build_snippet(&hook_command);
        let output = serde_json::to_string_pretty(&snippet)?;
        println!("{}", output);
        return Ok(());
    }

    let settings_path = settings_path(scope)?;

    println!("Installing Gemini hooks...\n");
    println!("  Binary: {}", cch_path.display());
    println!("  Settings: {}", settings_path.display());
    println!("  Scope: {}", scope_name(scope));
    println!();

    let mut settings = load_settings(&settings_path)?;
    let hooks = settings.hooks.get_or_insert_with(HashMap::new);
    let new_entry = build_hook_entry(&hook_command);

    for event in GEMINI_HOOK_EVENTS.iter() {
        let entries = hooks.entry((*event).to_string()).or_default();
        let cleaned = remove_cch_hooks(entries);
        *entries = cleaned;
        entries.push(new_entry.clone());
    }

    save_settings(&settings_path, &settings)?;

    println!("✓ Gemini hooks installed successfully!\n");
    println!("Hook registered for events:");
    for event in GEMINI_HOOK_EVENTS.iter() {
        println!("  • {}", event);
    }
    println!();
    println!("To verify installation:");
    println!("  cch gemini doctor");
    println!();
    println!("To print snippet for docs:");
    println!("  cch gemini install --print");

    Ok(())
}

fn build_snippet(command: &str) -> GeminiSettings {
    let mut hooks = HashMap::new();
    let entry = build_hook_entry(command);

    for event in GEMINI_HOOK_EVENTS.iter() {
        hooks.insert((*event).to_string(), vec![entry.clone()]);
    }

    GeminiSettings {
        hooks: Some(hooks),
        other: HashMap::new(),
    }
}

fn build_hook_entry(command: &str) -> GeminiMatcherEntry {
    GeminiMatcherEntry {
        matcher: Some(".*".to_string()),
        hooks: Some(vec![GeminiHookCommand {
            hook_type: Some("command".to_string()),
            command: Some(command.to_string()),
            timeout: Some(5),
            extra: HashMap::new(),
        }]),
        extra: HashMap::new(),
    }
}

fn remove_cch_hooks(entries: &[GeminiMatcherEntry]) -> Vec<GeminiMatcherEntry> {
    entries
        .iter()
        .filter_map(|entry| {
            let hooks = entry.hooks.as_ref()?;
            let kept: Vec<GeminiHookCommand> = hooks
                .iter()
                .filter(|hook| !is_cch_hook(hook))
                .cloned()
                .collect();

            if kept.is_empty() {
                return None;
            }

            let mut updated = entry.clone();
            updated.hooks = Some(kept);
            Some(updated)
        })
        .collect()
}

fn is_cch_hook(hook: &GeminiHookCommand) -> bool {
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

fn settings_path(scope: Scope) -> Result<PathBuf> {
    match scope {
        Scope::Project => {
            let cwd = std::env::current_dir().context("Failed to determine current directory")?;
            Ok(cwd.join(".gemini").join("settings.json"))
        }
        Scope::User => {
            let home = dirs::home_dir().context("Could not determine home directory")?;
            Ok(home.join(".gemini").join("settings.json"))
        }
        Scope::System => Ok(system_settings_path().0),
    }
}

fn scope_name(scope: Scope) -> &'static str {
    match scope {
        Scope::Project => "project",
        Scope::User => "user",
        Scope::System => "system",
    }
}

fn system_settings_path() -> (PathBuf, Vec<String>) {
    if let Ok(path) = std::env::var("GEMINI_SYSTEM_SETTINGS_PATH") {
        let path_buf = PathBuf::from(&path);
        return (path_buf.clone(), vec![path]);
    }

    let candidates = system_settings_candidates();
    let existing = candidates.iter().find(|path| path.exists());
    let selected = existing.cloned().unwrap_or_else(|| candidates[0].clone());
    let checked = candidates
        .into_iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    (selected, checked)
}

fn system_settings_candidates() -> Vec<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        vec![
            PathBuf::from("/Library/Application Support/Gemini/settings.json"),
            PathBuf::from("/etc/gemini/settings.json"),
        ]
    }

    #[cfg(target_os = "linux")]
    {
        vec![PathBuf::from("/etc/gemini/settings.json")]
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(program_data) = std::env::var("ProgramData") {
            vec![PathBuf::from(program_data)
                .join("Gemini")
                .join("settings.json")]
        } else {
            vec![PathBuf::from("C:\\ProgramData\\Gemini\\settings.json")]
        }
    }
}

fn load_settings(path: &Path) -> Result<GeminiSettings> {
    if path.exists() {
        let content = fs::read_to_string(path).context("Failed to read settings file")?;
        let settings: GeminiSettings =
            serde_json::from_str(&content).context("Failed to parse settings file")?;
        Ok(settings)
    } else {
        Ok(GeminiSettings::default())
    }
}

fn save_settings(path: &Path, settings: &GeminiSettings) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).context("Failed to create settings directory")?;
        }
    }

    let content = serde_json::to_string_pretty(settings).context("Failed to serialize settings")?;
    fs::write(path, content).context("Failed to write settings file")?;
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
        3. Specify path: cch gemini install --binary /path/to/cch"
    );
}

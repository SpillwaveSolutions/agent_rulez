//! RuleZ Install Command - Register RuleZ with Claude Code
//!
//! Adds RuleZ hook configuration to Claude Code settings.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Claude Code settings structure (partial)
#[derive(Debug, Serialize, Deserialize, Default)]
struct ClaudeSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hooks: Option<HooksConfig>,
    #[serde(flatten)]
    other: HashMap<String, serde_json::Value>,
}

/// Hooks configuration in Claude Code settings.
///
/// Claude Code expects PascalCase event keys with a nested matcher/hooks structure:
/// ```json
/// {
///   "hooks": {
///     "PreToolUse": [
///       { "matcher": "*", "hooks": [{ "type": "command", "command": "/path/to/rulez", "timeout": 5 }] }
///     ]
///   }
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct HooksConfig {
    #[serde(rename = "PreToolUse", default, skip_serializing_if = "Vec::is_empty")]
    pre_tool_use: Vec<MatcherEntry>,
    #[serde(rename = "PostToolUse", default, skip_serializing_if = "Vec::is_empty")]
    post_tool_use: Vec<MatcherEntry>,
    #[serde(rename = "Stop", default, skip_serializing_if = "Vec::is_empty")]
    stop: Vec<MatcherEntry>,
    #[serde(
        rename = "SessionStart",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    session_start: Vec<MatcherEntry>,
}

/// A matcher entry groups a glob pattern with its hook commands
#[derive(Debug, Serialize, Deserialize, Clone)]
struct MatcherEntry {
    matcher: String,
    hooks: Vec<HookCommand>,
}

/// Individual hook command within a matcher entry
#[derive(Debug, Serialize, Deserialize, Clone)]
struct HookCommand {
    #[serde(rename = "type")]
    hook_type: String,
    command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<u32>,
}

/// Installation scope
#[derive(Debug, Clone, Copy)]
pub enum Scope {
    /// Install for current project only (.claude/settings.json)
    Project,
    /// Install globally (~/.claude/settings.json)
    Global,
}

/// Run the install command
pub async fn run(scope: Scope, binary_path: Option<String>) -> Result<()> {
    let rulez_path = resolve_binary_path(binary_path)?;
    let settings_path = get_settings_path(scope)?;

    println!("Installing RuleZ hook...\n");
    println!("  Binary: {}", rulez_path.display());
    println!("  Settings: {}", settings_path.display());
    println!("  Scope: {}", scope_name(scope));
    println!();

    // Verify hooks.yaml exists for project scope
    if matches!(scope, Scope::Project) {
        let hooks_yaml = Path::new(".claude/hooks.yaml");
        if !hooks_yaml.exists() {
            println!("⚠️  No hooks.yaml found. Run 'rulez init' first.");
            println!("   Creating default configuration...\n");
            super::init::run(false, false).await?;
            println!();
        }
    }

    // Load or create settings
    let mut settings = load_settings(&settings_path)?;

    // Build hook command
    let hook_command = format!("{}", rulez_path.display());

    // Create the matcher entry with nested hook command
    let matcher_entry = MatcherEntry {
        matcher: "*".to_string(),
        hooks: vec![HookCommand {
            hook_type: "command".to_string(),
            command: hook_command.clone(),
            timeout: Some(5),
        }],
    };

    // Get or create hooks config
    let hooks = settings.hooks.get_or_insert_with(HooksConfig::default);

    // Check if already installed (look inside nested hooks[].command)
    let already_installed = hooks.pre_tool_use.iter().any(|m| {
        m.hooks
            .iter()
            .any(|h| h.command.contains("rulez") || h.command.contains("cch"))
    });

    if already_installed {
        println!("✓ RuleZ is already installed");
        println!("  To reinstall, first run 'rulez uninstall'");
        return Ok(());
    }

    // Add RuleZ to all hook events
    hooks.pre_tool_use.push(matcher_entry.clone());
    hooks.post_tool_use.push(matcher_entry.clone());
    hooks.stop.push(matcher_entry.clone());
    hooks.session_start.push(matcher_entry);

    // Save settings
    save_settings(&settings_path, &settings)?;

    println!("✓ RuleZ installed successfully!\n");
    println!("Hook registered for events:");
    println!("  • PreToolUse");
    println!("  • PostToolUse");
    println!("  • Stop");
    println!("  • SessionStart");
    println!();
    println!("To verify installation:");
    println!("  rulez validate");
    println!();
    println!("To uninstall:");
    println!("  rulez uninstall");

    Ok(())
}

/// Resolve the RuleZ binary path
fn resolve_binary_path(explicit_path: Option<String>) -> Result<PathBuf> {
    if let Some(path) = explicit_path {
        let p = PathBuf::from(&path);
        if p.exists() {
            return p.canonicalize().context("Failed to resolve binary path");
        }
        anyhow::bail!("Specified binary not found: {}", path);
    }

    // Try to find rulez in PATH
    if let Ok(output) = std::process::Command::new("which").arg("rulez").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Ok(PathBuf::from(path));
            }
        }
    }

    // Try current directory
    let local = PathBuf::from("./target/release/rulez");
    if local.exists() {
        return Ok(local.canonicalize()?);
    }

    // Try debug build
    let debug = PathBuf::from("./target/debug/rulez");
    if debug.exists() {
        return Ok(debug.canonicalize()?);
    }

    anyhow::bail!(
        "Could not find RuleZ binary. Either:\n  \
        1. Install globally: cargo install --path .\n  \
        2. Build locally: cargo build --release\n  \
        3. Specify path: rulez install --binary /path/to/rulez"
    );
}

/// Get the settings file path based on scope
fn get_settings_path(scope: Scope) -> Result<PathBuf> {
    match scope {
        Scope::Project => Ok(PathBuf::from(".claude/settings.json")),
        Scope::Global => {
            let home = dirs::home_dir().context("Could not determine home directory")?;
            Ok(home.join(".claude").join("settings.json"))
        }
    }
}

/// Get scope display name
fn scope_name(scope: Scope) -> &'static str {
    match scope {
        Scope::Project => "project",
        Scope::Global => "global",
    }
}

/// Load settings from file or create default
fn load_settings(path: &Path) -> Result<ClaudeSettings> {
    if path.exists() {
        let content = fs::read_to_string(path).context("Failed to read settings file")?;
        let settings: ClaudeSettings =
            serde_json::from_str(&content).context("Failed to parse settings file")?;
        Ok(settings)
    } else {
        Ok(ClaudeSettings::default())
    }
}

/// Save settings to file
fn save_settings(path: &Path, settings: &ClaudeSettings) -> Result<()> {
    // Create parent directory if needed
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).context("Failed to create settings directory")?;
        }
    }

    let content = serde_json::to_string_pretty(settings).context("Failed to serialize settings")?;
    fs::write(path, content).context("Failed to write settings file")?;

    Ok(())
}

/// Uninstall RuleZ from Claude Code settings
pub async fn uninstall(scope: Scope) -> Result<()> {
    let settings_path = get_settings_path(scope)?;

    println!("Uninstalling RuleZ...\n");

    if !settings_path.exists() {
        println!("No settings file found at: {}", settings_path.display());
        return Ok(());
    }

    let mut settings = load_settings(&settings_path)?;

    // Helper: detect both old "cch" and new "rulez" hook commands
    let is_rulez_hook = |m: &MatcherEntry| {
        m.hooks
            .iter()
            .any(|h| h.command.contains("rulez") || h.command.contains("cch"))
    };

    if let Some(hooks) = &mut settings.hooks {
        let before = hooks.pre_tool_use.len()
            + hooks.post_tool_use.len()
            + hooks.stop.len()
            + hooks.session_start.len();

        hooks.pre_tool_use.retain(|m| !is_rulez_hook(m));
        hooks.post_tool_use.retain(|m| !is_rulez_hook(m));
        hooks.stop.retain(|m| !is_rulez_hook(m));
        hooks.session_start.retain(|m| !is_rulez_hook(m));

        let after = hooks.pre_tool_use.len()
            + hooks.post_tool_use.len()
            + hooks.stop.len()
            + hooks.session_start.len();

        if before == after {
            println!("RuleZ was not installed");
            return Ok(());
        }

        // Clean up empty hooks config
        if hooks.pre_tool_use.is_empty()
            && hooks.post_tool_use.is_empty()
            && hooks.stop.is_empty()
            && hooks.session_start.is_empty()
        {
            settings.hooks = None;
        }
    } else {
        println!("RuleZ was not installed");
        return Ok(());
    }

    save_settings(&settings_path, &settings)?;
    println!("✓ RuleZ uninstalled successfully");

    Ok(())
}

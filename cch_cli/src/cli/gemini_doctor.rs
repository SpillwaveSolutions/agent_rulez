use anyhow::{Context, Result};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum DoctorStatus {
    Installed,
    Missing,
    Misconfigured,
    Error,
}

#[derive(Debug, Serialize)]
struct DoctorReport {
    scopes: Vec<ScopeReport>,
    extensions: ExtensionsReport,
    summary: SummaryReport,
}

#[derive(Debug, Serialize)]
struct ScopeReport {
    scope: String,
    path: String,
    exists: bool,
    status: DoctorStatus,
    details: String,
    hooks_total: usize,
    cch_hooks: usize,
    events: Vec<EventReport>,
    checked_paths: Vec<String>,
}

#[derive(Debug, Serialize)]
struct EventReport {
    event: String,
    hooks_total: usize,
    cch_hooks: usize,
}

#[derive(Debug, Serialize)]
struct ExtensionsReport {
    extensions_dir: String,
    extensions_dir_exists: bool,
    extensions: Vec<HookFileReport>,
    shared_hooks_dir: String,
    shared_hooks_dir_exists: bool,
    shared_hooks: Vec<HookFileReport>,
}

#[derive(Debug, Serialize)]
struct HookFileReport {
    name: String,
    path: String,
    exists: bool,
    status: DoctorStatus,
    hooks_total: usize,
    cch_hooks: usize,
    details: String,
}

#[derive(Debug, Serialize)]
struct SummaryReport {
    installed_scopes: usize,
    missing_scopes: usize,
    misconfigured_scopes: usize,
    error_scopes: usize,
    extension_hooks_with_cch: usize,
}

#[derive(Debug, Deserialize, Default)]
struct GeminiSettings {
    #[serde(default)]
    hooks: Option<HashMap<String, Vec<GeminiMatcherEntry>>>,
    #[serde(flatten)]
    _extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, Default)]
struct GeminiMatcherEntry {
    #[serde(rename = "matcher", default)]
    _matcher: Option<String>,
    #[serde(default)]
    hooks: Option<Vec<GeminiHookCommand>>,
    #[serde(flatten)]
    _extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, Default)]
struct GeminiHookCommand {
    #[serde(rename = "type")]
    hook_type: Option<String>,
    #[serde(default)]
    command: Option<String>,
    #[serde(flatten)]
    _extra: HashMap<String, Value>,
}

pub async fn run(json: bool) -> Result<()> {
    let report = build_report()?;

    if json {
        let output = serde_json::to_string_pretty(&report)?;
        println!("{}", output);
    } else {
        print_human_report(&report);
    }

    Ok(())
}

fn build_report() -> Result<DoctorReport> {
    let project_settings = project_settings_path()?;
    let user_settings = user_settings_path()?;
    let (system_settings, system_checked) = system_settings_path();

    let mut scopes = Vec::new();
    scopes.push(inspect_settings_scope(
        "project",
        &project_settings,
        vec![project_settings.to_string_lossy().to_string()],
    ));
    scopes.push(inspect_settings_scope(
        "user",
        &user_settings,
        vec![user_settings.to_string_lossy().to_string()],
    ));
    scopes.push(inspect_settings_scope(
        "system",
        &system_settings,
        system_checked,
    ));

    let extensions = inspect_extensions()?;

    let mut summary = SummaryReport {
        installed_scopes: 0,
        missing_scopes: 0,
        misconfigured_scopes: 0,
        error_scopes: 0,
        extension_hooks_with_cch: 0,
    };

    for scope in &scopes {
        match scope.status {
            DoctorStatus::Installed => summary.installed_scopes += 1,
            DoctorStatus::Missing => summary.missing_scopes += 1,
            DoctorStatus::Misconfigured => summary.misconfigured_scopes += 1,
            DoctorStatus::Error => summary.error_scopes += 1,
        }
    }

    summary.extension_hooks_with_cch = extensions
        .extensions
        .iter()
        .chain(extensions.shared_hooks.iter())
        .filter(|entry| entry.status == DoctorStatus::Installed)
        .count();

    Ok(DoctorReport {
        scopes,
        extensions,
        summary,
    })
}

fn project_settings_path() -> Result<PathBuf> {
    let cwd = std::env::current_dir().context("Failed to determine current directory")?;
    Ok(cwd.join(".gemini").join("settings.json"))
}

fn user_settings_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Failed to determine home directory")?;
    Ok(home.join(".gemini").join("settings.json"))
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

fn inspect_settings_scope(scope: &str, path: &Path, checked_paths: Vec<String>) -> ScopeReport {
    let path_str = path.to_string_lossy().to_string();
    if !path.exists() {
        return ScopeReport {
            scope: scope.to_string(),
            path: path_str,
            exists: false,
            status: DoctorStatus::Missing,
            details: "Settings file not found".to_string(),
            hooks_total: 0,
            cch_hooks: 0,
            events: Vec::new(),
            checked_paths,
        };
    }

    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            return ScopeReport {
                scope: scope.to_string(),
                path: path_str,
                exists: true,
                status: DoctorStatus::Error,
                details: format!("Failed to read settings: {}", err),
                hooks_total: 0,
                cch_hooks: 0,
                events: Vec::new(),
                checked_paths,
            };
        }
    };

    let settings: GeminiSettings = match serde_json::from_str(&content) {
        Ok(settings) => settings,
        Err(err) => {
            return ScopeReport {
                scope: scope.to_string(),
                path: path_str,
                exists: true,
                status: DoctorStatus::Error,
                details: format!("Failed to parse JSON: {}", err),
                hooks_total: 0,
                cch_hooks: 0,
                events: Vec::new(),
                checked_paths,
            };
        }
    };

    let hooks = match settings.hooks {
        Some(hooks) => hooks,
        None => {
            return ScopeReport {
                scope: scope.to_string(),
                path: path_str,
                exists: true,
                status: DoctorStatus::Missing,
                details: "No hooks section found".to_string(),
                hooks_total: 0,
                cch_hooks: 0,
                events: Vec::new(),
                checked_paths,
            };
        }
    };

    let mut events = Vec::new();
    let mut hooks_total = 0;
    let mut cch_hooks = 0;
    let mut outdated_hooks = 0;

    let mut event_names: Vec<String> = hooks.keys().cloned().collect();
    event_names.sort();

    for event in event_names {
        if let Some(entries) = hooks.get(&event) {
            let mut event_hooks = 0;
            let mut event_cch = 0;

            for entry in entries {
                let commands = entry.hooks.as_ref().map(|hooks| hooks.as_slice());
                if let Some(hooks) = commands {
                    for hook in hooks {
                        event_hooks += 1;
                        if let Some(command) = hook.command.as_ref() {
                            match classify_cch_command(command, hook.hook_type.as_deref()) {
                                CchCommandStatus::Runner => event_cch += 1,
                                CchCommandStatus::Outdated => outdated_hooks += 1,
                                CchCommandStatus::Other => {}
                            }
                        }
                    }
                }
            }

            hooks_total += event_hooks;
            cch_hooks += event_cch;
            events.push(EventReport {
                event,
                hooks_total: event_hooks,
                cch_hooks: event_cch,
            });
        }
    }

    let (status, details) = if hooks_total == 0 {
        (
            DoctorStatus::Missing,
            "Hooks section present but empty".to_string(),
        )
    } else if cch_hooks == 0 {
        (
            DoctorStatus::Misconfigured,
            if outdated_hooks > 0 {
                "CCH hook commands found but missing `cch gemini hook` (binary may be outdated)"
                    .to_string()
            } else {
                "Hooks found but no cch command entries".to_string()
            },
        )
    } else if outdated_hooks > 0 {
        (
            DoctorStatus::Misconfigured,
            format!(
                "Found {} cch hook entries and {} outdated cch entries",
                cch_hooks, outdated_hooks
            ),
        )
    } else {
        (
            DoctorStatus::Installed,
            format!("Found {} cch hook entries", cch_hooks),
        )
    };

    ScopeReport {
        scope: scope.to_string(),
        path: path_str,
        exists: true,
        status,
        details,
        hooks_total,
        cch_hooks,
        events,
        checked_paths,
    }
}

enum CchCommandStatus {
    Runner,
    Outdated,
    Other,
}

fn classify_cch_command(command: &str, hook_type: Option<&str>) -> CchCommandStatus {
    if let Some(hook_type) = hook_type {
        if hook_type != "command" {
            return CchCommandStatus::Other;
        }
    }

    if !command.contains("cch") {
        return CchCommandStatus::Other;
    }

    if command.contains("gemini hook") {
        return CchCommandStatus::Runner;
    }

    CchCommandStatus::Outdated
}

fn inspect_extensions() -> Result<ExtensionsReport> {
    let home = dirs::home_dir().context("Failed to determine home directory")?;
    let extensions_dir = home.join(".gemini").join("extensions");
    let shared_hooks_dir = home.join(".gemini").join("hooks");

    let mut extensions = Vec::new();
    let extensions_dir_exists = extensions_dir.exists();
    if extensions_dir_exists {
        if let Ok(entries) = fs::read_dir(&extensions_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path
                        .file_name()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    let hooks_path = path.join("hooks").join("hooks.json");
                    extensions.push(inspect_hook_file(&name, &hooks_path));
                }
            }
        }
    }

    let mut shared_hooks = Vec::new();
    let shared_hooks_dir_exists = shared_hooks_dir.exists();
    if shared_hooks_dir_exists {
        if let Ok(entries) = fs::read_dir(&shared_hooks_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().map(|e| e == "json").unwrap_or(false) {
                    let name = path
                        .file_name()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    shared_hooks.push(inspect_hook_file(&name, &path));
                }
            }
        }
    }

    Ok(ExtensionsReport {
        extensions_dir: extensions_dir.to_string_lossy().to_string(),
        extensions_dir_exists,
        extensions,
        shared_hooks_dir: shared_hooks_dir.to_string_lossy().to_string(),
        shared_hooks_dir_exists,
        shared_hooks,
    })
}

fn inspect_hook_file(name: &str, path: &Path) -> HookFileReport {
    let path_str = path.to_string_lossy().to_string();
    if !path.exists() {
        return HookFileReport {
            name: name.to_string(),
            path: path_str,
            exists: false,
            status: DoctorStatus::Missing,
            hooks_total: 0,
            cch_hooks: 0,
            details: "Hook file not found".to_string(),
        };
    }

    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            return HookFileReport {
                name: name.to_string(),
                path: path_str,
                exists: true,
                status: DoctorStatus::Error,
                hooks_total: 0,
                cch_hooks: 0,
                details: format!("Failed to read hook file: {}", err),
            };
        }
    };

    let value: Value = match serde_json::from_str(&content) {
        Ok(value) => value,
        Err(err) => {
            return HookFileReport {
                name: name.to_string(),
                path: path_str,
                exists: true,
                status: DoctorStatus::Error,
                hooks_total: 0,
                cch_hooks: 0,
                details: format!("Failed to parse JSON: {}", err),
            };
        }
    };

    let mut commands = Vec::new();
    collect_command_strings(&value, &mut commands);
    let hooks_total = commands.len();
    let mut cch_hooks = 0;
    let mut outdated_hooks = 0;
    for command in &commands {
        if command.contains("cch") {
            if command.contains("gemini hook") {
                cch_hooks += 1;
            } else {
                outdated_hooks += 1;
            }
        }
    }

    let (status, details) = if hooks_total == 0 {
        (
            DoctorStatus::Misconfigured,
            "No command entries found".to_string(),
        )
    } else if cch_hooks == 0 {
        (
            DoctorStatus::Misconfigured,
            if outdated_hooks > 0 {
                "CCH hook commands found but missing `cch gemini hook` (binary may be outdated)"
                    .to_string()
            } else {
                "Command entries found but none reference cch".to_string()
            },
        )
    } else if outdated_hooks > 0 {
        (
            DoctorStatus::Misconfigured,
            format!(
                "Found {} cch hook entries and {} outdated cch entries",
                cch_hooks, outdated_hooks
            ),
        )
    } else {
        (
            DoctorStatus::Installed,
            format!("Found {} cch command entries", cch_hooks),
        )
    };

    HookFileReport {
        name: name.to_string(),
        path: path_str,
        exists: true,
        status,
        hooks_total,
        cch_hooks,
        details,
    }
}

fn collect_command_strings(value: &Value, commands: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                if key == "command" {
                    if let Value::String(command) = value {
                        commands.push(command.clone());
                    }
                }
                collect_command_strings(value, commands);
            }
        }
        Value::Array(values) => {
            for value in values {
                collect_command_strings(value, commands);
            }
        }
        _ => {}
    }
}

fn print_human_report(report: &DoctorReport) {
    println!("Gemini hook diagnostics");
    println!("");
    println!("Settings scopes:");

    for scope in &report.scopes {
        let status = match scope.status {
            DoctorStatus::Installed => "OK",
            DoctorStatus::Missing => "MISSING",
            DoctorStatus::Misconfigured => "WARN",
            DoctorStatus::Error => "ERROR",
        };

        println!("- {}: {} ({})", scope.scope, status, scope.details);
        println!("  path: {}", scope.path);

        if scope.scope == "system" && scope.checked_paths.len() > 1 {
            println!("  checked: {}", scope.checked_paths.join(", "));
        }

        if scope.status == DoctorStatus::Installed {
            for event in &scope.events {
                if event.cch_hooks > 0 {
                    println!(
                        "  event {}: {} cch hook(s) ({} total)",
                        event.event, event.cch_hooks, event.hooks_total
                    );
                }
            }
        }
    }

    println!("");
    println!("Extension hooks:");
    if !report.extensions.extensions_dir_exists {
        println!(
            "- extensions dir missing: {}",
            report.extensions.extensions_dir
        );
    } else if report.extensions.extensions.is_empty() {
        println!("- extensions dir present, no hook files found");
    } else {
        for entry in &report.extensions.extensions {
            let status = match entry.status {
                DoctorStatus::Installed => "OK",
                DoctorStatus::Missing => "MISSING",
                DoctorStatus::Misconfigured => "WARN",
                DoctorStatus::Error => "ERROR",
            };
            println!("- {}: {} ({})", entry.name, status, entry.details);
            println!("  path: {}", entry.path);
        }
    }

    println!("");
    println!("Shared hooks directory:");
    if !report.extensions.shared_hooks_dir_exists {
        println!(
            "- hooks dir missing: {}",
            report.extensions.shared_hooks_dir
        );
    } else if report.extensions.shared_hooks.is_empty() {
        println!("- hooks dir present, no hook files found");
    } else {
        for entry in &report.extensions.shared_hooks {
            let status = match entry.status {
                DoctorStatus::Installed => "OK",
                DoctorStatus::Missing => "MISSING",
                DoctorStatus::Misconfigured => "WARN",
                DoctorStatus::Error => "ERROR",
            };
            println!("- {}: {} ({})", entry.name, status, entry.details);
            println!("  path: {}", entry.path);
        }
    }

    println!("");
    println!("Summary:");
    println!("- scopes installed: {}", report.summary.installed_scopes);
    println!("- scopes missing: {}", report.summary.missing_scopes);
    println!(
        "- scopes misconfigured: {}",
        report.summary.misconfigured_scopes
    );
    if report.summary.error_scopes > 0 {
        println!("- scope errors: {}", report.summary.error_scopes);
    }
    println!(
        "- extension hook files with cch: {}",
        report.summary.extension_hooks_with_cch
    );
}

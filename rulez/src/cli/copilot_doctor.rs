use anyhow::{Context, Result};
use serde::Serialize;
use serde_json::Value;
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

const OUTDATED_CCH_HINT: &str = "See docs/COPILOT_CLI_HOOKS.md; run `cch copilot install`.";

#[derive(Debug, Serialize)]
struct DoctorReport {
    hooks_dir: String,
    hooks_dir_exists: bool,
    hook_files: Vec<HookFileReport>,
    summary: SummaryReport,
}

#[derive(Debug, Serialize)]
struct HookFileReport {
    name: String,
    path: String,
    exists: bool,
    status: DoctorStatus,
    details: String,
    hooks_total: usize,
    cch_hooks: usize,
    events: Vec<EventReport>,
}

#[derive(Debug, Serialize)]
struct EventReport {
    event: String,
    hooks_total: usize,
    cch_hooks: usize,
}

#[derive(Debug, Serialize)]
#[allow(clippy::struct_field_names)]
struct SummaryReport {
    files_installed: usize,
    files_missing: usize,
    files_misconfigured: usize,
    files_error: usize,
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
    let hooks_dir = hooks_dir()?;
    let hooks_dir_exists = hooks_dir.exists();

    let mut hook_files = Vec::new();

    if hooks_dir_exists {
        if let Ok(entries) = fs::read_dir(&hooks_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().map(|e| e == "json").unwrap_or(false) {
                    let name = path
                        .file_name()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    hook_files.push(inspect_hook_file(&name, &path));
                }
            }
        }
    }

    let mut summary = SummaryReport {
        files_installed: 0,
        files_missing: 0,
        files_misconfigured: 0,
        files_error: 0,
    };

    for file in &hook_files {
        match file.status {
            DoctorStatus::Installed => summary.files_installed += 1,
            DoctorStatus::Missing => summary.files_missing += 1,
            DoctorStatus::Misconfigured => summary.files_misconfigured += 1,
            DoctorStatus::Error => summary.files_error += 1,
        }
    }

    Ok(DoctorReport {
        hooks_dir: hooks_dir.to_string_lossy().to_string(),
        hooks_dir_exists,
        hook_files,
        summary,
    })
}

fn hooks_dir() -> Result<PathBuf> {
    let cwd = std::env::current_dir().context("Failed to determine current directory")?;
    Ok(cwd.join(".github").join("hooks"))
}

fn inspect_hook_file(name: &str, path: &Path) -> HookFileReport {
    let path_str = path.to_string_lossy().to_string();

    if !path.exists() {
        return HookFileReport {
            name: name.to_string(),
            path: path_str,
            exists: false,
            status: DoctorStatus::Missing,
            details: "Hook file not found".to_string(),
            hooks_total: 0,
            cch_hooks: 0,
            events: Vec::new(),
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
                details: format!("Failed to read hook file: {}", err),
                hooks_total: 0,
                cch_hooks: 0,
                events: Vec::new(),
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
                details: format!("Failed to parse JSON: {}", err),
                hooks_total: 0,
                cch_hooks: 0,
                events: Vec::new(),
            };
        }
    };

    let Some(hooks) = value.get("hooks").and_then(Value::as_object) else {
        return HookFileReport {
            name: name.to_string(),
            path: path_str,
            exists: true,
            status: DoctorStatus::Missing,
            details: "No hooks section found".to_string(),
            hooks_total: 0,
            cch_hooks: 0,
            events: Vec::new(),
        };
    };

    let mut events = Vec::new();
    let mut hooks_total = 0;
    let mut cch_hooks = 0;
    let mut outdated_hooks = 0;

    let mut event_names: Vec<String> = hooks.keys().cloned().collect();
    event_names.sort();

    for event in event_names {
        if let Some(entries) = hooks.get(&event).and_then(Value::as_array) {
            let mut event_hooks = 0;
            let mut event_cch = 0;

            for entry in entries {
                // Count commands in each entry
                let commands = collect_commands(entry);
                for command in &commands {
                    event_hooks += 1;
                    match classify_cch_command(command) {
                        CchCommandStatus::Runner => event_cch += 1,
                        CchCommandStatus::Outdated => outdated_hooks += 1,
                        CchCommandStatus::Other => {}
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
                format!(
                    "CCH hook commands found but missing `cch copilot hook` (binary may be outdated). {}",
                    OUTDATED_CCH_HINT
                )
            } else {
                "Hooks found but no cch command entries".to_string()
            },
        )
    } else if outdated_hooks > 0 {
        (
            DoctorStatus::Misconfigured,
            format!(
                "Found {} cch hook entries and {} outdated cch entries. {}",
                cch_hooks, outdated_hooks, OUTDATED_CCH_HINT
            ),
        )
    } else {
        (
            DoctorStatus::Installed,
            format!("Found {} cch hook entries", cch_hooks),
        )
    };

    HookFileReport {
        name: name.to_string(),
        path: path_str,
        exists: true,
        status,
        details,
        hooks_total,
        cch_hooks,
        events,
    }
}

enum CchCommandStatus {
    Runner,
    Outdated,
    Other,
}

fn classify_cch_command(command: &str) -> CchCommandStatus {
    if !command.contains("cch") {
        return CchCommandStatus::Other;
    }

    if command.contains("copilot hook") {
        return CchCommandStatus::Runner;
    }

    CchCommandStatus::Outdated
}

/// Extract command strings from a Copilot hook entry.
/// Copilot hooks use "bash" and "powershell" fields instead of "command".
fn collect_commands(value: &Value) -> Vec<String> {
    let mut commands = Vec::new();
    if let Some(obj) = value.as_object() {
        for key in &["bash", "powershell", "command"] {
            if let Some(Value::String(cmd)) = obj.get(*key) {
                commands.push(cmd.clone());
            }
        }
    }
    commands
}

fn print_human_report(report: &DoctorReport) {
    println!("Copilot hook diagnostics");
    println!();

    if !report.hooks_dir_exists {
        println!("Hooks directory not found: {}", report.hooks_dir);
        println!();
        println!("Run `cch copilot install` to create hooks.");
        return;
    }

    if report.hook_files.is_empty() {
        println!(
            "Hooks directory exists but contains no JSON files: {}",
            report.hooks_dir
        );
        println!();
        println!("Run `cch copilot install` to create hooks.");
        return;
    }

    println!("Hook files in {}:", report.hooks_dir);

    for file in &report.hook_files {
        let status = match file.status {
            DoctorStatus::Installed => "OK",
            DoctorStatus::Missing => "MISSING",
            DoctorStatus::Misconfigured => "WARN",
            DoctorStatus::Error => "ERROR",
        };

        println!("- {}: {} ({})", file.name, status, file.details);
        println!("  path: {}", file.path);

        if file.status == DoctorStatus::Installed {
            for event in &file.events {
                if event.cch_hooks > 0 {
                    println!(
                        "  event {}: {} cch hook(s) ({} total)",
                        event.event, event.cch_hooks, event.hooks_total
                    );
                }
            }
        }
    }

    println!();
    println!("Summary:");
    println!("- files with cch hooks: {}", report.summary.files_installed);
    if report.summary.files_misconfigured > 0 {
        println!(
            "- files misconfigured: {}",
            report.summary.files_misconfigured
        );
    }
    if report.summary.files_error > 0 {
        println!("- files with errors: {}", report.summary.files_error);
    }
}

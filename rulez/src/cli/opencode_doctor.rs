use anyhow::{Context, Result};
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

const OPENCODE_HOOK_EVENTS: [&str; 4] = [
    "file.edited",
    "tool.execute.before",
    "tool.execute.after",
    "session.updated",
];

const OUTDATED_CCH_HINT: &str = "See docs/OPENCODE_CLI_HOOKS.md; run `cch opencode install`.";

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
    summary: SummaryReport,
}

#[derive(Debug, Serialize)]
struct ScopeReport {
    scope: String,
    config_path: String,
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
    scopes_installed: usize,
    scopes_missing: usize,
    scopes_misconfigured: usize,
    scopes_error: usize,
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
    let mut scopes = Vec::new();

    // Project scope
    let project_path = project_config_path()?;
    scopes.push(inspect_scope("project", &project_path));

    // User scope
    let user_path = user_config_path()?;
    scopes.push(inspect_scope("user", &user_path));

    let mut summary = SummaryReport {
        scopes_installed: 0,
        scopes_missing: 0,
        scopes_misconfigured: 0,
        scopes_error: 0,
    };

    for scope in &scopes {
        match scope.status {
            DoctorStatus::Installed => summary.scopes_installed += 1,
            DoctorStatus::Missing => summary.scopes_missing += 1,
            DoctorStatus::Misconfigured => summary.scopes_misconfigured += 1,
            DoctorStatus::Error => summary.scopes_error += 1,
        }
    }

    Ok(DoctorReport { scopes, summary })
}

fn project_config_path() -> Result<PathBuf> {
    let cwd = std::env::current_dir().context("Failed to determine current directory")?;
    Ok(cwd.join(".opencode").join("settings.json"))
}

fn user_config_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    Ok(home
        .join(".config")
        .join("opencode")
        .join("plugins")
        .join("rulez-plugin")
        .join("settings.json"))
}

fn inspect_scope(scope_name: &str, path: &Path) -> ScopeReport {
    let path_str = path.to_string_lossy().to_string();

    if !path.exists() {
        return ScopeReport {
            scope: scope_name.to_string(),
            config_path: path_str,
            exists: false,
            status: DoctorStatus::Missing,
            details: "Config file not found".to_string(),
            hooks_total: 0,
            cch_hooks: 0,
            events: Vec::new(),
        };
    }

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(err) => {
            return ScopeReport {
                scope: scope_name.to_string(),
                config_path: path_str,
                exists: true,
                status: DoctorStatus::Error,
                details: format!("Failed to read config: {}", err),
                hooks_total: 0,
                cch_hooks: 0,
                events: Vec::new(),
            };
        }
    };

    let value: Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(err) => {
            return ScopeReport {
                scope: scope_name.to_string(),
                config_path: path_str,
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
        return ScopeReport {
            scope: scope_name.to_string(),
            config_path: path_str,
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

    for event_name in &OPENCODE_HOOK_EVENTS {
        if let Some(entries) = hooks.get(*event_name).and_then(Value::as_array) {
            let mut event_hooks = 0;
            let mut event_cch = 0;

            for entry in entries {
                if let Some(command) = entry.get("command").and_then(Value::as_str) {
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
                event: (*event_name).to_string(),
                hooks_total: event_hooks,
                cch_hooks: event_cch,
            });
        }
    }

    let (status, details) = if hooks_total == 0 {
        (
            DoctorStatus::Missing,
            "Hooks section present but no recognized events".to_string(),
        )
    } else if cch_hooks == 0 {
        (
            DoctorStatus::Misconfigured,
            if outdated_hooks > 0 {
                format!(
                    "Found cch commands but missing `cch opencode hook` (binary may be outdated). {}",
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
                "Found {} cch hook entries and {} outdated entries. {}",
                cch_hooks, outdated_hooks, OUTDATED_CCH_HINT
            ),
        )
    } else {
        (
            DoctorStatus::Installed,
            format!(
                "Found {} cch hook entries across {} events",
                cch_hooks,
                events.len()
            ),
        )
    };

    ScopeReport {
        scope: scope_name.to_string(),
        config_path: path_str,
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

    if command.contains("opencode hook") {
        return CchCommandStatus::Runner;
    }

    CchCommandStatus::Outdated
}

fn print_human_report(report: &DoctorReport) {
    println!("OpenCode hook diagnostics");
    println!();

    for scope in &report.scopes {
        let status_str = match scope.status {
            DoctorStatus::Installed => "OK",
            DoctorStatus::Missing => "MISSING",
            DoctorStatus::Misconfigured => "WARN",
            DoctorStatus::Error => "ERROR",
        };

        println!("{} scope: {} ({})", scope.scope, status_str, scope.details);
        println!("  path: {}", scope.config_path);

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

        println!();
    }

    println!("Summary:");
    println!(
        "- scopes with cch hooks: {}",
        report.summary.scopes_installed
    );
    if report.summary.scopes_misconfigured > 0 {
        println!(
            "- scopes misconfigured: {}",
            report.summary.scopes_misconfigured
        );
    }
    if report.summary.scopes_error > 0 {
        println!("- scopes with errors: {}", report.summary.scopes_error);
    }
    if report.summary.scopes_installed == 0 {
        println!();
        println!("Run `cch opencode install` to set up hooks.");
    }
}

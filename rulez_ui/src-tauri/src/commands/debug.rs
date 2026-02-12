use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Command};
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleEvaluation {
    #[serde(rename = "ruleName")]
    pub rule_name: String,
    pub matched: bool,
    #[serde(rename = "timeMs")]
    pub time_ms: f64,
    pub details: Option<String>,
    pub pattern: Option<String>,
    pub input: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugResult {
    pub outcome: String,
    pub reason: Option<String>,
    #[serde(rename = "matchedRules")]
    pub matched_rules: Vec<String>,
    #[serde(rename = "evaluationTimeMs")]
    pub evaluation_time_ms: f64,
    pub evaluations: Vec<RuleEvaluation>,
}

/// Run RuleZ debug command and parse output
#[tauri::command]
pub async fn run_debug(
    app_handle: tauri::AppHandle,
    event_type: String,
    tool: Option<String>,
    command: Option<String>,
    path: Option<String>,
) -> Result<DebugResult, String> {
    let mut args = vec!["debug".to_string(), event_type, "--json".to_string()];

    if let Some(t) = tool {
        args.push("--tool".to_string());
        args.push(t);
    }

    if let Some(c) = command {
        args.push("--command".to_string());
        args.push(c);
    }

    if let Some(p) = path {
        args.push("--path".to_string());
        args.push(p);
    }

    let command_path = resolve_rulez_binary_path(&app_handle)?;
    let output = Command::new(&command_path)
        .args(&args)
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                "RuleZ binary not found. Configure a binary path or ensure 'rulez' is in your PATH."
                    .to_string()
            } else {
                format!("Failed to execute RuleZ: {}", e)
            }
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("RuleZ debug failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse RuleZ output: {}", e))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
}

/// Validate config file using RuleZ
#[tauri::command]
pub async fn validate_config(
    app_handle: tauri::AppHandle,
    path: String,
) -> Result<ValidationResult, String> {
    let command_path = resolve_rulez_binary_path(&app_handle)?;
    let output = Command::new(&command_path)
        .args(["validate", &path, "--json"])
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                "RuleZ binary not found. Configure a binary path or ensure 'rulez' is in your PATH."
                    .to_string()
            } else {
                format!("Failed to execute RuleZ: {}", e)
            }
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    if stdout.is_empty() {
        // If output is empty, assume validation passed
        return Ok(ValidationResult {
            valid: output.status.success(),
            errors: vec![],
        });
    }

    serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse RuleZ output: {}", e))
}

fn resolve_rulez_binary_path(app_handle: &tauri::AppHandle) -> Result<String, String> {
    if let Some(path) = read_rulez_binary_path(app_handle) {
        return Ok(path);
    }

    if let Some(path) = resolve_rulez_from_path_env() {
        return Ok(path);
    }

    Err(
        "RuleZ binary not found. Configure a binary path or ensure 'rulez' is in your PATH."
            .to_string(),
    )
}

fn read_rulez_binary_path(app_handle: &tauri::AppHandle) -> Option<String> {
    let path = settings_store_path(app_handle).ok()?;
    let contents = std::fs::read_to_string(path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&contents).ok()?;
    let settings = json.get("settings").unwrap_or(&json);
    let value = settings.get("rulezBinaryPath")?;
    let path_value = value.as_str()?.trim();
    if path_value.is_empty() {
        None
    } else {
        Some(path_value.to_string())
    }
}

fn settings_store_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    app_handle
        .path()
        .app_data_dir()
        .map(|dir| dir.join("settings.json"))
        .map_err(|e| format!("Failed to resolve settings path: {}", e))
}

fn resolve_rulez_from_path_env() -> Option<String> {
    let paths = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&paths) {
        let candidate = dir.join("rulez");
        if is_executable(&candidate) {
            return Some(candidate.to_string_lossy().to_string());
        }

        #[cfg(windows)]
        {
            let candidate = dir.join("rulez.exe");
            if is_executable(&candidate) {
                return Some(candidate.to_string_lossy().to_string());
            }
        }
    }

    None
}

fn is_executable(path: &PathBuf) -> bool {
    path.is_file()
}

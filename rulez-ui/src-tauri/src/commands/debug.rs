use serde::{Deserialize, Serialize};
use std::process::Command;

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

/// Run CCH debug command and parse output
#[tauri::command]
pub async fn run_debug(
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

    let output = Command::new("cch")
        .args(&args)
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                "CCH binary not found. Please ensure 'cch' is installed and in your PATH.".to_string()
            } else {
                format!("Failed to execute CCH: {}", e)
            }
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("CCH debug failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse CCH output: {}", e))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
}

/// Validate config file using CCH
#[tauri::command]
pub async fn validate_config(path: String) -> Result<ValidationResult, String> {
    let output = Command::new("cch")
        .args(["validate", &path, "--json"])
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                "CCH binary not found. Please ensure 'cch' is installed and in your PATH.".to_string()
            } else {
                format!("Failed to execute CCH: {}", e)
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

    serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse CCH output: {}", e))
}

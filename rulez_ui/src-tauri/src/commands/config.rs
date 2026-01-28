use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub path: String,
    pub exists: bool,
    pub modified: bool,
    #[serde(rename = "hasErrors")]
    pub has_errors: bool,
}

/// Get the global config path (~/.claude/hooks.yaml)
fn get_global_config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".claude").join("hooks.yaml"))
}

/// Get the project config path (.claude/hooks.yaml)
fn get_project_config_path(project_dir: Option<String>) -> PathBuf {
    project_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
        .join(".claude")
        .join("hooks.yaml")
}

/// List available config files (global and project)
#[tauri::command]
pub async fn list_config_files(project_dir: Option<String>) -> Result<Vec<ConfigFile>, String> {
    let mut files = Vec::new();

    // Global config
    if let Some(global_path) = get_global_config_path() {
        let exists = global_path.exists();
        files.push(ConfigFile {
            path: global_path.to_string_lossy().to_string(),
            exists,
            modified: false,
            has_errors: false,
        });
    }

    // Project config
    let project_path = get_project_config_path(project_dir);
    let exists = project_path.exists();
    files.push(ConfigFile {
        path: project_path.to_string_lossy().to_string(),
        exists,
        modified: false,
        has_errors: false,
    });

    Ok(files)
}

/// Read config file content
#[tauri::command]
pub async fn read_config(path: String) -> Result<String, String> {
    let path = expand_tilde(&path);

    if !std::path::Path::new(&path).exists() {
        // Return default content for new files
        return Ok(r#"# CCH Configuration
version: "1.0"

settings:
  log_level: "info"

rules: []
"#
        .to_string());
    }

    fs::read_to_string(&path)
        .await
        .map_err(|e| format!("Failed to read file: {}", e))
}

/// Write config file content
#[tauri::command]
pub async fn write_config(path: String, content: String) -> Result<(), String> {
    let path = expand_tilde(&path);
    let path = std::path::Path::new(&path);

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    fs::write(path, content)
        .await
        .map_err(|e| format!("Failed to write file: {}", e))
}

/// Expand ~ to home directory
fn expand_tilde(path: &str) -> String {
    if path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return path.replacen("~", &home.to_string_lossy(), 1);
        }
    }
    path.to_string()
}

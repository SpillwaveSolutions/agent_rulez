use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::opencode::defaults;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCodePluginConfig {
    #[serde(default = "defaults::default_rulez_binary_path")]
    pub rulez_binary_path: String,

    #[serde(default = "defaults::default_audit_log_path")]
    pub audit_log_path: PathBuf,

    #[serde(default = "defaults::default_event_filters")]
    pub event_filters: Vec<String>,
}

impl Default for OpenCodePluginConfig {
    fn default() -> Self {
        Self {
            rulez_binary_path: defaults::default_rulez_binary_path(),
            audit_log_path: defaults::default_audit_log_path(),
            event_filters: defaults::default_event_filters(),
        }
    }
}

impl OpenCodePluginConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;

        let mut config = if config_path.exists() {
            let content = fs::read_to_string(&config_path).with_context(|| {
                format!("Failed to read config file: {}", config_path.display())
            })?;
            serde_json::from_str(&content).with_context(|| {
                format!("Failed to parse config file: {}", config_path.display())
            })?
        } else {
            Self::default()
        };

        // Environment variable overrides
        if let Ok(binary_path) = std::env::var("RULEZ_BINARY_PATH") {
            config.rulez_binary_path = binary_path;
        }

        if let Ok(log_path) = std::env::var("RULEZ_AUDIT_LOG_PATH") {
            config.audit_log_path = PathBuf::from(log_path);
        }

        Ok(config)
    }

    pub fn config_file_path() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Could not determine home directory")?;
        Ok(home
            .join(".config")
            .join("opencode")
            .join("plugins")
            .join("rulez-plugin")
            .join("settings.json"))
    }
}

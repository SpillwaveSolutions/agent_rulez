use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCodeAuditEntry {
    pub timestamp: DateTime<Utc>,
    pub event_id: String,
    pub event_name: String,
    pub decision: String,
    pub reason: Option<String>,
    pub latency_ms: u64,
    pub plugin_name: String,
    pub plugin_version: String,
    pub session_id: String,
}

pub struct OpenCodeAuditLogger {
    log_path: PathBuf,
}

impl OpenCodeAuditLogger {
    pub fn new(log_path: PathBuf) -> Self {
        Self { log_path }
    }

    pub fn log(&self, entry: OpenCodeAuditEntry) -> Result<()> {
        if let Some(parent) = self.log_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        let mut writer = BufWriter::new(file);
        let json = serde_json::to_string(&entry)?;
        writeln!(writer, "{}", json)?;
        writer.flush()?;

        Ok(())
    }

    pub async fn log_async(&self, entry: OpenCodeAuditEntry) {
        // Simple async wrapper, error is ignored as per plan SC-04
        if let Err(e) = self.log(entry) {
            tracing::warn!("Failed to write OpenCode audit log: {}", e);
        }
    }
}

use anyhow::Result;
use chrono::{DateTime, Utc};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::models::LogEntry;

/// JSON Lines logger for audit trails
pub struct Logger {
    writer: Mutex<BufWriter<File>>,
}

impl Logger {
    /// Create a new logger with the default log file path
    pub fn new() -> Result<Self> {
        let log_path = Self::default_log_path();
        Self::with_path(log_path)
    }

    /// Create a new logger with a custom log file path
    #[allow(dead_code)]
    pub fn with_path<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path = path.into();

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new().create(true).append(true).open(&path)?;

        let writer = BufWriter::new(file);

        Ok(Self {
            writer: Mutex::new(writer),
        })
    }

    /// Get the default log file path (~/.claude/logs/rulez.log)
    pub fn default_log_path() -> PathBuf {
        let mut path = dirs::home_dir().expect("Could not determine home directory");
        path.push(".claude");
        path.push("logs");
        path.push("rulez.log");
        path
    }

    /// Log an entry to the JSON Lines file
    pub fn log(&self, entry: LogEntry) -> Result<()> {
        let json = serde_json::to_string(&entry)?;
        let mut writer = self.writer.lock().unwrap();
        writeln!(writer, "{}", json)?;
        writer.flush()?;
        Ok(())
    }

    /// Log an entry asynchronously
    pub async fn log_async(&self, entry: LogEntry) -> Result<()> {
        // For now, just log synchronously since file I/O is fast
        // In the future, this could be made truly async with tokio::fs
        self.log(entry)
    }
}

/// Query logs with filtering and pagination
pub struct LogQuery {
    log_path: PathBuf,
}

impl LogQuery {
    /// Create a new log query for the default log file
    pub fn new() -> Self {
        Self {
            log_path: Logger::default_log_path(),
        }
    }

    /// Create a new log query for a custom log file
    #[allow(dead_code)]
    pub fn with_path<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            log_path: path.into(),
        }
    }

    /// Query logs with optional filters
    pub fn query(&self, filters: QueryFilters) -> Result<Vec<LogEntry>> {
        if !self.log_path.exists() {
            return Ok(Vec::new());
        }

        let content = std::fs::read_to_string(&self.log_path)?;
        let mut entries = Vec::new();

        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let entry: LogEntry = serde_json::from_str(line)?;
            if self.matches_filters(&entry, &filters) {
                entries.push(entry);
            }
        }

        // Sort by timestamp (newest first)
        entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        // Apply limit
        if let Some(limit) = filters.limit {
            entries.truncate(limit);
        }

        Ok(entries)
    }

    /// Check if a log entry matches the given filters
    fn matches_filters(&self, entry: &LogEntry, filters: &QueryFilters) -> bool {
        // Filter by session ID
        if let Some(ref session_id) = filters.session_id {
            if &entry.session_id != session_id {
                return false;
            }
        }

        // Filter by tool name
        if let Some(ref tool_name) = filters.tool_name {
            if entry.tool_name.as_ref() != Some(tool_name) {
                return false;
            }
        }

        // Filter by rule name
        if let Some(ref rule_name) = filters.rule_name {
            if !entry.rules_matched.contains(rule_name) {
                return false;
            }
        }

        // Filter by outcome
        if let Some(ref outcome) = filters.outcome {
            if &entry.outcome != outcome {
                return false;
            }
        }

        // Filter by time range
        if let Some(since) = filters.since {
            if entry.timestamp < since {
                return false;
            }
        }

        if let Some(until) = filters.until {
            if entry.timestamp > until {
                return false;
            }
        }

        // Filter by policy mode (Phase 2.2)
        if let Some(ref mode) = filters.mode {
            if entry.mode.as_ref() != Some(mode) {
                return false;
            }
        }

        // Filter by decision (Phase 2.2)
        if let Some(ref decision) = filters.decision {
            if entry.decision.as_ref() != Some(decision) {
                return false;
            }
        }

        true
    }
}

/// Filters for log queries
#[derive(Debug, Clone, Default)]
pub struct QueryFilters {
    /// Maximum number of entries to return
    pub limit: Option<usize>,

    /// Filter by session ID
    pub session_id: Option<String>,

    /// Filter by tool name
    pub tool_name: Option<String>,

    /// Filter by rule that matched
    pub rule_name: Option<String>,

    /// Filter by outcome
    pub outcome: Option<crate::models::Outcome>,

    /// Filter entries since this timestamp
    pub since: Option<DateTime<Utc>>,

    /// Filter entries until this timestamp
    pub until: Option<DateTime<Utc>>,

    /// Filter by policy mode (Phase 2.2)
    pub mode: Option<crate::models::PolicyMode>,

    /// Filter by decision (Phase 2.2)
    pub decision: Option<crate::models::Decision>,
}

use std::sync::OnceLock;

/// Global logger instance using OnceLock for safe initialization
static GLOBAL_LOGGER: OnceLock<Logger> = OnceLock::new();

/// Initialize the global logger
pub fn init_global_logger() -> Result<()> {
    let logger = Logger::new()?;
    GLOBAL_LOGGER
        .set(logger)
        .map_err(|_| anyhow::anyhow!("Logger already initialized"))?;
    Ok(())
}

/// Get the global logger instance
pub fn global_logger() -> Option<&'static Logger> {
    GLOBAL_LOGGER.get()
}

/// Log an entry using the global logger
pub async fn log_entry(entry: LogEntry) -> Result<()> {
    if let Some(logger) = global_logger() {
        logger.log_async(entry).await?;
    }
    Ok(())
}

/// Rotate log files when they exceed a certain size
#[allow(dead_code)]
pub struct LogRotator {
    max_size_bytes: u64,
    max_files: usize,
}

#[allow(dead_code)]
impl LogRotator {
    /// Create a new log rotator
    pub fn new(max_size_bytes: u64, max_files: usize) -> Self {
        Self {
            max_size_bytes,
            max_files,
        }
    }

    /// Rotate logs if the current log file is too large
    pub fn rotate_if_needed(&self, log_path: &PathBuf) -> Result<()> {
        if !log_path.exists() {
            return Ok(());
        }

        let metadata = std::fs::metadata(log_path)?;
        if metadata.len() < self.max_size_bytes {
            return Ok(());
        }

        // Rotate existing files
        for i in (1..self.max_files).rev() {
            let old_path = format!("{}.{}", log_path.display(), i);
            let new_path = format!("{}.{}", log_path.display(), i + 1);

            if PathBuf::from(&old_path).exists() {
                std::fs::rename(&old_path, &new_path)?;
            }
        }

        // Move current log to .1
        let backup_path = format!("{}.1", log_path.display());
        std::fs::rename(log_path, &backup_path)?;

        Ok(())
    }
}

impl Default for LogRotator {
    fn default() -> Self {
        Self {
            max_size_bytes: 10 * 1024 * 1024, // 10MB
            max_files: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{LogMetadata, LogTiming, Outcome};
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_logger() {
        let temp_file = NamedTempFile::new().unwrap();
        let logger = Logger::with_path(temp_file.path()).unwrap();

        let entry = LogEntry {
            timestamp: Utc::now(),
            event_type: "PreToolUse".to_string(),
            session_id: "test-session".to_string(),
            tool_name: Some("Bash".to_string()),
            rules_matched: vec!["test-rule".to_string()],
            outcome: Outcome::Block,
            timing: LogTiming {
                processing_ms: 5,
                rules_evaluated: 3,
            },
            metadata: Some(LogMetadata {
                injected_files: None,
                validator_output: Some("blocked by policy".to_string()),
            }),
            // Enhanced logging fields (CRD-001)
            event_details: None,
            response: None,
            raw_event: None,
            rule_evaluations: None,
            // Phase 2.2 governance logging fields
            mode: None,
            priority: None,
            decision: None,
            governance: None,
            trust_level: None,
        };

        logger.log_async(entry.clone()).await.unwrap();

        // Read back and verify
        let query = LogQuery::with_path(temp_file.path());
        let filters = QueryFilters {
            limit: Some(10),
            ..Default::default()
        };

        let entries = query.query(filters).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].session_id, "test-session");
    }

    #[test]
    fn test_log_filtering() {
        let temp_file = NamedTempFile::new().unwrap();
        let query = LogQuery::with_path(temp_file.path());

        // Test with empty file
        let filters = QueryFilters::default();
        let entries = query.query(filters).unwrap();
        assert_eq!(entries.len(), 0);
    }
}

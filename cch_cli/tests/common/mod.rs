//! Common test utilities for CCH integration tests.
//!
//! Provides helpers for setting up test environments, running the CLI,
//! and generating test evidence in JSON format.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::time::Instant;

use serde::{Deserialize, Serialize};

/// Test evidence record for IQ/OQ/PQ qualification
#[derive(Debug, Serialize, Deserialize)]
pub struct TestEvidence {
    /// Test name/identifier
    pub test_name: String,
    /// Test category (IQ, OQ, PQ)
    pub category: String,
    /// Whether the test passed
    pub passed: bool,
    /// Execution time in milliseconds
    pub duration_ms: u64,
    /// Test output/details
    pub details: String,
    /// Timestamp of test execution
    pub timestamp: String,
}

impl TestEvidence {
    /// Create new test evidence
    pub fn new(test_name: &str, category: &str) -> Self {
        Self {
            test_name: test_name.to_string(),
            category: category.to_string(),
            passed: false,
            duration_ms: 0,
            details: String::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Mark test as passed with details
    pub fn pass(&mut self, details: &str, duration_ms: u64) {
        self.passed = true;
        self.details = details.to_string();
        self.duration_ms = duration_ms;
    }

    /// Mark test as failed with details
    pub fn fail(&mut self, details: &str, duration_ms: u64) {
        self.passed = false;
        self.details = details.to_string();
        self.duration_ms = duration_ms;
    }

    /// Save evidence to JSON file
    pub fn save(&self, output_dir: &Path) -> std::io::Result<PathBuf> {
        fs::create_dir_all(output_dir)?;
        let filename = format!("{}_{}.json", self.category, self.test_name);
        let path = output_dir.join(filename);
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&path, json)?;
        Ok(path)
    }
}

/// Get the path to the test fixtures directory
pub fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

/// Get the path to the evidence output directory
pub fn evidence_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-evidence")
}

/// Get path to a fixture file
pub fn fixture_path(relative: &str) -> PathBuf {
    fixtures_dir().join(relative)
}

/// Read a fixture file as a string
pub fn read_fixture(relative: &str) -> String {
    fs::read_to_string(fixture_path(relative))
        .unwrap_or_else(|e| panic!("Failed to read fixture {relative}: {e}"))
}

/// Create a temporary test directory with hooks configuration
pub fn setup_test_env(config_name: &str) -> tempfile::TempDir {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    
    // Create .claude directory
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("Failed to create .claude dir");
    
    // Copy hooks configuration
    let config_src = fixture_path(&format!("hooks/{config_name}"));
    let config_dst = claude_dir.join("hooks.yaml");
    fs::copy(&config_src, &config_dst)
        .unwrap_or_else(|e| panic!("Failed to copy config {}: {e}", config_src.display()));
    
    temp_dir
}

/// Timer for measuring test duration
pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn start() -> Self {
        Self { start: Instant::now() }
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
}

/// Parse CCH response from command output
#[derive(Debug, Deserialize)]
pub struct CchResponse {
    #[serde(rename = "continue_")]
    pub continue_: bool,
    pub context: Option<String>,
    pub reason: Option<String>,
    pub timing: Option<CchTiming>,
}

#[derive(Debug, Deserialize)]
pub struct CchTiming {
    pub processing_ms: u64,
    pub rules_evaluated: usize,
}

impl CchResponse {
    /// Parse from command output
    pub fn from_output(output: &Output) -> Result<Self, String> {
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Command failed: {stderr}"));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str(&stdout)
            .map_err(|e| format!("Failed to parse response: {e}\nOutput: {stdout}"))
    }
}

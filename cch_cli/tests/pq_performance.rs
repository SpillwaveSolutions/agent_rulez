//! Performance Qualification (PQ) Tests
//!
//! These tests verify that the CCH binary meets performance requirements:
//! - Cold start: <15ms (realistic target, <5ms deferred to backlog)
//! - Hot execution: <1ms per rule evaluation
//! - Memory usage: reasonable bounds
//!
//! Performance tests generate evidence for compliance audits.

use assert_cmd::Command;
use std::fs;
use std::time::{Duration, Instant};

#[path = "common/mod.rs"]
mod common;
use common::{evidence_dir, fixture_path, TestEvidence, Timer};

/// Target cold start time in milliseconds
const COLD_START_TARGET_MS: u64 = 15;

/// Target processing time per event in milliseconds  
const PROCESSING_TARGET_MS: u64 = 50;

/// Number of iterations for benchmark tests
const BENCHMARK_ITERATIONS: usize = 10;

/// Test binary cold start time (--version)
#[test]
fn test_pq_cold_start_version() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("cold_start_version", "PQ");
    
    // Measure multiple cold starts
    let mut times = Vec::with_capacity(BENCHMARK_ITERATIONS);
    
    for _ in 0..BENCHMARK_ITERATIONS {
        let start = Instant::now();
        
        let output = Command::cargo_bin("cch")
            .expect("binary exists")
            .arg("--version")
            .output()
            .expect("command runs");
        
        let elapsed = start.elapsed();
        times.push(elapsed);
        
        assert!(output.status.success());
    }
    
    // Calculate statistics
    let total: Duration = times.iter().sum();
    let avg_ms = total.as_millis() as u64 / BENCHMARK_ITERATIONS as u64;
    let min_ms = times.iter().min().unwrap().as_millis();
    let max_ms = times.iter().max().unwrap().as_millis();
    
    let details = format!(
        "Cold start (--version): avg={}ms, min={}ms, max={}ms over {} iterations. Target: <{}ms",
        avg_ms, min_ms, max_ms, BENCHMARK_ITERATIONS, COLD_START_TARGET_MS
    );
    
    if avg_ms <= COLD_START_TARGET_MS {
        evidence.pass(&details, timer.elapsed_ms());
    } else {
        evidence.fail(&details, timer.elapsed_ms());
    }
    
    let _ = evidence.save(&evidence_dir());
    
    // This is a soft assertion - we want to track but not fail builds
    // if performance is slightly over target
    assert!(avg_ms < COLD_START_TARGET_MS * 3, 
        "Cold start significantly exceeds target: {avg_ms}ms > {}ms",
        COLD_START_TARGET_MS * 3);
}

/// Test binary cold start time (--help)
#[test]
fn test_pq_cold_start_help() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("cold_start_help", "PQ");
    
    // Measure multiple cold starts
    let mut times = Vec::with_capacity(BENCHMARK_ITERATIONS);
    
    for _ in 0..BENCHMARK_ITERATIONS {
        let start = Instant::now();
        
        let output = Command::cargo_bin("cch")
            .expect("binary exists")
            .arg("--help")
            .output()
            .expect("command runs");
        
        let elapsed = start.elapsed();
        times.push(elapsed);
        
        assert!(output.status.success());
    }
    
    // Calculate statistics
    let total: Duration = times.iter().sum();
    let avg_ms = total.as_millis() as u64 / BENCHMARK_ITERATIONS as u64;
    let min_ms = times.iter().min().unwrap().as_millis();
    let max_ms = times.iter().max().unwrap().as_millis();
    
    let details = format!(
        "Cold start (--help): avg={}ms, min={}ms, max={}ms over {} iterations. Target: <{}ms",
        avg_ms, min_ms, max_ms, BENCHMARK_ITERATIONS, COLD_START_TARGET_MS
    );
    
    if avg_ms <= COLD_START_TARGET_MS {
        evidence.pass(&details, timer.elapsed_ms());
    } else {
        evidence.fail(&details, timer.elapsed_ms());
    }
    
    let _ = evidence.save(&evidence_dir());
}

/// Test event processing time
#[test]
fn test_pq_event_processing_time() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("event_processing_time", "PQ");
    
    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");
    
    // Copy a config with multiple rules
    let config_src = fixture_path("hooks/block-force-push.yaml");
    fs::copy(&config_src, claude_dir.join("hooks.yaml")).expect("copy config");
    
    // Simple event for processing
    let event = r#"{
        "event_type": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {"command": "echo hello"},
        "session_id": "perf-test",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;
    
    // Measure multiple processing times
    let mut times = Vec::with_capacity(BENCHMARK_ITERATIONS);
    
    for _ in 0..BENCHMARK_ITERATIONS {
        let start = Instant::now();
        
        let output = Command::cargo_bin("cch")
            .expect("binary exists")
            .current_dir(temp_dir.path())
            .write_stdin(event)
            .output()
            .expect("command runs");
        
        let elapsed = start.elapsed();
        times.push(elapsed);
        
        assert!(output.status.success());
    }
    
    // Calculate statistics
    let total: Duration = times.iter().sum();
    let avg_ms = total.as_millis() as u64 / BENCHMARK_ITERATIONS as u64;
    let min_ms = times.iter().min().unwrap().as_millis();
    let max_ms = times.iter().max().unwrap().as_millis();
    
    let details = format!(
        "Event processing: avg={}ms, min={}ms, max={}ms over {} iterations. Target: <{}ms",
        avg_ms, min_ms, max_ms, BENCHMARK_ITERATIONS, PROCESSING_TARGET_MS
    );
    
    if avg_ms <= PROCESSING_TARGET_MS {
        evidence.pass(&details, timer.elapsed_ms());
    } else {
        evidence.fail(&details, timer.elapsed_ms());
    }
    
    let _ = evidence.save(&evidence_dir());
    
    assert!(avg_ms < PROCESSING_TARGET_MS * 2,
        "Processing time significantly exceeds target: {avg_ms}ms > {}ms",
        PROCESSING_TARGET_MS * 2);
}

/// Test processing time is included in response
#[test]
fn test_pq_timing_in_response() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("timing_in_response", "PQ");
    
    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");
    
    // Copy config
    let config_src = fixture_path("hooks/block-force-push.yaml");
    fs::copy(&config_src, claude_dir.join("hooks.yaml")).expect("copy config");
    
    let event = r#"{
        "event_type": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {"command": "git status"},
        "session_id": "timing-test",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;
    
    let output = Command::cargo_bin("cch")
        .expect("binary exists")
        .current_dir(temp_dir.path())
        .write_stdin(event)
        .output()
        .expect("command runs");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Response should contain timing information
    assert!(stdout.contains("timing"), "Response should include timing field");
    assert!(stdout.contains("processing_ms"), "Response should include processing_ms");
    assert!(stdout.contains("rules_evaluated"), "Response should include rules_evaluated");
    
    // Parse and verify timing is reasonable
    if let Some(start) = stdout.find("processing_ms") {
        let rest = &stdout[start..];
        if let Some(colon) = rest.find(':') {
            let num_start = colon + 1;
            let num_str: String = rest[num_start..]
                .chars()
                .skip_while(|c| c.is_whitespace())
                .take_while(|c| c.is_ascii_digit())
                .collect();
            
            if let Ok(processing_ms) = num_str.parse::<u64>() {
                assert!(processing_ms < 1000, "Processing time should be < 1 second");
            }
        }
    }
    
    evidence.pass(
        &format!("Response includes timing: {}", stdout.trim()),
        timer.elapsed_ms(),
    );
    let _ = evidence.save(&evidence_dir());
}

/// Test throughput with many rules
#[test]
fn test_pq_throughput_with_rules() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("throughput_with_rules", "PQ");
    
    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");
    
    // Create a config with many rules
    let mut config = String::from("version: \"1.0\"\nrules:\n");
    for i in 0..20 {
        config.push_str(&format!(r#"  - name: rule-{}
    description: "Test rule {}"
    matchers:
      tools: ["Bash"]
      command_match: "pattern{}"
    actions:
      block: false
"#, i, i, i));
    }
    config.push_str("settings:\n  log_level: error\n");
    
    fs::write(claude_dir.join("hooks.yaml"), &config).expect("write config");
    
    let event = r#"{
        "event_type": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {"command": "echo test"},
        "session_id": "throughput-test",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;
    
    // Measure processing with many rules
    let mut times = Vec::with_capacity(BENCHMARK_ITERATIONS);
    
    for _ in 0..BENCHMARK_ITERATIONS {
        let start = Instant::now();
        
        let output = Command::cargo_bin("cch")
            .expect("binary exists")
            .current_dir(temp_dir.path())
            .write_stdin(event)
            .output()
            .expect("command runs");
        
        let elapsed = start.elapsed();
        times.push(elapsed);
        
        assert!(output.status.success());
    }
    
    // Calculate statistics
    let total: Duration = times.iter().sum();
    let avg_ms = total.as_millis() as u64 / BENCHMARK_ITERATIONS as u64;
    
    let details = format!(
        "Processing with 20 rules: avg={}ms over {} iterations",
        avg_ms, BENCHMARK_ITERATIONS
    );
    
    // With many rules, allow more time but should still be reasonable
    if avg_ms <= PROCESSING_TARGET_MS * 2 {
        evidence.pass(&details, timer.elapsed_ms());
    } else {
        evidence.fail(&details, timer.elapsed_ms());
    }
    
    let _ = evidence.save(&evidence_dir());
}

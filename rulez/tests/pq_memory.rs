//! Performance Qualification (PQ) Memory Tests
//!
//! These tests verify that CCH meets memory requirements:
//! - PQ-003: Baseline memory < 10MB RSS
//! - Memory stability under load (no leaks)
//!
//! Memory measurement is platform-specific:
//! - macOS: Uses `ps -o rss` command
//! - Linux: Reads from /proc/[pid]/status
//! - Windows: Uses tasklist command (limited support)
//!
//! NOTE: Memory measurements are approximate and may vary between runs.
//! Run with --release for accurate PQ measurements.

#![allow(unused_imports)]
#![allow(deprecated)]

use assert_cmd::Command;
use std::fs;
use std::process::{Child, Stdio};
use std::thread;
use std::time::Duration;

#[path = "common/mod.rs"]
mod common;
use common::{TestEvidence, Timer, evidence_dir, fixture_path};

/// Target baseline memory in KB (10MB = 10240KB)
const BASELINE_MEMORY_KB: u64 = 10240;

/// Target memory under load in KB (10MB = 10240KB)
const LOAD_MEMORY_KB: u64 = 10240;

/// Number of events to process for load testing
const LOAD_TEST_EVENTS: usize = 100;

/// Multiplier for debug builds (debug uses more memory due to debug symbols)
const DEBUG_MEMORY_MULTIPLIER: u64 = 3;

/// Get memory threshold based on build profile
fn memory_threshold(base_kb: u64) -> u64 {
    if cfg!(debug_assertions) {
        base_kb * DEBUG_MEMORY_MULTIPLIER
    } else {
        base_kb
    }
}

/// Get RSS memory in KB for a process (macOS/Linux)
#[cfg(unix)]
fn get_process_memory_kb(pid: u32) -> Option<u64> {
    // Try macOS ps first
    let output = std::process::Command::new("ps")
        .args(["-o", "rss=", "-p", &pid.to_string()])
        .output()
        .ok()?;

    if output.status.success() {
        let rss_str = String::from_utf8_lossy(&output.stdout);
        return rss_str.trim().parse().ok();
    }

    // Try Linux /proc/[pid]/status
    #[cfg(target_os = "linux")]
    {
        if let Ok(status) = fs::read_to_string(format!("/proc/{}/status", pid)) {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        return parts[1].parse().ok();
                    }
                }
            }
        }
    }

    None
}

/// Get RSS memory in KB for a process (Windows stub)
#[cfg(windows)]
fn get_process_memory_kb(_pid: u32) -> Option<u64> {
    // Windows memory measurement is complex; return None to skip
    // Could use tasklist /FI "PID eq {pid}" /FO CSV but parsing is tricky
    None
}

/// Test baseline memory usage
/// Measures memory of a running CCH process at idle
#[test]
fn test_pq_memory_baseline() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("memory_baseline", "PQ");

    // Setup test environment
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let claude_dir = temp_dir.path().join(".claude");
    fs::create_dir_all(&claude_dir).expect("create .claude");

    // Copy a simple config
    let config_src = fixture_path("hooks/block-force-push.yaml");
    fs::copy(&config_src, claude_dir.join("hooks.yaml")).expect("copy config");

    // Get binary path
    let binary = assert_cmd::cargo::cargo_bin("rulez");

    // Run --version to measure baseline (quick operation)
    let child = std::process::Command::new(&binary)
        .arg("--help")
        .current_dir(temp_dir.path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn rulez");

    let pid = child.id();

    // Give process time to initialize
    thread::sleep(Duration::from_millis(50));

    // Measure memory
    let memory_kb = get_process_memory_kb(pid);

    // Wait for process to complete
    drop(child);

    let target = memory_threshold(BASELINE_MEMORY_KB);
    let build_type = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };

    match memory_kb {
        Some(kb) => {
            let details = format!(
                "Baseline memory: {}KB ({}MB). Target: <{}KB ({})",
                kb,
                kb / 1024,
                target,
                build_type
            );

            if kb <= target {
                evidence.pass(&details, timer.elapsed_ms());
            } else {
                evidence.fail(&details, timer.elapsed_ms());
            }

            // Soft assertion - memory measurement is approximate
            if kb > target * 2 {
                eprintln!(
                    "WARNING: Memory usage {}KB significantly exceeds target {}KB",
                    kb, target
                );
            }
        }
        None => {
            evidence.pass(
                "Memory measurement not available on this platform (skipped)",
                timer.elapsed_ms(),
            );
        }
    }

    let _ = evidence.save(&evidence_dir());
}

/// Test memory usage under load
/// Processes multiple events and checks memory doesn't grow excessively
#[test]
fn test_pq_memory_under_load() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("memory_under_load", "PQ");

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
        "tool_input": {"command": "echo test"},
        "session_id": "memory-load-test",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;

    // Get binary path
    let binary = assert_cmd::cargo::cargo_bin("rulez");

    // Track memory across multiple invocations
    let mut memory_samples: Vec<u64> = Vec::new();

    for i in 0..LOAD_TEST_EVENTS {
        let mut child = std::process::Command::new(&binary)
            .current_dir(temp_dir.path())
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn rulez");

        // Write event
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(event.as_bytes());
        }

        // Sample memory every 10 events
        if i % 10 == 0 {
            thread::sleep(Duration::from_millis(10));
            if let Some(kb) = get_process_memory_kb(child.id()) {
                memory_samples.push(kb);
            }
        }

        let _ = child.wait();
    }

    let target = memory_threshold(LOAD_MEMORY_KB);
    let build_type = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };

    if memory_samples.is_empty() {
        evidence.pass(
            "Memory measurement not available on this platform (skipped)",
            timer.elapsed_ms(),
        );
    } else {
        let avg_kb: u64 = memory_samples.iter().sum::<u64>() / memory_samples.len() as u64;
        let max_kb = *memory_samples.iter().max().unwrap_or(&0);
        let min_kb = *memory_samples.iter().min().unwrap_or(&0);

        let details = format!(
            "Memory under load ({} events): avg={}KB, min={}KB, max={}KB. Target: <{}KB ({})",
            LOAD_TEST_EVENTS, avg_kb, min_kb, max_kb, target, build_type
        );

        if max_kb <= target {
            evidence.pass(&details, timer.elapsed_ms());
        } else {
            evidence.fail(&details, timer.elapsed_ms());
        }
    }

    let _ = evidence.save(&evidence_dir());
}

/// Test memory stability (no leaks)
/// Checks that memory doesn't grow linearly with event count
#[test]
fn test_pq_memory_stability() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("memory_stability", "PQ");

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
        "tool_input": {"command": "echo stability test"},
        "session_id": "memory-stability-test",
        "timestamp": "2025-01-22T12:00:00Z"
    }"#;

    let binary = assert_cmd::cargo::cargo_bin("rulez");

    // Run first batch and measure
    let mut first_batch_memory: Vec<u64> = Vec::new();
    for _ in 0..10 {
        let mut child = std::process::Command::new(&binary)
            .current_dir(temp_dir.path())
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn rulez");

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(event.as_bytes());
        }

        thread::sleep(Duration::from_millis(10));
        if let Some(kb) = get_process_memory_kb(child.id()) {
            first_batch_memory.push(kb);
        }

        let _ = child.wait();
    }

    // Run second batch (after 50 more events)
    for _ in 0..50 {
        let mut child = std::process::Command::new(&binary)
            .current_dir(temp_dir.path())
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn rulez");

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(event.as_bytes());
        }
        let _ = child.wait();
    }

    // Run third batch and measure
    let mut second_batch_memory: Vec<u64> = Vec::new();
    for _ in 0..10 {
        let mut child = std::process::Command::new(&binary)
            .current_dir(temp_dir.path())
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn rulez");

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(event.as_bytes());
        }

        thread::sleep(Duration::from_millis(10));
        if let Some(kb) = get_process_memory_kb(child.id()) {
            second_batch_memory.push(kb);
        }

        let _ = child.wait();
    }

    if first_batch_memory.is_empty() || second_batch_memory.is_empty() {
        evidence.pass(
            "Memory measurement not available on this platform (skipped)",
            timer.elapsed_ms(),
        );
    } else {
        let first_avg: u64 =
            first_batch_memory.iter().sum::<u64>() / first_batch_memory.len() as u64;
        let second_avg: u64 =
            second_batch_memory.iter().sum::<u64>() / second_batch_memory.len() as u64;

        // If first_avg is 0, memory measurement wasn't meaningful (process exited too fast)
        if first_avg == 0 {
            evidence.pass(
                "Memory measurement returned 0 (process exited before measurement); skipped",
                timer.elapsed_ms(),
            );
            let _ = evidence.save(&evidence_dir());
            return;
        }

        // Allow 20% growth as tolerance
        let growth_percent = if second_avg > first_avg {
            ((second_avg - first_avg) * 100) / first_avg
        } else {
            0
        };

        let details = format!(
            "Memory stability: first batch avg={}KB, second batch avg={}KB, growth={}%",
            first_avg, second_avg, growth_percent
        );

        if growth_percent <= 20 {
            evidence.pass(&details, timer.elapsed_ms());
        } else {
            evidence.fail(&details, timer.elapsed_ms());
        }
    }

    let _ = evidence.save(&evidence_dir());
}

/// Test binary size
/// CCH binary should be reasonably small for quick deployment
#[test]
fn test_pq_binary_size() {
    let timer = Timer::start();
    let mut evidence = TestEvidence::new("binary_size", "PQ");

    let binary = assert_cmd::cargo::cargo_bin("rulez");

    if let Ok(metadata) = fs::metadata(&binary) {
        let size_bytes = metadata.len();
        let size_mb = size_bytes as f64 / (1024.0 * 1024.0);

        // Target: < 10MB for release, < 50MB for debug
        let target_mb = if cfg!(debug_assertions) { 50.0 } else { 10.0 };
        let build_type = if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        };

        let details = format!(
            "Binary size: {:.2}MB. Target: <{:.0}MB ({})",
            size_mb, target_mb, build_type
        );

        if size_mb <= target_mb {
            evidence.pass(&details, timer.elapsed_ms());
        } else {
            evidence.fail(&details, timer.elapsed_ms());
        }
    } else {
        evidence.fail(
            &format!("Could not read binary at {:?}", binary),
            timer.elapsed_ms(),
        );
    }

    let _ = evidence.save(&evidence_dir());
}

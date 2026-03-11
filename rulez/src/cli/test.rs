//! RuleZ Test Command - Run batch test scenarios from a YAML file
//!
//! Allows running multiple event scenarios and comparing results against expected outcomes.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

use crate::config::Config;
use crate::hooks;
use crate::models::DebugConfig;

use super::debug::{SimEventType, build_event};

/// YAML test file format
#[derive(Deserialize)]
struct TestFile {
    tests: Vec<TestCase>,
}

/// A single test case within the test file
#[derive(Deserialize)]
struct TestCase {
    name: String,
    event_type: String,
    tool: Option<String>,
    command: Option<String>,
    path: Option<String>,
    prompt: Option<String>,
    /// Expected outcome: "allow", "block", or "inject"
    expected: String,
}

/// Result of running a single test case
struct TestResult {
    passed: bool,
}

/// Run the test command
pub async fn run(test_file: String, verbose: bool) -> Result<()> {
    // Clear regex cache for state isolation
    {
        use crate::hooks::REGEX_CACHE;
        REGEX_CACHE.lock().unwrap().clear();
    }

    // Read and parse the test file
    let content = fs::read_to_string(&test_file)
        .with_context(|| format!("Failed to read test file: {}", test_file))?;
    let test_data: TestFile = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse test file: {}", test_file))?;

    if test_data.tests.is_empty() {
        println!("No test cases found in {}", test_file);
        return Ok(());
    }

    // Load configuration
    let config = Config::load(None)?;
    let debug_config = DebugConfig::new(false, config.settings.debug_logs);

    println!(
        "Running {} test(s) from {}",
        test_data.tests.len(),
        test_file
    );
    println!("{}", "=".repeat(60));
    println!();

    let mut results: Vec<TestResult> = Vec::new();

    for test_case in &test_data.tests {
        // Parse event type
        let event_type =
            SimEventType::parse_event_type(&test_case.event_type).with_context(|| {
                format!(
                    "Test '{}': unknown event type '{}'",
                    test_case.name, test_case.event_type
                )
            })?;

        // Build the simulated event
        let event = build_event(
            event_type,
            test_case.tool.clone(),
            test_case.command.clone(),
            test_case.path.clone(),
            test_case.prompt.clone(),
        );

        // Process the event
        let response = hooks::process_event(event, &debug_config).await?;

        // Determine actual outcome
        let actual = if !response.continue_ {
            "block".to_string()
        } else if response.context.is_some() {
            "inject".to_string()
        } else {
            "allow".to_string()
        };

        let expected = test_case.expected.to_lowercase();
        let passed = actual == expected;

        // Print result
        if passed {
            println!("  PASS  {}", test_case.name);
        } else {
            println!("  FAIL  {}", test_case.name);
            println!("        expected: {}, actual: {}", expected, actual);
            if verbose {
                if let Some(ref reason) = response.reason {
                    println!("        reason: {}", reason);
                }
            }
        }

        results.push(TestResult { passed });
    }

    // Print summary
    let passed = results.iter().filter(|r| r.passed).count();
    let failed = results.iter().filter(|r| !r.passed).count();
    let total = results.len();

    println!();
    println!("{}", "=".repeat(60));
    println!("{} passed, {} failed, {} total", passed, failed, total);

    if failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}

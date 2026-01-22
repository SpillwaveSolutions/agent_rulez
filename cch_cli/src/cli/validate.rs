use anyhow::{Context, Result};
use std::path::Path;

use crate::config::Config;

/// Validate configuration file
pub async fn run(config_path: Option<String>) -> Result<()> {
    let config_path = config_path.unwrap_or_else(|| ".claude/hooks.yaml".to_string());

    println!("Validating configuration file: {}", config_path);

    if !Path::new(&config_path).exists() {
        println!("Configuration file does not exist: {}", config_path);
        println!("Creating default configuration...");

        // Create default config
        let default_config = Config::default();
        let yaml = serde_yaml::to_string(&default_config)
            .context("Failed to serialize default config")?;

        std::fs::create_dir_all(Path::new(&config_path).parent().unwrap_or(Path::new(".")))?;
        std::fs::write(&config_path, yaml)
            .context("Failed to write default config")?;

        println!("Created default configuration at: {}", config_path);
        return Ok(());
    }

    // Load and validate existing config
    let config = Config::from_file(&config_path)
        .context("Failed to load configuration")?;

    println!("✓ Configuration syntax is valid");
    println!("✓ Version: {}", config.version);
    println!("✓ Rules loaded: {}", config.rules.len());

    let enabled_rules = config.enabled_rules();
    println!("✓ Enabled rules: {}", enabled_rules.len());

    if enabled_rules.is_empty() {
        println!("⚠️  No enabled rules found - all operations will be allowed");
    } else {
        println!("✓ Rules validated successfully");
        for rule in enabled_rules {
            println!("  - {}: {}", rule.name, rule.description.as_deref().unwrap_or("No description"));
        }
    }

    Ok(())
}
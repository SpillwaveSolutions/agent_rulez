//! CLI handler for `rulez skills` subcommand family.
//!
//! Manages skill/command distribution across AI coding runtimes.

use anyhow::Result;
use std::path::PathBuf;

use crate::skills::config_gen;
use crate::skills::discovery::SkillInventory;
use crate::skills::profiles::{Runtime, Scope};
use crate::skills::writer;

/// Run `rulez skills install`.
pub async fn install(
    runtime_str: &str,
    scope_str: &str,
    custom_dir: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    let runtime = Runtime::from_str_arg(runtime_str, custom_dir).map_err(|e| anyhow::anyhow!(e))?;
    let scope = parse_scope(scope_str)?;
    let project_root = std::env::current_dir()?;

    let inventory = discover_inventory(&project_root);

    if inventory.skills.is_empty() && inventory.commands.is_empty() {
        println!("No skills or commands found in .claude/skills/ or .claude/commands/");
        return Ok(());
    }

    println!(
        "Discovered {} skills and {} commands",
        inventory.skills.len(),
        inventory.commands.len()
    );

    let result = writer::write_skills(&inventory, &runtime, scope, &project_root, dry_run)?;

    if !dry_run {
        println!(
            "  Installed {} skills and {} commands ({} files) to {}",
            result.skills_written,
            result.commands_written,
            result.files_total,
            result.target_dir.display()
        );

        // Update config file (GEMINI.md, AGENTS.md) if applicable
        if let Some(config_path) = config_gen::config_file_path(&runtime, &project_root) {
            if config_gen::update_config_file(&config_path, &inventory, &runtime)? {
                println!("  Updated {}", config_path.display());
            }
        }
    }

    Ok(())
}

/// Run `rulez skills status` with human-readable timestamps.
pub async fn status() -> Result<()> {
    let project_root = std::env::current_dir()?;

    let runtimes = [
        Runtime::Claude,
        Runtime::OpenCode,
        Runtime::Gemini,
        Runtime::Codex,
    ];

    println!("{:<12} {:<12} Last Updated", "Runtime", "Installed");
    let separator = "-".repeat(50);
    println!("{separator}");

    let now = std::time::SystemTime::now();

    for rt in &runtimes {
        let (installed, mtime) = writer::check_status(rt, Scope::Project, &project_root);
        let status_str = if installed { "yes" } else { "no" };
        let time_str = mtime
            .and_then(|t| now.duration_since(t).ok())
            .map(|d| format_duration(d.as_secs()))
            .unwrap_or_else(|| "-".to_string());

        println!("{:<12} {:<12} {}", rt.name(), status_str, time_str);
    }

    Ok(())
}

/// Run `rulez skills diff` — show what would change.
pub async fn diff(runtime_str: &str, custom_dir: Option<&str>) -> Result<()> {
    let runtime = Runtime::from_str_arg(runtime_str, custom_dir).map_err(|e| anyhow::anyhow!(e))?;
    let project_root = std::env::current_dir()?;

    let inventory = discover_inventory(&project_root);

    if inventory.skills.is_empty() && inventory.commands.is_empty() {
        println!("No skills or commands found.");
        return Ok(());
    }

    // Use dry-run with verbose output showing per-file status
    let profile = crate::skills::profiles::RuntimeProfile::for_runtime(&runtime);
    let skills_dir = profile.target_skills_dir(Scope::Project, &project_root);

    if !skills_dir.exists() {
        println!(
            "No existing installation at {}. All files would be new.",
            skills_dir.display()
        );
        println!(
            "  {} skills and {} commands would be installed.",
            inventory.skills.len(),
            inventory.commands.len()
        );
        return Ok(());
    }

    // Show what would change by comparing discovery with existing
    let pipeline = crate::skills::transform::TransformPipeline::for_runtime(&runtime);
    let mut changes = 0;
    let mut unchanged = 0;

    for skill in &inventory.skills {
        let target_skill_md = skills_dir.join(&skill.name).join("SKILL.md");
        if target_skill_md.exists() {
            let existing = std::fs::read_to_string(&target_skill_md).unwrap_or_default();
            let source = std::fs::read_to_string(&skill.entry_point).unwrap_or_default();
            let transformed = pipeline.transform_content(&source);
            if existing == transformed {
                unchanged += 1;
            } else {
                println!("  M {}/{}/SKILL.md", profile.skills_dir, skill.name);
                changes += 1;
            }
        } else {
            println!("  + {}/{}/SKILL.md", profile.skills_dir, skill.name);
            changes += 1;
        }
    }

    if let Some(cmds_dir) = &profile.commands_dir {
        let target_cmds = project_root.join(cmds_dir);
        for cmd in &inventory.commands {
            let new_name = pipeline.transform_command_filename(&cmd.filename);
            let target_path = target_cmds.join(&new_name);
            if target_path.exists() {
                let existing = std::fs::read_to_string(&target_path).unwrap_or_default();
                let source = std::fs::read_to_string(&cmd.source_path).unwrap_or_default();
                let transformed = pipeline.transform_content(&source);
                if existing == transformed {
                    unchanged += 1;
                } else {
                    println!("  M {cmds_dir}/{new_name}");
                    changes += 1;
                }
            } else {
                println!("  + {cmds_dir}/{new_name}");
                changes += 1;
            }
        }
    }

    println!();
    println!("{changes} changed, {unchanged} unchanged");

    Ok(())
}

/// Run `rulez skills clean`.
pub async fn clean(runtime_str: &str, custom_dir: Option<&str>) -> Result<()> {
    let runtime = Runtime::from_str_arg(runtime_str, custom_dir).map_err(|e| anyhow::anyhow!(e))?;
    let project_root = std::env::current_dir()?;

    let removed = writer::clean_skills(&runtime, Scope::Project, &project_root)?;
    if removed > 0 {
        println!("Cleaned {} directories for {}", removed, runtime.name());
    } else {
        println!("No installed skills found for {}", runtime.name());
    }

    Ok(())
}

/// Run `rulez skills sync` — install to all known runtimes.
pub async fn sync(dry_run: bool) -> Result<()> {
    let runtimes = ["opencode", "gemini", "codex"];

    for rt in runtimes {
        println!("\n--- Installing to {rt} ---");
        install(rt, "project", None, dry_run).await?;
    }

    println!("\n--- Sync complete ---");

    Ok(())
}

/// Discover skills and commands from canonical source.
fn discover_inventory(project_root: &std::path::Path) -> SkillInventory {
    let skills_dir = project_root.join(".claude/skills");
    let commands_dir = project_root.join(".claude/commands");

    let mastering_hooks = project_root.join("mastering-hooks");
    let extra_skills: Vec<PathBuf> = if mastering_hooks.is_dir() {
        vec![mastering_hooks]
    } else {
        vec![]
    };

    SkillInventory::discover(&skills_dir, Some(&commands_dir), &extra_skills)
}

fn parse_scope(s: &str) -> Result<Scope> {
    match s.to_lowercase().as_str() {
        "project" => Ok(Scope::Project),
        "global" => Ok(Scope::Global),
        other => Err(anyhow::anyhow!(
            "Unknown scope: {other}. Use 'project' or 'global'."
        )),
    }
}

/// Format a duration in seconds to human-readable relative time.
fn format_duration(secs: u64) -> String {
    if secs < 60 {
        return format!("{secs}s ago");
    }
    let mins = secs / 60;
    if mins < 60 {
        return format!("{mins}m ago");
    }
    let hours = mins / 60;
    if hours < 24 {
        return format!("{hours}h ago");
    }
    let days = hours / 24;
    format!("{days}d ago")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30), "30s ago");
        assert_eq!(format_duration(90), "1m ago");
        assert_eq!(format_duration(3600), "1h ago");
        assert_eq!(format_duration(86_400), "1d ago");
        assert_eq!(format_duration(172_800), "2d ago");
    }
}

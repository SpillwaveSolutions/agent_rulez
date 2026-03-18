//! File writer for transformed skills and commands.
//!
//! Uses clean-install approach: removes existing target directory before writing
//! fresh files. Binary resources (scripts, images) are copied without transformation.
//! Markdown files go through the transform pipeline.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::skills::discovery::{DiscoveredCommand, DiscoveredSkill, SkillInventory};
use crate::skills::profiles::{Runtime, RuntimeProfile, Scope};
use crate::skills::transform::TransformPipeline;

/// Result of a write operation.
#[derive(Debug, Default)]
pub struct WriteResult {
    pub skills_written: usize,
    pub commands_written: usize,
    pub files_total: usize,
    pub target_dir: PathBuf,
}

/// Write transformed skills and commands to the target runtime directory.
pub fn write_skills(
    inventory: &SkillInventory,
    target: &Runtime,
    scope: Scope,
    project_root: &Path,
    dry_run: bool,
) -> Result<WriteResult> {
    let profile = RuntimeProfile::for_runtime(target);
    let pipeline = TransformPipeline::for_runtime(target);
    let skills_dir = profile.target_skills_dir(scope, project_root);
    let commands_dir = profile.target_commands_dir(scope, project_root);

    let mut result = WriteResult {
        target_dir: skills_dir.clone(),
        ..Default::default()
    };

    if dry_run {
        println!("Dry run: would install to {}", skills_dir.display());
        for skill in &inventory.skills {
            println!("  skill: {}", skill.name);
        }
        for cmd in &inventory.commands {
            let new_name = pipeline.transform_command_filename(&cmd.filename);
            println!("  command: {} -> {}", cmd.filename, new_name);
        }
        return Ok(result);
    }

    // Clean install: remove and recreate skills directory
    if skills_dir.exists() {
        fs::remove_dir_all(&skills_dir)
            .with_context(|| format!("Failed to clean {}", skills_dir.display()))?;
    }
    fs::create_dir_all(&skills_dir)
        .with_context(|| format!("Failed to create {}", skills_dir.display()))?;

    // Write skills
    for skill in &inventory.skills {
        let count = write_skill(skill, &skills_dir, &pipeline)?;
        result.skills_written += 1;
        result.files_total += count;
    }

    // Write commands (if runtime supports them)
    if let Some(ref cmds_dir) = commands_dir {
        if cmds_dir.exists() {
            fs::remove_dir_all(cmds_dir)
                .with_context(|| format!("Failed to clean {}", cmds_dir.display()))?;
        }
        fs::create_dir_all(cmds_dir)
            .with_context(|| format!("Failed to create {}", cmds_dir.display()))?;

        for cmd in &inventory.commands {
            write_command(cmd, cmds_dir, &pipeline)?;
            result.commands_written += 1;
            result.files_total += 1;
        }
    }

    Ok(result)
}

/// Write a single skill directory to the target.
fn write_skill(
    skill: &DiscoveredSkill,
    target_skills_dir: &Path,
    pipeline: &TransformPipeline,
) -> Result<usize> {
    let skill_dir = target_skills_dir.join(&skill.name);
    fs::create_dir_all(&skill_dir)?;

    let mut file_count = 0;

    // Transform and write the entry point (SKILL.md)
    let content = fs::read_to_string(&skill.entry_point)?;
    let transformed = pipeline.transform_content(&content);
    fs::write(skill_dir.join("SKILL.md"), transformed)?;
    file_count += 1;

    // Copy resources
    for resource in &skill.resources {
        let relative = resource.strip_prefix(&skill.source_dir).unwrap_or(resource);
        let dest = skill_dir.join(relative);

        if resource.is_dir() {
            file_count += copy_dir_recursive(resource, &dest, pipeline)?;
        } else {
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            file_count += copy_file(resource, &dest, pipeline)?;
        }
    }

    Ok(file_count)
}

/// Write a single command file to the target.
fn write_command(
    cmd: &DiscoveredCommand,
    target_commands_dir: &Path,
    pipeline: &TransformPipeline,
) -> Result<()> {
    let new_filename = pipeline.transform_command_filename(&cmd.filename);
    let content = fs::read_to_string(&cmd.source_path)?;
    let transformed = pipeline.transform_content(&content);
    fs::write(target_commands_dir.join(new_filename), transformed)?;
    Ok(())
}

/// Copy a file, transforming markdown content but copying binary files as-is.
fn copy_file(src: &Path, dest: &Path, pipeline: &TransformPipeline) -> Result<usize> {
    let is_markdown = src
        .extension()
        .is_some_and(|ext| ext == "md" || ext == "yaml" || ext == "yml");

    if is_markdown {
        let content = fs::read_to_string(src)?;
        let transformed = pipeline.transform_content(&content);
        fs::write(dest, transformed)?;
    } else {
        fs::copy(src, dest)?;
    }
    Ok(1)
}

/// Recursively copy a directory, transforming markdown content.
fn copy_dir_recursive(src: &Path, dest: &Path, pipeline: &TransformPipeline) -> Result<usize> {
    fs::create_dir_all(dest)?;
    let mut count = 0;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if src_path.is_dir() {
            count += copy_dir_recursive(&src_path, &dest_path, pipeline)?;
        } else {
            count += copy_file(&src_path, &dest_path, pipeline)?;
        }
    }

    Ok(count)
}

/// Remove installed skills for a runtime (clean operation).
pub fn clean_skills(target: &Runtime, scope: Scope, project_root: &Path) -> Result<usize> {
    let profile = RuntimeProfile::for_runtime(target);
    let skills_dir = profile.target_skills_dir(scope, project_root);
    let commands_dir = profile.target_commands_dir(scope, project_root);
    let mut removed = 0;

    if skills_dir.exists() {
        fs::remove_dir_all(&skills_dir)?;
        removed += 1;
    }
    if let Some(cmds_dir) = commands_dir {
        if cmds_dir.exists() {
            fs::remove_dir_all(&cmds_dir)?;
            removed += 1;
        }
    }

    Ok(removed)
}

/// Check if a runtime has installed skills and their freshness.
pub fn check_status(
    target: &Runtime,
    scope: Scope,
    project_root: &Path,
) -> (bool, Option<std::time::SystemTime>) {
    let profile = RuntimeProfile::for_runtime(target);
    let skills_dir = profile.target_skills_dir(scope, project_root);

    if !skills_dir.exists() {
        return (false, None);
    }

    // Get the most recent modification time
    let mtime = fs::metadata(&skills_dir)
        .ok()
        .and_then(|m| m.modified().ok());

    (true, mtime)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skills::discovery::SkillInventory;

    #[test]
    fn test_write_skills_dry_run() {
        let tmp = tempfile::tempdir().unwrap();
        let skills_dir = tmp.path().join(".claude/skills");
        fs::create_dir_all(skills_dir.join("test-skill")).unwrap();
        fs::write(skills_dir.join("test-skill/SKILL.md"), "# Test").unwrap();

        let commands_dir = tmp.path().join(".claude/commands");
        fs::create_dir_all(&commands_dir).unwrap();
        fs::write(
            commands_dir.join("speckit.analyze.md"),
            "---\ndescription: test\n---\nContent",
        )
        .unwrap();

        let inventory = SkillInventory::discover(&skills_dir, Some(&commands_dir), &[]);

        let result = write_skills(
            &inventory,
            &Runtime::OpenCode,
            Scope::Project,
            tmp.path(),
            true, // dry run
        )
        .unwrap();

        // Dry run should not create files
        assert!(!tmp.path().join(".opencode/skill").exists());
        assert_eq!(result.skills_written, 0);
    }

    #[test]
    fn test_write_skills_opencode() {
        let tmp = tempfile::tempdir().unwrap();
        let skills_dir = tmp.path().join(".claude/skills");
        fs::create_dir_all(skills_dir.join("my-skill")).unwrap();
        fs::write(
            skills_dir.join("my-skill/SKILL.md"),
            "---\nname: my-skill\ndescription: Test\nallowed-tools:\n  - Read\n  - Bash\n---\n\nUse Read at ~/.claude/",
        )
        .unwrap();

        let commands_dir = tmp.path().join(".claude/commands");
        fs::create_dir_all(&commands_dir).unwrap();
        fs::write(
            commands_dir.join("speckit.analyze.md"),
            "---\ndescription: Analyze\n---\nRun /speckit.plan first",
        )
        .unwrap();

        let inventory = SkillInventory::discover(&skills_dir, Some(&commands_dir), &[]);

        let result = write_skills(
            &inventory,
            &Runtime::OpenCode,
            Scope::Project,
            tmp.path(),
            false,
        )
        .unwrap();

        assert_eq!(result.skills_written, 1);
        assert_eq!(result.commands_written, 1);

        // Verify skill was transformed
        let skill_content =
            fs::read_to_string(tmp.path().join(".opencode/skill/my-skill/SKILL.md")).unwrap();
        assert!(skill_content.contains("tools:"));
        assert!(skill_content.contains("  read: true"));
        assert!(skill_content.contains("~/.config/opencode/"));
        assert!(!skill_content.contains("~/.claude/"));

        // Verify command was renamed and transformed
        let cmd_path = tmp.path().join(".opencode/command/speckit-analyze.md");
        assert!(cmd_path.exists());
        let cmd_content = fs::read_to_string(cmd_path).unwrap();
        assert!(cmd_content.contains("/speckit-plan"));
    }

    #[test]
    fn test_clean_skills() {
        let tmp = tempfile::tempdir().unwrap();
        let skill_dir = tmp.path().join(".opencode/skill/test");
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(skill_dir.join("SKILL.md"), "test").unwrap();

        let removed = clean_skills(&Runtime::OpenCode, Scope::Project, tmp.path()).unwrap();
        assert_eq!(removed, 1);
        assert!(!tmp.path().join(".opencode/skill").exists());
    }
}

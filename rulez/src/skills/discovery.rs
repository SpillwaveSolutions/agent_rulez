//! Skill and command discovery from canonical Claude Code source directories.
//!
//! Scans `.claude/skills/` for skill directories (each containing a `SKILL.md`)
//! and `.claude/commands/` for command files (`*.md`).

use std::path::{Path, PathBuf};

/// A discovered skill directory.
#[derive(Debug, Clone)]
pub struct DiscoveredSkill {
    /// Skill name (directory name, e.g., "architect-agent")
    pub name: String,
    /// Full path to the skill directory
    pub source_dir: PathBuf,
    /// Path to the entry point file (SKILL.md)
    pub entry_point: PathBuf,
    /// Additional resource files/directories (references/, scripts/, etc.)
    pub resources: Vec<PathBuf>,
}

/// A discovered command file.
#[derive(Debug, Clone)]
pub struct DiscoveredCommand {
    /// Command name from filename (e.g., "speckit.analyze")
    pub name: String,
    /// Full path to the command file
    pub source_path: PathBuf,
    /// Filename (e.g., "speckit.analyze.md")
    pub filename: String,
}

/// Inventory of all discovered skills and commands.
#[derive(Debug, Default)]
pub struct SkillInventory {
    pub skills: Vec<DiscoveredSkill>,
    pub commands: Vec<DiscoveredCommand>,
}

impl SkillInventory {
    /// Scan canonical source directories and build an inventory.
    ///
    /// - `skills_dir`: Path to `.claude/skills/` (or equivalent)
    /// - `commands_dir`: Path to `.claude/commands/` (or equivalent)
    /// - `extra_skills`: Additional skill directories outside the standard location
    ///   (e.g., `mastering-hooks/` at repo root)
    pub fn discover(
        skills_dir: &Path,
        commands_dir: Option<&Path>,
        extra_skills: &[PathBuf],
    ) -> Self {
        let mut inventory = Self::default();

        // Discover skills from the standard directory
        if skills_dir.is_dir() {
            inventory.discover_skills_from(skills_dir);
        }

        // Discover skills from extra locations
        for extra in extra_skills {
            if extra.is_dir() {
                // Check if this is a single skill directory (has SKILL.md)
                let skill_md = extra.join("SKILL.md");
                if skill_md.exists() {
                    if let Some(skill) = Self::discover_single_skill(extra) {
                        inventory.skills.push(skill);
                    }
                } else {
                    // It's a directory of skills
                    inventory.discover_skills_from(extra);
                }
            }
        }

        // Discover commands
        if let Some(cmds_dir) = commands_dir {
            if cmds_dir.is_dir() {
                inventory.discover_commands_from(cmds_dir);
            }
        }

        // Sort for deterministic output
        inventory.skills.sort_by(|a, b| a.name.cmp(&b.name));
        inventory.commands.sort_by(|a, b| a.name.cmp(&b.name));

        inventory
    }

    fn discover_skills_from(&mut self, dir: &Path) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(skill) = Self::discover_single_skill(&path) {
                    self.skills.push(skill);
                }
            }
        }
    }

    fn discover_single_skill(skill_dir: &Path) -> Option<DiscoveredSkill> {
        let name = skill_dir.file_name()?.to_str()?.to_string();
        let entry_point = skill_dir.join("SKILL.md");

        if !entry_point.exists() {
            return None;
        }

        // Collect resource files (everything except SKILL.md)
        let mut resources = Vec::new();
        if let Ok(entries) = std::fs::read_dir(skill_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p != entry_point {
                    resources.push(p);
                }
            }
        }
        resources.sort();

        Some(DiscoveredSkill {
            name,
            source_dir: skill_dir.to_path_buf(),
            entry_point,
            resources,
        })
    }

    fn discover_commands_from(&mut self, dir: &Path) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|e| e == "md") {
                if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                    let name = filename.trim_end_matches(".md").to_string();
                    let filename_owned = filename.to_string();
                    self.commands.push(DiscoveredCommand {
                        name,
                        source_path: path,
                        filename: filename_owned,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_discover_skills() {
        let tmp = tempfile::tempdir().unwrap();
        let skills_dir = tmp.path().join("skills");
        fs::create_dir_all(skills_dir.join("my-skill/references")).unwrap();
        fs::write(skills_dir.join("my-skill/SKILL.md"), "# My Skill").unwrap();
        fs::write(skills_dir.join("my-skill/references/guide.md"), "# Guide").unwrap();

        // Directory without SKILL.md should be ignored
        fs::create_dir_all(skills_dir.join("not-a-skill")).unwrap();
        fs::write(skills_dir.join("not-a-skill/README.md"), "# Readme").unwrap();

        let inventory = SkillInventory::discover(&skills_dir, None, &[]);
        assert_eq!(inventory.skills.len(), 1);
        assert_eq!(inventory.skills[0].name, "my-skill");
        assert_eq!(inventory.skills[0].resources.len(), 1);
    }

    #[test]
    fn test_discover_commands() {
        let tmp = tempfile::tempdir().unwrap();
        let commands_dir = tmp.path().join("commands");
        fs::create_dir_all(&commands_dir).unwrap();
        fs::write(commands_dir.join("speckit.analyze.md"), "---\n---\nContent").unwrap();
        fs::write(commands_dir.join("cch-release.md"), "---\n---\nRelease").unwrap();

        let inventory = SkillInventory::discover(tmp.path(), Some(&commands_dir), &[]);
        assert_eq!(inventory.commands.len(), 2);
        assert_eq!(inventory.commands[0].name, "cch-release");
        assert_eq!(inventory.commands[1].name, "speckit.analyze");
    }

    #[test]
    fn test_discover_extra_skill() {
        let tmp = tempfile::tempdir().unwrap();
        let skills_dir = tmp.path().join("skills");
        fs::create_dir_all(&skills_dir).unwrap();

        // Extra skill at repo root
        let extra = tmp.path().join("mastering-hooks");
        fs::create_dir_all(extra.join("references")).unwrap();
        fs::write(extra.join("SKILL.md"), "# Mastering Hooks").unwrap();

        let inventory = SkillInventory::discover(&skills_dir, None, &[extra]);
        assert_eq!(inventory.skills.len(), 1);
        assert_eq!(inventory.skills[0].name, "mastering-hooks");
    }
}

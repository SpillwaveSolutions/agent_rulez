//! Runtime profile definitions for multi-platform skill distribution.
//!
//! Each supported runtime has a [`RuntimeProfile`] that describes its conventions:
//! directory layout, naming style, tool name format, and path prefixes.

use std::path::PathBuf;

/// Supported AI coding runtimes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Runtime {
    Claude,
    OpenCode,
    Gemini,
    Codex,
    /// Generic skill-based runtime with a custom output directory.
    Custom(String),
}

impl Runtime {
    /// Parse from CLI string argument.
    pub fn from_str_arg(s: &str, custom_dir: Option<&str>) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "claude" => Ok(Self::Claude),
            "opencode" => Ok(Self::OpenCode),
            "gemini" => Ok(Self::Gemini),
            "codex" => Ok(Self::Codex),
            "skills" | "custom" => {
                let dir = custom_dir
                    .ok_or_else(|| "Custom runtime requires --dir argument".to_string())?;
                Ok(Self::Custom(dir.to_string()))
            }
            other => Err(format!("Unknown runtime: {other}")),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Claude => "claude",
            Self::OpenCode => "opencode",
            Self::Gemini => "gemini",
            Self::Codex => "codex",
            Self::Custom(_) => "custom",
        }
    }
}

/// How tool names should be cased in skill/command content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolNameStyle {
    /// Claude Code: `Read`, `Write`, `Bash`
    PascalCase,
    /// OpenCode: `read`, `write`, `bash`
    Lowercase,
    /// Gemini CLI: `read_file`, `write_file`, `run_shell_command`
    SnakeCase,
}

/// Installation scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Project,
    Global,
}

/// Describes one runtime's conventions for skills and commands.
#[derive(Debug, Clone)]
pub struct RuntimeProfile {
    pub runtime: Runtime,
    /// Relative path for skills directory (e.g., ".claude/skills")
    pub skills_dir: &'static str,
    /// Relative path for commands directory (None if commands become skills)
    pub commands_dir: Option<&'static str>,
    /// Separator used in command filenames: `.` for Claude, `-` for OpenCode
    pub command_separator: char,
    /// How tool names appear in markdown content
    pub tool_name_style: ToolNameStyle,
    /// Global config path prefix (e.g., "~/.claude/")
    pub global_path_prefix: &'static str,
    /// Local config path prefix (e.g., ".claude/")
    pub local_path_prefix: &'static str,
}

impl RuntimeProfile {
    /// Get the built-in profile for a known runtime.
    pub fn for_runtime(runtime: &Runtime) -> Self {
        match runtime {
            Runtime::Claude => Self {
                runtime: Runtime::Claude,
                skills_dir: ".claude/skills",
                commands_dir: Some(".claude/commands"),
                command_separator: '.',
                tool_name_style: ToolNameStyle::PascalCase,
                global_path_prefix: "~/.claude/",
                local_path_prefix: ".claude/",
            },
            Runtime::OpenCode => Self {
                runtime: Runtime::OpenCode,
                skills_dir: ".opencode/skill",
                commands_dir: Some(".opencode/command"),
                command_separator: '-',
                tool_name_style: ToolNameStyle::Lowercase,
                global_path_prefix: "~/.config/opencode/",
                local_path_prefix: ".opencode/",
            },
            Runtime::Gemini => Self {
                runtime: Runtime::Gemini,
                skills_dir: ".gemini/skills",
                commands_dir: None, // Gemini uses TOML or skills
                command_separator: '-',
                tool_name_style: ToolNameStyle::SnakeCase,
                global_path_prefix: "~/.gemini/",
                local_path_prefix: ".gemini/",
            },
            Runtime::Codex => Self {
                runtime: Runtime::Codex,
                skills_dir: ".codex/skills",
                commands_dir: None, // Codex converts commands to skills
                command_separator: '-',
                tool_name_style: ToolNameStyle::Lowercase,
                global_path_prefix: "~/.codex/",
                local_path_prefix: ".codex/",
            },
            Runtime::Custom(dir) => Self {
                runtime: Runtime::Custom(dir.clone()),
                skills_dir: "", // Will use custom_dir directly
                commands_dir: None,
                command_separator: '-',
                tool_name_style: ToolNameStyle::Lowercase,
                global_path_prefix: "",
                local_path_prefix: "",
            },
        }
    }

    /// Get the target skills directory path for a given scope and project root.
    pub fn target_skills_dir(&self, scope: Scope, project_root: &std::path::Path) -> PathBuf {
        if let Runtime::Custom(dir) = &self.runtime {
            if scope == Scope::Global {
                let home = dirs::home_dir().unwrap_or_default();
                return home.join(dir);
            }
            return project_root.join(dir);
        }

        match scope {
            Scope::Project => project_root.join(self.skills_dir),
            Scope::Global => {
                let home = dirs::home_dir().unwrap_or_default();
                let prefix = self
                    .global_path_prefix
                    .trim_start_matches("~/")
                    .trim_end_matches('/');
                home.join(prefix).join("skills")
            }
        }
    }

    /// Get the target commands directory path (if the runtime supports commands).
    pub fn target_commands_dir(
        &self,
        scope: Scope,
        project_root: &std::path::Path,
    ) -> Option<PathBuf> {
        let commands_dir = self.commands_dir?;
        Some(match scope {
            Scope::Project => project_root.join(commands_dir),
            Scope::Global => {
                let home = dirs::home_dir().unwrap_or_default();
                let prefix = self
                    .global_path_prefix
                    .trim_start_matches("~/")
                    .trim_end_matches('/');
                home.join(prefix).join("commands")
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_from_str() {
        assert_eq!(
            Runtime::from_str_arg("claude", None).unwrap(),
            Runtime::Claude
        );
        assert_eq!(
            Runtime::from_str_arg("opencode", None).unwrap(),
            Runtime::OpenCode
        );
        assert_eq!(
            Runtime::from_str_arg("gemini", None).unwrap(),
            Runtime::Gemini
        );
        assert_eq!(
            Runtime::from_str_arg("codex", None).unwrap(),
            Runtime::Codex
        );
        assert!(Runtime::from_str_arg("skills", None).is_err());
        assert_eq!(
            Runtime::from_str_arg("skills", Some(".qwen/skills")).unwrap(),
            Runtime::Custom(".qwen/skills".to_string())
        );
    }

    #[test]
    fn test_profile_skills_dir() {
        let profile = RuntimeProfile::for_runtime(&Runtime::OpenCode);
        assert_eq!(profile.skills_dir, ".opencode/skill");
        assert_eq!(profile.commands_dir, Some(".opencode/command"));
        assert_eq!(profile.command_separator, '-');
    }

    #[test]
    fn test_target_skills_dir_project() {
        let profile = RuntimeProfile::for_runtime(&Runtime::Gemini);
        let root = std::path::Path::new("/project");
        let dir = profile.target_skills_dir(Scope::Project, root);
        assert_eq!(dir, PathBuf::from("/project/.gemini/skills"));
    }

    #[test]
    fn test_target_commands_dir_none_for_codex() {
        let profile = RuntimeProfile::for_runtime(&Runtime::Codex);
        let root = std::path::Path::new("/project");
        assert!(profile.target_commands_dir(Scope::Project, root).is_none());
    }

    #[test]
    fn test_custom_runtime_target() {
        let rt = Runtime::Custom(".qwen/skills".to_string());
        let profile = RuntimeProfile::for_runtime(&rt);
        let root = std::path::Path::new("/project");
        let dir = profile.target_skills_dir(Scope::Project, root);
        assert_eq!(dir, PathBuf::from("/project/.qwen/skills"));
    }
}

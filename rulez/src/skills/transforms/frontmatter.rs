//! YAML frontmatter conversion between runtimes.
//!
//! Claude Code skills use YAML frontmatter with fields like `allowed-tools:` (array),
//! `color:`, `name:`, etc. Each runtime has different expectations:
//!
//! - **OpenCode**: `tools:` object with `tool: true` entries, hex colors, strip `name:`
//! - **Gemini**: `tools:` array with snake_case names, strip `color:`, strip MCP tools
//! - **Codex**: similar to OpenCode but simpler

use super::tool_names::convert_tool_name;
use super::{SkillTransform, TransformContext};
use crate::skills::profiles::Runtime;
use crate::skills::transforms::colors::color_name_to_hex;

/// Transform that converts YAML frontmatter fields.
pub struct FrontmatterTransform;

impl FrontmatterTransform {
    /// Extract frontmatter and body from markdown content.
    fn split_frontmatter(content: &str) -> Option<(&str, &str)> {
        let trimmed = content.trim_start();
        if !trimmed.starts_with("---") {
            return None;
        }
        // Find the closing ---
        let rest = &trimmed[3..];
        let end = rest.find("\n---")?;
        let fm = &rest[..end];
        let body = &rest[end + 4..]; // skip \n---
        Some((fm.trim(), body))
    }

    /// Convert allowed-tools list for OpenCode format.
    fn convert_tools_opencode(frontmatter: &str, target: &Runtime) -> String {
        let mut lines: Vec<String> = Vec::new();
        let mut in_tools = false;
        let mut tool_entries: Vec<String> = Vec::new();

        for line in frontmatter.lines() {
            if line.starts_with("allowed-tools:") {
                in_tools = true;
                continue;
            }
            if in_tools {
                if let Some(tool) = line
                    .strip_prefix("  - ")
                    .or_else(|| line.strip_prefix("- "))
                {
                    let tool = tool.trim().trim_matches('"').trim_matches('\'');
                    if let Some(converted) = convert_tool_name(tool, target) {
                        tool_entries.push(format!("  {converted}: true"));
                    }
                    continue;
                }
                // End of tools list
                in_tools = false;
                if !tool_entries.is_empty() {
                    lines.push("tools:".to_string());
                    lines.append(&mut tool_entries);
                }
            }

            // Strip fields unsupported by OpenCode
            if line.starts_with("name:") {
                continue;
            }
            if line.starts_with("skills:")
                || line.starts_with("memory:")
                || line.starts_with("maxTurns:")
                || line.starts_with("permissionMode:")
                || line.starts_with("disallowedTools:")
            {
                continue;
            }

            // Convert color names to hex
            if line.starts_with("color:") {
                if let Some(color_val) = line
                    .strip_prefix("color:")
                    .map(|s| s.trim().trim_matches('"').trim_matches('\''))
                {
                    if let Some(hex) = color_name_to_hex(color_val) {
                        lines.push(format!("color: \"{hex}\""));
                        continue;
                    }
                }
            }

            lines.push(line.to_string());
        }

        // Flush remaining tools if frontmatter ended while in tools list
        if in_tools && !tool_entries.is_empty() {
            lines.push("tools:".to_string());
            lines.append(&mut tool_entries);
        }

        lines.join("\n")
    }

    /// Convert allowed-tools list for Gemini format.
    fn convert_tools_gemini(frontmatter: &str, target: &Runtime) -> String {
        let mut lines: Vec<String> = Vec::new();
        let mut in_tools = false;
        let mut tool_entries: Vec<String> = Vec::new();

        for line in frontmatter.lines() {
            if line.starts_with("allowed-tools:") {
                in_tools = true;
                continue;
            }
            if in_tools {
                if let Some(tool) = line
                    .strip_prefix("  - ")
                    .or_else(|| line.strip_prefix("- "))
                {
                    let tool = tool.trim().trim_matches('"').trim_matches('\'');
                    if let Some(converted) = convert_tool_name(tool, target) {
                        tool_entries.push(format!("  - {converted}"));
                    }
                    continue;
                }
                in_tools = false;
                if !tool_entries.is_empty() {
                    lines.push("tools:".to_string());
                    lines.append(&mut tool_entries);
                }
            }

            // Strip fields unsupported by Gemini
            if line.starts_with("color:") || line.starts_with("skills:") {
                continue;
            }

            lines.push(line.to_string());
        }

        if in_tools && !tool_entries.is_empty() {
            lines.push("tools:".to_string());
            lines.append(&mut tool_entries);
        }

        lines.join("\n")
    }
}

impl SkillTransform for FrontmatterTransform {
    fn transform_content(&self, content: &str, ctx: &TransformContext) -> String {
        if matches!(ctx.target_runtime(), Runtime::Claude) {
            return content.to_string();
        }

        let Some((frontmatter, body)) = Self::split_frontmatter(content) else {
            return content.to_string();
        };

        let converted_fm = match ctx.target_runtime() {
            Runtime::OpenCode | Runtime::Codex | Runtime::Custom(_) => {
                Self::convert_tools_opencode(frontmatter, ctx.target_runtime())
            }
            Runtime::Gemini => Self::convert_tools_gemini(frontmatter, ctx.target_runtime()),
            Runtime::Claude => unreachable!(),
        };

        format!("---\n{converted_fm}\n---{body}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_frontmatter() {
        let content = "---\nname: test\ndescription: A test\n---\n\n# Content";
        let (fm, body) = FrontmatterTransform::split_frontmatter(content).unwrap();
        assert_eq!(fm, "name: test\ndescription: A test");
        assert!(body.contains("# Content"));
    }

    #[test]
    fn test_opencode_tools_conversion() {
        let fm = "name: my-skill\ndescription: Test\nallowed-tools:\n  - Read\n  - Write\n  - AskUserQuestion";
        let result = FrontmatterTransform::convert_tools_opencode(fm, &Runtime::OpenCode);
        assert!(result.contains("tools:"));
        assert!(result.contains("  read: true"));
        assert!(result.contains("  write: true"));
        assert!(result.contains("  question: true"));
        // name: should be stripped
        assert!(!result.contains("name:"));
    }

    #[test]
    fn test_gemini_tools_conversion() {
        let fm = "description: Test\nallowed-tools:\n  - Read\n  - Bash\n  - Task\ncolor: cyan";
        let result = FrontmatterTransform::convert_tools_gemini(fm, &Runtime::Gemini);
        assert!(result.contains("  - read_file"));
        assert!(result.contains("  - run_shell_command"));
        // Task excluded for Gemini
        assert!(!result.contains("task"));
        // Color stripped for Gemini
        assert!(!result.contains("color"));
    }

    #[test]
    fn test_full_content_transform_opencode() {
        let t = FrontmatterTransform;
        let ctx = TransformContext::new(&Runtime::OpenCode);
        let input = "---\nname: test\ndescription: A skill\nallowed-tools:\n  - Read\n  - Write\n---\n\n# Skill Content";
        let result = t.transform_content(input, &ctx);
        assert!(result.starts_with("---\n"));
        assert!(result.contains("tools:"));
        assert!(result.contains("# Skill Content"));
        assert!(!result.contains("allowed-tools:"));
    }
}

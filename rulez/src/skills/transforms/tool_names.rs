//! Tool name conversion between runtimes.
//!
//! Maps Claude Code PascalCase tool names to runtime-specific equivalents.
//! These are the **reverse** of the adapter `map_tool_name()` functions.

use super::{SkillTransform, TransformContext};
use crate::skills::profiles::{Runtime, ToolNameStyle};
use regex::Regex;
use std::sync::LazyLock;

/// Claude Code canonical tool name -> OpenCode equivalent.
fn claude_to_opencode(name: &str) -> &str {
    match name {
        "AskUserQuestion" => "question",
        "TodoWrite" => "todowrite",
        "WebFetch" => "webfetch",
        "WebSearch" => "websearch",
        // Most tools just lowercase
        _ => "",
    }
}

/// Claude Code canonical tool name -> Gemini CLI equivalent.
fn claude_to_gemini(name: &str) -> Option<&str> {
    match name {
        "Read" => Some("read_file"),
        "Write" => Some("write_file"),
        "Edit" => Some("replace"),
        "Bash" => Some("run_shell_command"),
        "Glob" => Some("glob"),
        "Grep" => Some("search_file_content"),
        "WebSearch" => Some("google_web_search"),
        "WebFetch" => Some("web_fetch"),
        "TodoWrite" => Some("write_todos"),
        "AskUserQuestion" => Some("ask_user"),
        // Task is excluded for Gemini (auto-registered)
        "Task" => None,
        _ => Some(""),
    }
}

/// Convert a single Claude tool name to the target runtime's format.
pub fn convert_tool_name(name: &str, target: &Runtime) -> Option<String> {
    // MCP tools pass through unchanged (except Gemini excludes them)
    if name.starts_with("mcp__") {
        return match target {
            Runtime::Gemini => None, // Gemini auto-discovers MCP
            _ => Some(name.to_string()),
        };
    }

    match target {
        Runtime::Claude => Some(name.to_string()),
        Runtime::OpenCode | Runtime::Codex | Runtime::Custom(_) => {
            let mapped = claude_to_opencode(name);
            if mapped.is_empty() {
                Some(name.to_lowercase())
            } else {
                Some(mapped.to_string())
            }
        }
        Runtime::Gemini => claude_to_gemini(name).map(|mapped| {
            if mapped.is_empty() {
                name.to_lowercase()
            } else {
                mapped.to_string()
            }
        }),
    }
}

// Regex to match PascalCase tool names in markdown content.
// Matches known tool names as whole words (not inside other words).
static TOOL_NAME_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\b(Read|Write|Edit|Bash|Glob|Grep|WebSearch|WebFetch|TodoWrite|AskUserQuestion|Task)\b",
    )
    .unwrap()
});

/// Transform that rewrites tool names in markdown content.
pub struct ToolNameTransform;

impl SkillTransform for ToolNameTransform {
    fn transform_content(&self, content: &str, ctx: &TransformContext) -> String {
        if ctx.target_profile.tool_name_style == ToolNameStyle::PascalCase {
            return content.to_string(); // Claude -> Claude, no change
        }

        TOOL_NAME_RE
            .replace_all(content, |caps: &regex::Captures| {
                let name = caps.get(0).unwrap().as_str();
                convert_tool_name(name, ctx.target_runtime()).unwrap_or_default()
            })
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_opencode() {
        assert_eq!(
            convert_tool_name("Read", &Runtime::OpenCode),
            Some("read".to_string())
        );
        assert_eq!(
            convert_tool_name("AskUserQuestion", &Runtime::OpenCode),
            Some("question".to_string())
        );
        assert_eq!(
            convert_tool_name("WebFetch", &Runtime::OpenCode),
            Some("webfetch".to_string())
        );
    }

    #[test]
    fn test_convert_gemini() {
        assert_eq!(
            convert_tool_name("Bash", &Runtime::Gemini),
            Some("run_shell_command".to_string())
        );
        assert_eq!(
            convert_tool_name("Grep", &Runtime::Gemini),
            Some("search_file_content".to_string())
        );
        // Task excluded for Gemini
        assert_eq!(convert_tool_name("Task", &Runtime::Gemini), None);
    }

    #[test]
    fn test_mcp_tools() {
        assert_eq!(
            convert_tool_name("mcp__github__search", &Runtime::OpenCode),
            Some("mcp__github__search".to_string())
        );
        // Gemini excludes MCP tools
        assert_eq!(
            convert_tool_name("mcp__github__search", &Runtime::Gemini),
            None
        );
    }

    #[test]
    fn test_transform_content() {
        let t = ToolNameTransform;
        let ctx = TransformContext::new(&Runtime::OpenCode);
        let input = "Use the Read tool and Bash to check files.";
        let result = t.transform_content(input, &ctx);
        assert_eq!(result, "Use the read tool and bash to check files.");
    }

    #[test]
    fn test_transform_gemini_content() {
        let t = ToolNameTransform;
        let ctx = TransformContext::new(&Runtime::Gemini);
        let input = "The Grep tool searches content.";
        let result = t.transform_content(input, &ctx);
        assert_eq!(result, "The search_file_content tool searches content.");
    }
}

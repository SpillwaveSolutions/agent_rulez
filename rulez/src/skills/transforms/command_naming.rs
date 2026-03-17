//! Command filename flattening and cross-reference rewriting.
//!
//! Claude Code uses dot-separated namespaces: `speckit.analyze.md` invoked as `/speckit.analyze`.
//! OpenCode/Codex use hyphen-separated flat names: `speckit-analyze.md` invoked as `/speckit-analyze`.

use super::{SkillTransform, TransformContext};
use crate::skills::profiles::Runtime;
use regex::Regex;
use std::sync::LazyLock;

// Match slash-command cross-references like `/speckit.analyze` or `/speckit.plan`
static SLASH_CMD_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(/\w+)\.(\w+)").unwrap());

/// Transform that flattens dot-separated command names to hyphens.
pub struct CommandNamingTransform;

impl CommandNamingTransform {
    /// Convert a command filename from Claude format to target format.
    /// e.g., "speckit.analyze.md" -> "speckit-analyze.md"
    fn flatten_filename(filename: &str, separator: char) -> String {
        let without_ext = filename.trim_end_matches(".md");
        let flattened = without_ext.replace('.', &separator.to_string());
        format!("{flattened}.md")
    }
}

impl SkillTransform for CommandNamingTransform {
    fn transform_content(&self, content: &str, ctx: &TransformContext) -> String {
        if matches!(ctx.target_runtime(), Runtime::Claude) {
            return content.to_string();
        }

        let sep = ctx.target_profile.command_separator;
        // Rewrite slash-command cross-references: /speckit.plan -> /speckit-plan
        SLASH_CMD_RE
            .replace_all(content, |caps: &regex::Captures| {
                let prefix = caps.get(1).unwrap().as_str();
                let suffix = caps.get(2).unwrap().as_str();
                format!("{prefix}{sep}{suffix}")
            })
            .to_string()
    }

    fn transform_command_filename(&self, filename: &str, ctx: &TransformContext) -> String {
        if matches!(ctx.target_runtime(), Runtime::Claude) {
            return filename.to_string();
        }
        Self::flatten_filename(filename, ctx.target_profile.command_separator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_filename() {
        assert_eq!(
            CommandNamingTransform::flatten_filename("speckit.analyze.md", '-'),
            "speckit-analyze.md"
        );
        assert_eq!(
            CommandNamingTransform::flatten_filename("cch-release.md", '-'),
            "cch-release.md"
        );
    }

    #[test]
    fn test_cross_reference_rewrite() {
        let t = CommandNamingTransform;
        let ctx = TransformContext::new(&Runtime::OpenCode);
        let input = "Run `/speckit.plan` first, then `/speckit.tasks` to generate tasks.";
        let result = t.transform_content(input, &ctx);
        assert_eq!(
            result,
            "Run `/speckit-plan` first, then `/speckit-tasks` to generate tasks."
        );
    }

    #[test]
    fn test_no_rewrite_for_claude() {
        let t = CommandNamingTransform;
        let ctx = TransformContext::new(&Runtime::Claude);
        let input = "Run `/speckit.plan` to plan.";
        let result = t.transform_content(input, &ctx);
        assert_eq!(result, input);
    }
}

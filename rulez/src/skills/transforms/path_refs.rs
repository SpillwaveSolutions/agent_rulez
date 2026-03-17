//! Path reference rewriting for cross-runtime skill content.
//!
//! Rewrites `~/.claude/` and `.claude/` references to the target runtime's
//! equivalent paths.

use super::{SkillTransform, TransformContext};
use crate::skills::profiles::Runtime;

/// Transform that rewrites config directory path references.
pub struct PathRefTransform;

impl SkillTransform for PathRefTransform {
    fn transform_content(&self, content: &str, ctx: &TransformContext) -> String {
        if matches!(ctx.target_runtime(), Runtime::Claude) {
            return content.to_string();
        }

        let target = &ctx.target_profile;
        let source = &ctx.source_profile;

        content
            .replace(source.global_path_prefix, target.global_path_prefix)
            .replace(source.local_path_prefix, target.local_path_prefix)
            // Also handle $HOME style
            .replace(
                &format!(
                    "$HOME/{}",
                    source.global_path_prefix.trim_start_matches("~/")
                ),
                &format!(
                    "$HOME/{}",
                    target.global_path_prefix.trim_start_matches("~/")
                ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rewrite_opencode_paths() {
        let t = PathRefTransform;
        let ctx = TransformContext::new(&Runtime::OpenCode);
        let input = "Config at ~/.claude/hooks.yaml and .claude/settings.json";
        let result = t.transform_content(input, &ctx);
        assert_eq!(
            result,
            "Config at ~/.config/opencode/hooks.yaml and .opencode/settings.json"
        );
    }

    #[test]
    fn test_rewrite_gemini_paths() {
        let t = PathRefTransform;
        let ctx = TransformContext::new(&Runtime::Gemini);
        let input = "See ~/.claude/skills/ for available skills.";
        let result = t.transform_content(input, &ctx);
        assert_eq!(result, "See ~/.gemini/skills/ for available skills.");
    }

    #[test]
    fn test_rewrite_home_var() {
        let t = PathRefTransform;
        let ctx = TransformContext::new(&Runtime::Codex);
        let input = "Path: $HOME/.claude/logs/";
        let result = t.transform_content(input, &ctx);
        assert_eq!(result, "Path: $HOME/.codex/logs/");
    }

    #[test]
    fn test_no_rewrite_for_claude() {
        let t = PathRefTransform;
        let ctx = TransformContext::new(&Runtime::Claude);
        let input = "Keep ~/.claude/ as-is.";
        let result = t.transform_content(input, &ctx);
        assert_eq!(result, input);
    }
}

//! Transform pipeline that chains multiple transformations for a target runtime.

use crate::skills::profiles::Runtime;
use crate::skills::transforms::{
    SkillTransform, TransformContext, command_naming::CommandNamingTransform,
    frontmatter::FrontmatterTransform, path_refs::PathRefTransform, tool_names::ToolNameTransform,
};

/// An ordered chain of content transformations for a specific target runtime.
pub struct TransformPipeline {
    transforms: Vec<Box<dyn SkillTransform>>,
    ctx: TransformContext,
}

impl TransformPipeline {
    /// Build a pipeline for the given target runtime.
    ///
    /// Order matters: frontmatter is processed first (to convert tool lists),
    /// then tool names in body content, then paths, then command cross-references.
    pub fn for_runtime(target: &Runtime) -> Self {
        let ctx = TransformContext::new(target);

        // Claude -> Claude needs no transforms
        if matches!(target, Runtime::Claude) {
            return Self {
                transforms: vec![],
                ctx,
            };
        }

        let transforms: Vec<Box<dyn SkillTransform>> = vec![
            Box::new(FrontmatterTransform),
            Box::new(ToolNameTransform),
            Box::new(PathRefTransform),
            Box::new(CommandNamingTransform),
        ];

        Self { transforms, ctx }
    }

    /// Apply all transforms to content.
    pub fn transform_content(&self, content: &str) -> String {
        let mut result = content.to_string();
        for t in &self.transforms {
            result = t.transform_content(&result, &self.ctx);
        }
        result
    }

    /// Transform a command filename.
    pub fn transform_command_filename(&self, filename: &str) -> String {
        let mut result = filename.to_string();
        for t in &self.transforms {
            result = t.transform_command_filename(&result, &self.ctx);
        }
        result
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn context(&self) -> &TransformContext {
        &self.ctx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_pipeline_is_noop() {
        let pipeline = TransformPipeline::for_runtime(&Runtime::Claude);
        let input = "---\nallowed-tools:\n  - Read\n---\nUse Read in ~/.claude/";
        assert_eq!(pipeline.transform_content(input), input);
    }

    #[test]
    fn test_opencode_full_pipeline() {
        let pipeline = TransformPipeline::for_runtime(&Runtime::OpenCode);
        let input = "---\nname: test\ndescription: A test skill\nallowed-tools:\n  - Read\n  - Bash\n---\n\nUse the Read tool at ~/.claude/skills/. See /speckit.plan for planning.";

        let result = pipeline.transform_content(input);

        // Frontmatter: allowed-tools -> tools object, name stripped
        assert!(result.contains("tools:"));
        assert!(result.contains("  read: true"));
        assert!(result.contains("  bash: true"));
        assert!(!result.contains("allowed-tools:"));

        // Tool names in body: Read -> read, Bash -> bash
        assert!(result.contains("read tool"));

        // Paths: ~/.claude/ -> ~/.config/opencode/
        assert!(result.contains("~/.config/opencode/"));
        assert!(!result.contains("~/.claude/"));

        // Command refs: /speckit.plan -> /speckit-plan
        assert!(result.contains("/speckit-plan"));
    }

    #[test]
    fn test_command_filename_transform() {
        let pipeline = TransformPipeline::for_runtime(&Runtime::OpenCode);
        assert_eq!(
            pipeline.transform_command_filename("speckit.analyze.md"),
            "speckit-analyze.md"
        );
        assert_eq!(
            pipeline.transform_command_filename("cch-release.md"),
            "cch-release.md"
        );
    }

    #[test]
    fn test_gemini_pipeline() {
        let pipeline = TransformPipeline::for_runtime(&Runtime::Gemini);
        let input = "---\ndescription: Test\nallowed-tools:\n  - Read\n  - Task\ncolor: cyan\n---\n\nContent at ~/.claude/hooks.yaml";
        let result = pipeline.transform_content(input);

        // Tools: Read -> read_file, Task excluded
        assert!(result.contains("  - read_file"));
        assert!(!result.contains("task"));

        // Color stripped
        assert!(!result.contains("color:"));
        assert!(!result.contains("cyan"));

        // Paths rewritten
        assert!(result.contains("~/.gemini/"));
    }
}

//! Content transformation traits and implementations for multi-runtime conversion.

pub mod colors;
pub mod command_naming;
pub mod frontmatter;
pub mod path_refs;
pub mod tool_names;

use crate::skills::profiles::{Runtime, RuntimeProfile};

/// Context passed to each transformation step.
#[derive(Debug, Clone)]
pub struct TransformContext {
    pub source_profile: RuntimeProfile,
    pub target_profile: RuntimeProfile,
}

impl TransformContext {
    pub fn new(target: &Runtime) -> Self {
        Self {
            source_profile: RuntimeProfile::for_runtime(&Runtime::Claude),
            target_profile: RuntimeProfile::for_runtime(target),
        }
    }

    pub fn target_runtime(&self) -> &Runtime {
        &self.target_profile.runtime
    }
}

/// A content transformation step.
pub trait SkillTransform: Send + Sync {
    /// Transform file content (markdown, YAML frontmatter, etc.)
    fn transform_content(&self, content: &str, ctx: &TransformContext) -> String;

    /// Transform a command filename (e.g., "speckit.analyze.md" -> "speckit-analyze.md")
    fn transform_command_filename(&self, filename: &str, ctx: &TransformContext) -> String {
        let _ = ctx;
        filename.to_string()
    }
}

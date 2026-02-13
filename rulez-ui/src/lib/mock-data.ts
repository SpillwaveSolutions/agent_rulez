/**
 * Mock data module for browser testing mode.
 *
 * Provides sample RuleZ configuration data when running outside of Tauri.
 */

import type { ConfigFile } from "@/types";

// In-memory storage for mock configs
const mockConfigs = new Map<string, string>();

// Sample hooks.yaml content
const SAMPLE_GLOBAL_CONFIG = `# Global RuleZ Configuration
version: "1.0"

settings:
  log_level: "info"
  fail_open: true

rules:
  - name: block-force-push
    description: "Block force push to main/master branches"
    matchers:
      tools: ["Bash"]
      command_match: "git push.*(--force|-f).*(main|master)"
    actions:
      block: true

  - name: inject-python-context
    description: "Inject Python best practices context"
    matchers:
      tools: ["Write", "Edit"]
      extensions: [".py"]
    actions:
      inject: "Follow PEP 8 style guidelines"

  - name: block-rm-rf
    description: "Block dangerous rm -rf commands"
    matchers:
      tools: ["Bash"]
      command_match: "rm\\\\s+-rf\\\\s+/"
    actions:
      block: true
`;

const SAMPLE_PROJECT_CONFIG = `# Project-specific RuleZ Configuration
version: "1.0"

settings:
  log_level: "debug"

rules:
  - name: project-security-check
    description: "Run security scanner on commits"
    matchers:
      tools: ["Bash"]
      command_match: "git commit"
    actions:
      run: "./scripts/security-check.sh"
`;

// Initialize default configs
mockConfigs.set("~/.claude/hooks.yaml", SAMPLE_GLOBAL_CONFIG);
mockConfigs.set(".claude/hooks.yaml", SAMPLE_PROJECT_CONFIG);

/**
 * Get list of mock config files
 */
export function getMockConfigFiles(projectDir?: string): ConfigFile[] {
  const files: ConfigFile[] = [
    {
      path: "~/.claude/hooks.yaml",
      exists: true,
      modified: false,
      hasErrors: false,
    },
  ];

  if (projectDir || typeof window !== "undefined") {
    files.push({
      path: ".claude/hooks.yaml",
      exists: true,
      modified: false,
      hasErrors: false,
    });
  }

  return files;
}

/**
 * Get mock config content by path
 */
export function getMockConfig(path: string): string {
  const content = mockConfigs.get(path);
  if (!content) {
    // Return empty config for new files
    return `# New RuleZ Configuration
version: "1.0"

settings:
  log_level: "info"

rules: []
`;
  }
  return content;
}

/**
 * Set mock config content (for testing writes)
 */
export function setMockConfig(path: string, content: string): void {
  mockConfigs.set(path, content);
}

/**
 * Get sample rule templates
 */
export function getRuleTemplates(): Array<{ name: string; description: string; yaml: string }> {
  return [
    {
      name: "Block Force Push",
      description: "Prevent force pushing to protected branches",
      yaml: `- name: block-force-push
  description: "Block force push to main/master"
  matchers:
    tools: ["Bash"]
    command_match: "git push.*(--force|-f)"
  actions:
    block: true`,
    },
    {
      name: "Inject Context",
      description: "Add context for specific file types",
      yaml: `- name: inject-context
  description: "Inject coding guidelines"
  matchers:
    tools: ["Write", "Edit"]
    extensions: [".ts", ".tsx"]
  actions:
    inject: "Follow project coding standards"`,
    },
    {
      name: "Run Script",
      description: "Execute a script before a command",
      yaml: `- name: run-script
  description: "Run validation script"
  matchers:
    tools: ["Bash"]
    command_match: "git commit"
  actions:
    run: "./scripts/validate.sh"`,
    },
  ];
}

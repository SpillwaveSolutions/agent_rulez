/**
 * Tauri abstraction layer for dual-mode architecture.
 *
 * When running in Tauri desktop mode, uses actual Tauri IPC commands.
 * When running in browser (for testing), uses web fallbacks with mock data.
 */

import type {
  ConfigFile,
  DebugParams,
  DebugResult,
  LogEntryDto,
  LogQueryParams,
  LogStats,
} from "@/types";

/**
 * Check if running inside Tauri desktop app
 */
export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI__" in window;
}

/**
 * List available config files (global and project)
 */
export async function listConfigFiles(projectDir?: string): Promise<ConfigFile[]> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<ConfigFile[]>("list_config_files", { projectDir });
  }
  return mockListConfigFiles(projectDir);
}

/**
 * Read config file content
 */
export async function readConfig(path: string): Promise<string> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<string>("read_config", { path });
  }
  return mockReadConfig(path);
}

/**
 * Write config file content
 */
export async function writeConfig(path: string, content: string): Promise<void> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<void>("write_config", { path, content });
  }
  return mockWriteConfig(path, content);
}

/**
 * Run RuleZ debug command
 */
export async function runDebug(params: DebugParams): Promise<DebugResult> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<DebugResult>("run_debug", params as unknown as Record<string, unknown>);
  }
  return mockRunDebug(params);
}

/**
 * Validate config file using RuleZ
 */
export async function validateConfig(path: string): Promise<{ valid: boolean; errors: string[] }> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<{ valid: boolean; errors: string[] }>("validate_config", { path });
  }
  return mockValidateConfig(path);
}

/**
 * Check if the RuleZ binary is installed and accessible
 */
export async function checkBinary(): Promise<{ found: boolean; path: string | null }> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<{ found: boolean; path: string | null }>("check_binary");
  }
  return mockCheckBinary();
}

/**
 * Generate a sample hooks.yaml config and write it to the global config path.
 * Returns the path where the config was written.
 */
export async function generateSampleConfig(): Promise<string> {
  const configPath = "~/.claude/hooks.yaml";
  const content = SAMPLE_HOOKS_YAML;
  await writeConfig(configPath, content);
  return configPath;
}

/**
 * Read and filter log entries from rulez.log
 */
export async function readLogs(params: LogQueryParams): Promise<LogEntryDto[]> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<LogEntryDto[]>("read_logs", { params });
  }
  return mockReadLogs(params);
}

/**
 * Import a config file from disk via file picker dialog.
 * Returns the selected file's path and content, or null if cancelled.
 */
export async function importConfigFile(): Promise<{ path: string; content: string } | null> {
  if (isTauri()) {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const { readTextFile } = await import("@tauri-apps/plugin-fs");
    const selected = await open({
      filters: [{ name: "YAML", extensions: ["yaml", "yml"] }],
      multiple: false,
    });
    if (!selected) return null;
    const filePath = typeof selected === "string" ? selected : selected;
    const content = await readTextFile(filePath);
    return { path: filePath, content };
  }
  return mockImportConfigFile();
}

/**
 * Export config content to a file via save dialog.
 * Returns true if exported, false if cancelled.
 */
export async function exportConfigFile(content: string, defaultName?: string): Promise<boolean> {
  if (isTauri()) {
    const { save } = await import("@tauri-apps/plugin-dialog");
    const { writeTextFile } = await import("@tauri-apps/plugin-fs");
    const path = await save({
      filters: [{ name: "YAML", extensions: ["yaml", "yml"] }],
      defaultPath: defaultName ?? "hooks.yaml",
    });
    if (!path) return false;
    await writeTextFile(path, content);
    return true;
  }
  return mockExportConfigFile(content, defaultName);
}

/**
 * Get log file statistics
 */
export async function getLogStats(): Promise<LogStats> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<LogStats>("get_log_stats");
  }
  return mockGetLogStats();
}

// ============================================================================
// Mock implementations for browser testing mode
// ============================================================================

import { getMockConfig, getMockConfigFiles, setMockConfig } from "./mock-data";

async function mockListConfigFiles(projectDir?: string): Promise<ConfigFile[]> {
  // Simulate network delay
  await delay(50);
  return getMockConfigFiles(projectDir);
}

async function mockReadConfig(path: string): Promise<string> {
  await delay(30);
  return getMockConfig(path);
}

async function mockWriteConfig(path: string, content: string): Promise<void> {
  await delay(30);
  setMockConfig(path, content);
}

async function mockRunDebug(params: DebugParams): Promise<DebugResult> {
  await delay(100);

  // Check for injected mock response (used by E2E tests)
  if (
    typeof window !== "undefined" &&
    (window as unknown as { __rulezMockDebugResponse?: DebugResult }).__rulezMockDebugResponse
  ) {
    return (window as unknown as { __rulezMockDebugResponse: DebugResult })
      .__rulezMockDebugResponse;
  }

  // Simulate debug evaluation
  const evaluations = [
    {
      ruleName: "block-force-push",
      matched: params.command?.includes("--force") || params.command?.includes("-f") || false,
      timeMs: 0.8,
      details: "command_match evaluated",
      pattern: "git push.*(--force|-f).*(main|master)",
      input: params.command,
    },
    {
      ruleName: "inject-python-context",
      matched: false,
      timeMs: 0.1,
      details: "tool mismatch",
    },
  ];

  const matched = evaluations.filter((e) => e.matched);
  const isBlocked = matched.some((e) => e.ruleName === "block-force-push");

  return {
    outcome: isBlocked ? "Block" : "Allow",
    reason: isBlocked ? "Force push to main/master is prohibited" : undefined,
    matchedRules: matched.map((e) => e.ruleName),
    evaluationTimeMs: evaluations.reduce((sum, e) => sum + e.timeMs, 0),
    evaluations,
  };
}

async function mockValidateConfig(_path: string): Promise<{ valid: boolean; errors: string[] }> {
  await delay(50);
  // In mock mode, always return valid
  return { valid: true, errors: [] };
}

async function mockCheckBinary(): Promise<{ found: boolean; path: string | null }> {
  await delay(50);
  return { found: true, path: "/usr/local/bin/rulez" };
}

async function mockImportConfigFile(): Promise<{ path: string; content: string } | null> {
  await delay(100);
  // In browser mode, simulate importing the global config
  const content = getMockConfig("~/.claude/hooks.yaml");
  return { path: "imported-hooks.yaml", content };
}

async function mockExportConfigFile(content: string, _defaultName?: string): Promise<boolean> {
  // In browser mode, trigger a download via Blob
  const blob = new Blob([content], { type: "text/yaml" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = _defaultName ?? "hooks.yaml";
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
  return true;
}

function delay(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

// Mock log data generator
function generateMockLogEntries(count: number): LogEntryDto[] {
  const eventTypes = ["PreToolUse", "PostToolUse", "SessionStart", "UserPromptSubmit"];
  const tools = ["Bash", "Write", "Edit", "Read", "Glob", "Grep"];
  const outcomes: Array<"allow" | "block" | "inject"> = ["allow", "block", "inject"];
  const decisions: Array<"allowed" | "blocked" | "warned" | "audited"> = [
    "allowed",
    "blocked",
    "warned",
    "audited",
  ];
  const modes: Array<"enforce" | "warn" | "audit"> = ["enforce", "warn", "audit"];
  const rules = ["block-force-push", "inject-python-context", "block-rm-rf", "security-check"];

  const entries: LogEntryDto[] = [];
  const now = Date.now();

  for (let i = 0; i < count; i++) {
    const outcomeIdx = i % 10 === 0 ? 1 : i % 7 === 0 ? 2 : 0; // ~10% block, ~14% inject, rest allow
    const outcome = outcomes[outcomeIdx] ?? "allow";
    const decision =
      outcome === "block"
        ? "blocked"
        : outcome === "inject"
          ? "allowed"
          : (decisions[i % decisions.length] ?? "allowed");

    entries.push({
      timestamp: new Date(now - i * 60000).toISOString(),
      eventType: eventTypes[i % eventTypes.length] ?? "PreToolUse",
      sessionId: `session-${String(Math.floor(i / 20)).padStart(4, "0")}`,
      toolName:
        eventTypes[i % eventTypes.length] === "SessionStart"
          ? null
          : (tools[i % tools.length] ?? "Bash"),
      rulesMatched: outcome !== "allow" ? [rules[i % rules.length] ?? "rule"] : [],
      outcome,
      processingMs: Math.floor(Math.random() * 10),
      rulesEvaluated: 3 + (i % 5),
      decision,
      mode: modes[i % modes.length] ?? "enforce",
      priority: null,
      responseContinue: outcome !== "block",
      responseReason: outcome === "block" ? "Policy violation detected" : null,
      eventDetailCommand: eventTypes[i % eventTypes.length] === "PreToolUse" ? "git status" : null,
      eventDetailFilePath:
        eventTypes[i % eventTypes.length] === "PostToolUse" ? `/src/lib/example-${i}.ts` : null,
    });
  }

  return entries;
}

async function mockReadLogs(_params: LogQueryParams): Promise<LogEntryDto[]> {
  await delay(100);
  return generateMockLogEntries(50);
}

async function mockGetLogStats(): Promise<LogStats> {
  await delay(50);
  return {
    totalEntries: 14382,
    fileSizeBytes: 5_200_000,
    oldestEntry: new Date(Date.now() - 86400000).toISOString(),
    newestEntry: new Date().toISOString(),
  };
}

// ============================================================================
// Sample config template for onboarding
// ============================================================================

const SAMPLE_HOOKS_YAML = `# RuleZ Configuration
# Location: ~/.claude/hooks.yaml
# Documentation: https://github.com/SpillwaveSolutions/code_agent_context_hooks

version: "1.0"

# Global settings
settings:
  debug_logs: false
  log_level: info
  fail_open: true
  script_timeout: 5

# Policy rules
rules:
  # Block force push to protected branches
  - name: block-force-push
    description: Prevent force push to main/master
    matchers:
      tools: [Bash]
      command_match: "git push.*(--force|-f).*(main|master)"
    actions:
      block: true
    metadata:
      priority: 100
      enabled: true

  # Block hard reset on protected branches
  - name: block-hard-reset
    description: Prevent destructive git reset operations
    matchers:
      tools: [Bash]
      command_match: "git reset --hard"
    actions:
      block: true
    metadata:
      priority: 90
      enabled: true
`;

/**
 * Tauri abstraction layer for dual-mode architecture.
 *
 * When running in Tauri desktop mode, uses actual Tauri IPC commands.
 * When running in browser (for testing), uses web fallbacks with mock data.
 */

import type { ConfigFile, DebugParams, DebugResult } from "@/types";

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

function delay(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

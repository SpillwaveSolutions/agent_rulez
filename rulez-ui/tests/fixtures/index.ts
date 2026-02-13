/**
 * Test fixtures for Playwright E2E tests.
 *
 * Usage:
 *   import { loadMockConfig, getBlockScenarios } from "./fixtures";
 */

import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));

/**
 * Load a mock YAML configuration
 */
export function loadMockConfig(name: string): string {
  const configPath = join(__dirname, "mock-configs", `${name}.yaml`);
  return readFileSync(configPath, "utf-8");
}

/**
 * Load event scenarios from JSON fixture
 */
function loadEventScenarios(filename: string): EventScenarios {
  const scenariosPath = join(__dirname, "events", filename);
  const content = readFileSync(scenariosPath, "utf-8");
  return JSON.parse(content) as EventScenarios;
}

/**
 * Event scenario type definition
 */
export interface EventScenario {
  name: string;
  event: {
    hook_event_name: string;
    tool_name: string;
    tool_input: Record<string, string>;
  };
  expectedOutcome: "Allow" | "Block" | "Inject";
  expectedReason?: string;
  expectedContext?: string;
}

interface EventScenarios {
  description: string;
  scenarios: EventScenario[];
}

/**
 * Get block event scenarios
 */
export function getBlockScenarios(): EventScenario[] {
  return loadEventScenarios("block-scenarios.json").scenarios;
}

/**
 * Get allow event scenarios
 */
export function getAllowScenarios(): EventScenario[] {
  return loadEventScenarios("allow-scenarios.json").scenarios;
}

/**
 * Get inject event scenarios
 */
export function getInjectScenarios(): EventScenario[] {
  return loadEventScenarios("inject-scenarios.json").scenarios;
}

/**
 * Mock config file names
 */
export const mockConfigs = {
  validBasic: "valid-basic",
  invalidSyntax: "invalid-syntax",
  empty: "empty",
  large: "large",
} as const;

/**
 * Pre-loaded mock configurations
 */
export const mockConfigContents = {
  validBasic: loadMockConfig(mockConfigs.validBasic),
  invalidSyntax: loadMockConfig(mockConfigs.invalidSyntax),
  empty: loadMockConfig(mockConfigs.empty),
  large: loadMockConfig(mockConfigs.large),
};

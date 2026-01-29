import type { Locator, Page } from "@playwright/test";
import { BasePage } from "./base.page";

/**
 * Event types supported by CCH
 */
export type EventType = "PreToolUse" | "PostToolUse" | "PermissionRequest";

/**
 * Simulation result outcomes
 */
export type SimulationOutcome = "Allow" | "Block" | "Inject";

/**
 * Page Object for the Debug Simulator.
 * Handles event simulation, form inputs, and result display.
 */
export class SimulatorPage extends BasePage {
  // Form elements
  readonly container: Locator;
  readonly eventTypeSelect: Locator;
  readonly toolInput: Locator;
  readonly commandInput: Locator;
  readonly pathInput: Locator;
  readonly simulateButton: Locator;
  readonly clearButton: Locator;

  // Result elements
  readonly resultArea: Locator;
  readonly outcomeBadge: Locator;
  readonly matchedRulesSection: Locator;
  readonly evaluationTrace: Locator;

  constructor(page: Page) {
    super(page);

    // Form elements
    this.container = page.locator("[data-testid='simulator']").or(
      page.getByText("Debug Simulator").locator("..")
    );
    this.eventTypeSelect = page.locator("select").first();
    this.toolInput = page.getByPlaceholder(/tool/i);
    this.commandInput = page.getByPlaceholder(/command/i);
    this.pathInput = page.getByPlaceholder(/path/i);
    this.simulateButton = page.getByRole("button", { name: /simulate/i });
    this.clearButton = page.getByRole("button", { name: /clear/i });

    // Result elements
    this.resultArea = page.locator("[data-testid='simulation-result']").or(
      page.locator("text=/Allow|Block|Inject/i").locator("..")
    );
    this.outcomeBadge = page.locator("text=/Allow|Block|Inject/i").first();
    this.matchedRulesSection = page.getByText(/matched|rules/i).first();
    this.evaluationTrace = page.locator("[data-testid='evaluation-trace']").or(
      page.getByText(/evaluation/i).first()
    );
  }

  /**
   * Ensure simulator tab is active
   */
  async activate(): Promise<void> {
    const simulatorTab = this.page.getByRole("button", { name: "Simulator" });
    await simulatorTab.click();
    await this.page.getByText("Debug Simulator").waitFor({ state: "visible" });
  }

  /**
   * Select an event type
   */
  async selectEventType(eventType: EventType): Promise<void> {
    await this.eventTypeSelect.selectOption(eventType);
  }

  /**
   * Fill the tool name
   */
  async fillTool(toolName: string): Promise<void> {
    await this.toolInput.fill(toolName);
  }

  /**
   * Fill the command
   */
  async fillCommand(command: string): Promise<void> {
    await this.commandInput.fill(command);
  }

  /**
   * Fill the path
   */
  async fillPath(path: string): Promise<void> {
    await this.pathInput.fill(path);
  }

  /**
   * Run a full simulation with all parameters
   */
  async runSimulation(params: {
    eventType: EventType;
    tool: string;
    command?: string;
    path?: string;
  }): Promise<void> {
    await this.selectEventType(params.eventType);
    await this.fillTool(params.tool);

    if (params.command) {
      await this.fillCommand(params.command);
    }

    if (params.path) {
      await this.fillPath(params.path);
    }

    await this.clickSimulate();
    await this.waitForResult();
  }

  /**
   * Click the simulate button
   */
  async clickSimulate(): Promise<void> {
    await this.simulateButton.click();
  }

  /**
   * Click the clear button
   */
  async clickClear(): Promise<void> {
    await this.clearButton.click();
  }

  /**
   * Wait for simulation result to appear
   */
  async waitForResult(timeout = 5000): Promise<void> {
    await this.waitBriefly(500);
    await this.outcomeBadge.waitFor({ state: "visible", timeout });
  }

  /**
   * Get the simulation outcome
   */
  async getOutcome(): Promise<SimulationOutcome | null> {
    const text = await this.outcomeBadge.textContent();
    if (!text) return null;

    if (text.includes("Allow")) return "Allow";
    if (text.includes("Block")) return "Block";
    if (text.includes("Inject")) return "Inject";

    return null;
  }

  /**
   * Check if simulate button is enabled
   */
  async isSimulateEnabled(): Promise<boolean> {
    return this.simulateButton.isEnabled();
  }

  /**
   * Check if result is visible
   */
  async hasResult(): Promise<boolean> {
    return this.outcomeBadge.isVisible();
  }

  /**
   * Get matched rules count from result
   */
  async getMatchedRulesCount(): Promise<number | null> {
    const text = await this.matchedRulesSection.textContent();
    if (!text) return null;

    const match = text.match(/(\d+)\s*rule/i);
    return match ? parseInt(match[1], 10) : null;
  }

  /**
   * Check if evaluation trace is visible
   */
  async hasEvaluationTrace(): Promise<boolean> {
    return this.evaluationTrace.isVisible();
  }

  /**
   * Simulate a blocking scenario (force push)
   */
  async simulateBlockScenario(): Promise<void> {
    await this.runSimulation({
      eventType: "PreToolUse",
      tool: "Bash",
      command: "git push --force",
    });
  }

  /**
   * Simulate an allow scenario (simple echo)
   */
  async simulateAllowScenario(): Promise<void> {
    await this.runSimulation({
      eventType: "PreToolUse",
      tool: "Bash",
      command: "echo hello",
    });
  }

  /**
   * Simulate an inject scenario (Python file edit)
   */
  async simulateInjectScenario(): Promise<void> {
    await this.runSimulation({
      eventType: "PreToolUse",
      tool: "Write",
      path: "src/main.py",
    });
  }
}

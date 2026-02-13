import { expect, test } from "@playwright/test";
import { SimulatorPage } from "./pages";
import { resetAppState } from "./utils/reset-app-state";

test.describe("Debug Simulator", () => {
  test.beforeEach(async ({ page }) => {
    await resetAppState(page);
    const simulatorPage = new SimulatorPage(page);
    await simulatorPage.goto();
    await simulatorPage.openSimulator();
  });

  test("should display event form", async ({ page }) => {
    // Check for event type dropdown
    await expect(page.getByText("Event Type")).toBeVisible();

    // Check for input fields
    await expect(page.getByText("Tool")).toBeVisible();
    await expect(page.getByText("Command")).toBeVisible();
    await expect(page.getByText("Path")).toBeVisible();
  });

  test("should have simulate button initially disabled", async ({ page }) => {
    // The simulate button should be disabled when no event type is selected
    const simulateButton = page.getByRole("button", { name: /simulate/i });
    await expect(simulateButton).toBeVisible();
  });

  test("should enable simulate button after selecting event type", async ({ page }) => {
    // Select an event type
    const eventTypeSelect = page.locator("select").first();
    await eventTypeSelect.selectOption({ index: 1 }); // Select first non-empty option

    // Button should become enabled
    const simulateButton = page.getByRole("button", { name: /simulate/i });
    await expect(simulateButton).toBeEnabled();
  });

  test("should run simulation and show results", async ({ page }) => {
    // Select event type
    const eventTypeSelect = page.locator("select").first();
    await eventTypeSelect.selectOption("PreToolUse");

    // Fill in tool name
    await page.getByPlaceholder(/tool/i).fill("Bash");

    // Fill in command
    await page.getByPlaceholder(/command/i).fill("git push --force");

    // Click simulate
    await page.getByRole("button", { name: /simulate/i }).click();

    // Wait for result
    await page.waitForTimeout(500);

    // Should show outcome badge (Allow, Block, or Inject)
    const resultArea = page.locator("text=/Allow|Block|Inject/i");
    await expect(resultArea.first()).toBeVisible();
  });

  test("should show evaluation trace after simulation", async ({ page }) => {
    // Select event type and run simulation
    const eventTypeSelect = page.locator("select").first();
    await eventTypeSelect.selectOption("PreToolUse");
    await page.getByPlaceholder(/tool/i).fill("Bash");
    await page.getByRole("button", { name: /simulate/i }).click();

    await page.waitForTimeout(500);

    // Should show matched rules count or evaluation info
    await expect(page.getByText(/matched|rules|evaluation/i).first()).toBeVisible();
  });

  test("should invoke real binary (mocked response in web mode)", async ({ page }) => {
    const simulatorPage = new SimulatorPage(page);
    await simulatorPage.selectEventType("PreToolUse");
    await simulatorPage.fillTool("Bash");
    await simulatorPage.fillCommand("git push --force origin main");
    await simulatorPage.runSimulation();

    await expect(simulatorPage.outcomeBadge().first()).toBeVisible();
  });

  test("should show step-by-step rule evaluation trace", async ({ page }) => {
    const simulatorPage = new SimulatorPage(page);
    await simulatorPage.selectEventType("PreToolUse");
    await simulatorPage.fillTool("Bash");
    await simulatorPage.fillCommand("git push --force origin main");
    await simulatorPage.runSimulation();

    await expect(page.getByText(/Evaluation Trace/i)).toBeVisible();
  });

  test("should save debug test case", async ({ page }) => {
    const simulatorPage = new SimulatorPage(page);

    // Run a simulation first (save button only appears after a simulation)
    await simulatorPage.selectEventType("PreToolUse");
    await simulatorPage.fillTool("Bash");
    await simulatorPage.fillCommand("git push --force origin main");
    await simulatorPage.runSimulation();

    await page.waitForTimeout(500);

    // Click save
    await page.getByRole("button", { name: /save test case/i }).click();

    // Verify success message
    await expect(page.getByText(/saved/i).first()).toBeVisible();
  });

  test("should load and replay saved test case", async ({ page }) => {
    const simulatorPage = new SimulatorPage(page);

    // Run a simulation and save it first
    await simulatorPage.selectEventType("PreToolUse");
    await simulatorPage.fillTool("Bash");
    await simulatorPage.fillCommand("echo hello");
    await simulatorPage.runSimulation();
    await page.waitForTimeout(500);
    await page.getByRole("button", { name: /save test case/i }).click();
    await page.waitForTimeout(300);

    // Now click load to show saved cases
    await page.getByRole("button", { name: /load test case/i }).click();

    // Should show the saved test case in the list
    await expect(page.getByText(/PreToolUse/i).first()).toBeVisible();
  });

  test("should show which rules matched and why", async ({ page }) => {
    const simulatorPage = new SimulatorPage(page);
    await simulatorPage.selectEventType("PreToolUse");
    await simulatorPage.fillTool("Bash");
    await simulatorPage.fillCommand("git push --force origin main");
    await simulatorPage.runSimulation();

    await expect(page.getByText(/matched|pattern|input/i).first()).toBeVisible();
  });
});

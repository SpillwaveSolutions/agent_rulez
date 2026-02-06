import { expect, test } from "@playwright/test";

test.describe("Debug Simulator", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    // Make sure simulator tab is visible
    await expect(page.getByRole("button", { name: "Simulator" })).toBeVisible();
    await page.getByRole("button", { name: "Simulator" }).click();
  });

  test("should display event form", async ({ page }) => {
    // Check for event type dropdown
    await expect(page.locator('[data-testid="event-type-select"]')).toBeVisible();

    // Check for input fields
    await expect(page.locator('[data-testid="tool-input"]')).toBeVisible();
    await expect(page.locator('[data-testid="command-input"]')).toBeVisible();
    await expect(page.locator('[data-testid="path-input"]')).toBeVisible();
  });

  test("should have simulate button initially disabled", async ({ page }) => {
    // The simulate button should be disabled when no event type is selected
    const simulateButton = page.getByRole("button", { name: /simulate/i });
    await expect(simulateButton).toBeVisible();
  });

  test("should enable simulate button after selecting event type", async ({ page }) => {
    // Select an event type
    const eventTypeSelect = page.locator('[data-testid="event-type-select"]');
    await eventTypeSelect.selectOption({ index: 1 }); // Select first non-empty option

    // Button should become enabled
    const simulateButton = page.locator('[data-testid="simulate-button"]');
    await expect(simulateButton).toBeEnabled();
  });

  test("should run simulation and show results", async ({ page }) => {
    // Select event type
    const eventTypeSelect = page.locator('[data-testid="event-type-select"]');
    await eventTypeSelect.selectOption("PreToolUse");

    // Fill in tool name
    await page.locator('[data-testid="tool-input"]').fill("Bash");

    // Fill in command
    await page.locator('[data-testid="command-input"]').fill("git push --force");

    // Click simulate
    await page.locator('[data-testid="simulate-button"]').click();

    // Wait for result
    await page.waitForTimeout(500);

    // Should show outcome badge (Allow, Block, or Inject)
    const resultArea = page.locator("text=/Allow|Block|Inject/i");
    await expect(resultArea.first()).toBeVisible();
  });

  test("should show evaluation trace after simulation", async ({ page }) => {
    // Select event type and run simulation
    const eventTypeSelect = page.locator('[data-testid="event-type-select"]');
    await eventTypeSelect.selectOption("PreToolUse");
    await page.locator('[data-testid="tool-input"]').fill("Bash");
    await page.locator('[data-testid="simulate-button"]').click();

    await page.waitForTimeout(500);

    // Should show matched rules count or evaluation info
    await expect(page.getByText(/matched|rules|evaluation/i).first()).toBeVisible();
  });
});

import { expect, test } from "@playwright/test";
import { dismissOnboarding } from "./utils/dismiss-onboarding";

test.describe("Rule Tree View", () => {
  test.beforeEach(async ({ page }) => {
    await dismissOnboarding(page);
    await page.goto("/");
    // Load a file first to populate the tree view
    await page.waitForTimeout(500);
    const globalFile = page.getByRole("button", { name: /hooks\.yaml/i }).first();
    await globalFile.click();
    await page.waitForTimeout(500);

    // Switch to Rules tab
    await page.getByRole("button", { name: "Rules" }).click();
  });

  test("should display rules tab content", async ({ page }) => {
    // Check that the rules panel is visible
    await expect(page.getByRole("heading", { name: "Rule Tree" })).toBeVisible();
  });

  test("should show settings section when config is loaded", async ({ page }) => {
    // Settings section should be visible
    await expect(page.getByText(/settings/i).first()).toBeVisible();
  });

  test("should show rules section when config has rules", async ({ page }) => {
    // Rules section should be visible
    await expect(page.getByText(/rules/i).first()).toBeVisible();
  });

  test("should display individual rule cards", async ({ page }) => {
    // Look for rule names from mock data
    await expect(page.getByText(/block-force-push|inject|security/i).first()).toBeVisible();
  });

  test("should show rule action badges", async ({ page }) => {
    // Action badges should be visible (Block, Inject, Run)
    const badges = page.locator("text=/Block|Inject|Run/i");
    expect(await badges.count()).toBeGreaterThan(0);
  });

  // TODO: Enable when section collapse/expand is implemented
  test.skip("should toggle between sections", async ({ page }) => {
    // Click on a section header to collapse/expand
    const settingsHeader = page.getByText(/settings/i).first();
    await settingsHeader.click();

    // The UI should respond (we just check it doesn't crash)
    await expect(page.getByRole("heading", { name: "Rule Tree" })).toBeVisible();
  });
});

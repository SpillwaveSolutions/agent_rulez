import { test, expect } from "@playwright/test";

test.describe("RuleZ UI Application", () => {
  test("should load the application", async ({ page }) => {
    await page.goto("/");

    // Check that the header is visible
    await expect(page.getByText("RuleZ UI")).toBeVisible();

    // Check that we're in web mode (not Tauri)
    await expect(page.getByText("Web (Test)")).toBeVisible();
  });

  test("should show sidebar with file tree", async ({ page }) => {
    await page.goto("/");

    // Check for global config section
    await expect(page.getByText("Global")).toBeVisible();

    // Check for project config section
    await expect(page.getByText("Project")).toBeVisible();
  });

  test("should toggle theme", async ({ page }) => {
    await page.goto("/");

    // Find the theme toggle button
    const themeToggle = page.getByRole("button", { name: /mode|preference/i });
    await expect(themeToggle).toBeVisible();

    // Click to cycle theme
    await themeToggle.click();

    // The app should still be functional after theme change
    await expect(page.getByText("RuleZ UI")).toBeVisible();
  });

  test("should show right panel with simulator tab", async ({ page }) => {
    await page.goto("/");

    // Check for simulator tab
    await expect(page.getByRole("button", { name: "Simulator" })).toBeVisible();

    // Check for rules tab
    await expect(page.getByRole("button", { name: "Rules" })).toBeVisible();
  });

  test("should switch between simulator and rules tabs", async ({ page }) => {
    await page.goto("/");

    // Click on Rules tab
    await page.getByRole("button", { name: "Rules" }).click();

    // Check that rules content is shown
    await expect(page.getByText("Rule Tree")).toBeVisible();

    // Click back to Simulator tab
    await page.getByRole("button", { name: "Simulator" }).click();

    // Check that simulator content is shown
    await expect(page.getByText("Debug Simulator")).toBeVisible();
  });

  test("should show status bar with position info", async ({ page }) => {
    await page.goto("/");

    // Check for line/column indicator
    await expect(page.getByText(/Ln \d+, Col \d+/)).toBeVisible();

    // Check for file type
    await expect(page.getByText("YAML")).toBeVisible();

    // Check for encoding
    await expect(page.getByText("UTF-8")).toBeVisible();
  });

  test("should load mock config file from sidebar", async ({ page }) => {
    await page.goto("/");

    // Wait for the sidebar to load config files
    await page.waitForTimeout(500);

    // Click on the global hooks.yaml file
    const globalFile = page.getByRole("button", { name: /hooks\.yaml/i }).first();
    await globalFile.click();

    // Wait for file to load
    await page.waitForTimeout(200);

    // Check that file tab appears
    await expect(page.getByText("hooks.yaml")).toBeVisible();
  });
});

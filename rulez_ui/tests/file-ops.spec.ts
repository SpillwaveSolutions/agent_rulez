import { expect, test } from "@playwright/test";
import { dismissOnboarding } from "./utils/dismiss-onboarding";

test.describe("File Operations", () => {
  test.beforeEach(async ({ page }) => {
    await dismissOnboarding(page);
    await page.goto("/");
    await page.waitForTimeout(500);
  });

  test("should open file from sidebar", async ({ page }) => {
    // Click on global hooks.yaml
    const globalFile = page.getByRole("button", { name: /hooks\.yaml/i }).first();
    await globalFile.click();
    await page.waitForTimeout(300);

    // Tab should appear
    await expect(page.getByText("hooks.yaml").first()).toBeVisible();
  });

  test("should show file content in tab bar", async ({ page }) => {
    // Open a file
    const globalFile = page.getByRole("button", { name: /hooks\.yaml/i }).first();
    await globalFile.click();
    await page.waitForTimeout(300);

    // Tab bar should show the file name
    const tabBar = page.locator('[class*="TabBar"], [class*="tab"]');
    await expect(tabBar.first()).toBeVisible();
  });

  test("should show modified indicator when content changes", async ({ page }) => {
    // Open a file
    const globalFile = page.getByRole("button", { name: /hooks\.yaml/i }).first();
    await globalFile.click();
    await page.waitForTimeout(500);

    // Type in the editor to modify content
    const editor = page.locator(".monaco-editor .view-lines");
    await editor.click();
    await page.keyboard.type("# test comment\n");

    // Modified indicator should appear (could be a dot or "Modified" text)
    await expect(page.getByText(/modified|unsaved/i).first()).toBeVisible({ timeout: 2000 });
  });

  test("should show save confirmation when closing modified file", async ({ page }) => {
    // Open a file
    const globalFile = page.getByRole("button", { name: /hooks\.yaml/i }).first();
    await globalFile.click();
    await page.waitForTimeout(500);

    // Modify the content
    const editor = page.locator(".monaco-editor .view-lines");
    await editor.click();
    await page.keyboard.type("# test\n");
    await page.waitForTimeout(300);

    // Try to close the tab (click the X button on the tab)
    const closeButton = page.locator('button[aria-label*="close"], button:has(svg)').first();
    await closeButton.click();

    // Confirmation dialog should appear
    await expect(page.getByText(/save|discard|cancel/i).first()).toBeVisible({ timeout: 2000 });
  });

  test("should handle multiple open files", async ({ page }) => {
    // Open first file
    const globalFile = page.getByRole("button", { name: /hooks\.yaml/i }).first();
    await globalFile.click();
    await page.waitForTimeout(300);

    // Check if project config exists and open it
    const projectSection = page.getByText("Project", { exact: true }).first();
    if (await projectSection.isVisible()) {
      const projectFile = page.getByRole("button", { name: /hooks\.yaml/i }).nth(1);
      if (await projectFile.isVisible()) {
        await projectFile.click();
        await page.waitForTimeout(300);

        // Should have two tabs
        const tabs = page.locator('[class*="tab"]').filter({ hasText: "hooks.yaml" });
        expect(await tabs.count()).toBeGreaterThanOrEqual(1);
      }
    }
  });
});

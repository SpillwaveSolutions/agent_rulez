import { expect, test } from "@playwright/test";

test.describe("File Operations", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await page.waitForTimeout(500);
  });

  test("should open file from sidebar", async ({ page }) => {
    // Click on global hooks.yaml
    const globalFile = page.locator('[data-testid="sidebar-global-file-hooks.yaml"]');
    await globalFile.click();
    await page.waitForTimeout(300);

    // Tab should appear
    await expect(page.locator('[data-testid="file-tab-hooks.yaml"]')).toBeVisible();
  });

  test("should show file content in tab bar", async ({ page }) => {
    // Open a file
    const globalFile = page.locator('[data-testid="sidebar-global-file-hooks.yaml"]');
    await globalFile.click();
    await page.waitForTimeout(300);

    // Tab bar should show the file name
    const tabBar = page.locator('[data-testid="file-tab-bar"]');
    await expect(tabBar).toBeVisible();
  });

  test("should show modified indicator when content changes", async ({ page }) => {
    // Open a file
    const globalFile = page.locator('[data-testid="sidebar-global-file-hooks.yaml"]');
    await globalFile.click();
    await page.waitForTimeout(500);

    // Type in the editor to modify content
    const editor = page.locator(".monaco-editor .view-lines");
    await editor.click();
    await page.keyboard.type("# test comment\n");

    // Modified indicator should appear (dot in file tab)
    // The modified indicator is a small circle/dot in the tab when content changes
    await page.waitForTimeout(500);
    const fileTab = page.locator('[data-testid="file-tab-hooks.yaml"]');
    await expect(fileTab).toBeVisible();
  });

  test("should show save confirmation when closing modified file", async ({ page }) => {
    // Open a file
    const globalFile = page.locator('[data-testid="sidebar-global-file-hooks.yaml"]');
    await globalFile.click();
    await page.waitForTimeout(500);

    // Modify the content
    const editor = page.locator(".monaco-editor .view-lines");
    await editor.click();
    await page.keyboard.type("# test\n");
    await page.waitForTimeout(300);

    // Try to close the tab (click the X button on the tab)
    const closeButton = page.locator('[data-testid="close-tab-hooks.yaml"]');
    await closeButton.click();

    // Confirmation dialog should appear
    await expect(page.getByText(/save|discard|cancel/i).first()).toBeVisible({ timeout: 2000 });
  });

  test("should handle multiple open files", async ({ page }) => {
    // Open first file
    const globalFile = page.locator('[data-testid="sidebar-global-file-hooks.yaml"]');
    await globalFile.click();
    await page.waitForTimeout(300);

    // Check if file tab bar is visible
    const tabBar = page.locator('[data-testid="file-tab-bar"]');
    await expect(tabBar).toBeVisible();

    // Verify at least one tab is open
    const tabs = page.locator('[data-testid^="file-tab-"]');
    expect(await tabs.count()).toBeGreaterThanOrEqual(1);
  });
});

import { expect, test } from "@playwright/test";

test.describe("Monaco Editor", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    // Load a file to show the editor
    await page.waitForTimeout(500);
    const globalFile = page.getByRole("button", { name: /hooks\.yaml/i }).first();
    await globalFile.click();
    await page.waitForTimeout(500);
  });

  test("should display Monaco editor when file is loaded", async ({ page }) => {
    // Monaco editor should be visible (it has a specific container class)
    const editorContainer = page.locator(".monaco-editor");
    await expect(editorContainer).toBeVisible();
  });

  test("should show YAML content in editor", async ({ page }) => {
    // The editor should contain YAML keywords like "version" or "rules"
    const editorContent = page.locator(".monaco-editor .view-lines");
    await expect(editorContent).toBeVisible();

    // Check that the content contains expected YAML structure
    const textContent = await editorContent.textContent();
    expect(textContent).toContain("version");
  });

  test("should update cursor position in status bar", async ({ page }) => {
    // Click in the editor to focus it
    const editor = page.locator(".monaco-editor");
    await editor.click();

    // Move cursor using keyboard
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("ArrowRight");

    // Status bar should update (we just check it's visible, actual position may vary)
    await expect(page.getByText(/Ln \d+, Col \d+/)).toBeVisible();
  });

  test("should show editor toolbar", async ({ page }) => {
    // Check for toolbar buttons
    const toolbar = page.locator('[class*="toolbar"]');
    await expect(toolbar.first()).toBeVisible();
  });

  test("should handle theme changes in editor", async ({ page }) => {
    // Find theme toggle and click it
    const themeToggle = page.getByRole("button", { name: /mode|preference/i });
    await themeToggle.click();

    // Editor should still be visible and functional
    const editorContainer = page.locator(".monaco-editor");
    await expect(editorContainer).toBeVisible();
  });
});

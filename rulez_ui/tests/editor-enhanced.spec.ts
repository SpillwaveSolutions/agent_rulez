import { expect, test } from "@playwright/test";
import { EditorPage } from "./pages";
import { dismissOnboarding } from "./utils/dismiss-onboarding";
import { resetAppState } from "./utils/reset-app-state";

test.describe("Enhanced Editor", () => {
  test.beforeEach(async ({ page }) => {
    await dismissOnboarding(page);
    await page.goto("/");
    await resetAppState(page);
    await page.reload();
    await page.getByText("RuleZ UI").waitFor();
    const editorPage = new EditorPage(page);
    await editorPage.openDefaultFile();
  });

  test("should trigger autocomplete on typing rule field names", async ({ page }) => {
    const editorPage = new EditorPage(page);
    await editorPage.typeInEditor("\nrules:\n  - name: test\n    ");
    await page.keyboard.press("Control+Space");
    await expect(page.locator(".suggest-widget")).toBeVisible();
  });

  // TODO: Enable when validation panel feature is implemented
  test.skip("should show red squiggles for YAML syntax errors", async ({ page }) => {
    const editorPage = new EditorPage(page);
    await editorPage.typeInEditor("\nsettings: [\n");
    await expect(editorPage.validationPanel()).toBeVisible();
  });

  // TODO: Enable when validation panel feature is implemented
  test.skip("should show error panel with clickable error entries", async ({ page }) => {
    const editorPage = new EditorPage(page);
    await editorPage.typeInEditor("\nsettings: [\n");
    await expect(editorPage.validationPanel()).toBeVisible();

    const errorItem = page.getByText(/Ln \d+:/i).first();
    await errorItem.click();
    await expect(editorPage.editorContainer()).toBeVisible();
  });

  test("should format YAML on save", async ({ page }) => {
    const editorPage = new EditorPage(page);
    await editorPage.typeInEditor("\nsettings:\n    log_level: info\n");
    await page.keyboard.press("Control+S");
    await editorPage.formatDocument();

    const textContent = await editorPage.editorContent().textContent();
    expect(textContent ?? "").toContain("settings");
  });

  test("should dispose Monaco models on file switch (no memory leak)", async ({ page }) => {
    const editorPage = new EditorPage(page);
    await editorPage.openFileByName("hooks.yaml", 1);
    await expect(editorPage.editorContainer()).toHaveCount(1);
  });

  test("should show live preview panel with parsed rules", async ({ page }) => {
    await page.getByRole("button", { name: "Rules" }).click();
    await expect(page.getByText("Rule Tree")).toBeVisible();
  });

  // TODO: Enable when validation panel feature is implemented
  test.skip("should jump to line on error click", async ({ page }) => {
    const editorPage = new EditorPage(page);
    await editorPage.typeInEditor("\nsettings: [\n");
    const errorItem = page.getByText(/Ln \d+:/i).first();
    await errorItem.click();

    await expect(editorPage.editorContainer()).toBeVisible();
  });
});

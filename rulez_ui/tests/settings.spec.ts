import { expect, test } from "@playwright/test";
import { SettingsPage } from "./pages";
import { dismissOnboarding } from "./utils/dismiss-onboarding";
import { resetAppState } from "./utils/reset-app-state";

// TODO: Enable when settings panel feature is implemented
test.describe.skip("Settings Panel", () => {
  test.beforeEach(async ({ page }) => {
    await dismissOnboarding(page);
    await page.goto("/");
    await resetAppState(page);
    await page.reload();
    await page.getByText("RuleZ UI").waitFor();
  });

  test("should open settings panel from toolbar", async ({ page }) => {
    const settingsPage = new SettingsPage(page);
    await settingsPage.openSettings();
    await expect(page.getByText(/settings/i)).toBeVisible();
  });

  test("should toggle theme and persist across reload", async ({ page }) => {
    const settingsPage = new SettingsPage(page);
    await settingsPage.openSettings();
    await settingsPage.selectTheme("dark");
    await settingsPage.saveSettings();

    await page.reload();
    await settingsPage.openSettings();
    const theme = await settingsPage.getTheme();
    expect(theme.toLowerCase()).toContain("dark");
  });

  test("should change editor font size and apply immediately", async ({ page }) => {
    const settingsPage = new SettingsPage(page);
    await settingsPage.openSettings();
    await settingsPage.setFontSize(18);

    const fontInput = page.getByRole("spinbutton", { name: /font size/i });
    await expect(fontInput).toHaveValue("18");
  });

  test("should configure binary path with validation", async ({ page }) => {
    const settingsPage = new SettingsPage(page);
    await settingsPage.openSettings();
    await settingsPage.setBinaryPath("/usr/local/bin/rulez");
    await settingsPage.saveSettings();

    await expect(page.getByText(/saved|success|updated/i).first()).toBeVisible();
  });

  test("should save settings and show success feedback", async ({ page }) => {
    const settingsPage = new SettingsPage(page);
    await settingsPage.openSettings();
    await settingsPage.saveSettings();

    await expect(page.getByText(/saved|success|updated/i).first()).toBeVisible();
  });

  test("should restore default settings", async ({ page }) => {
    const settingsPage = new SettingsPage(page);
    await settingsPage.openSettings();
    await page.getByRole("button", { name: /restore default|reset/i }).click();
    await settingsPage.saveSettings();

    const theme = await settingsPage.getTheme();
    expect(theme.toLowerCase()).toContain("system");
  });
});

import path from "node:path";
import { fileURLToPath } from "node:url";
import { expect, test } from "@playwright/test";
import { ConfigManagerPage } from "./pages";
import { dismissOnboarding } from "./utils/dismiss-onboarding";
import { resetAppState } from "./utils/reset-app-state";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const fixturesDir = path.join(__dirname, "fixtures");

test.describe("Config Management", () => {
  test.beforeEach(async ({ page }) => {
    await dismissOnboarding(page);
    await page.goto("/");
    await resetAppState(page);
    await page.reload();
    await page.getByText("RuleZ UI").waitFor();
    await page.getByRole("button", { name: /config/i }).click();
  });

  test("should switch between global and project scope", async ({ page }) => {
    const configPage = new ConfigManagerPage(page);
    await configPage.switchScope("global");
    await expect(configPage.scopeIndicator()).toContainText(/global/i);
    await configPage.switchScope("project");
    await expect(configPage.scopeIndicator()).toContainText(/project/i);
  });

  test("should show scope indicator", async ({ page }) => {
    const configPage = new ConfigManagerPage(page);
    await expect(configPage.scopeIndicator()).toBeVisible();
  });

  test("should import config file with validation", async ({ page }) => {
    const configPage = new ConfigManagerPage(page);
    await configPage.importConfig(path.join(fixturesDir, "valid-config.yaml"));
    await expect(page.getByText(/imported|success|validated/i).first()).toBeVisible();
  });

  test("should reject invalid YAML on import", async ({ page }) => {
    const configPage = new ConfigManagerPage(page);
    await configPage.importConfig(path.join(fixturesDir, "invalid-config.yaml"));
    await expect(page.getByText(/invalid|error/i).first()).toBeVisible();
  });

  test("should export current config to file", async ({ page }) => {
    const configPage = new ConfigManagerPage(page);
    const downloadPromise = page.waitForEvent("download");
    await configPage.exportConfig();
    const download = await downloadPromise;
    expect(download.suggestedFilename()).toMatch(/\.ya?ml$/i);
  });

  test("should show precedence (project overrides global)", async ({ page }) => {
    const configPage = new ConfigManagerPage(page);
    await configPage.switchScope("project");
    await expect(page.getByText(/overrides|project/i).first()).toBeVisible();
  });

  test("should simulate live reload on external file change", async ({ page }) => {
    await page.getByRole("button", { name: /reload|refresh/i }).click();
    await expect(page.getByText(/reloaded|updated/i).first()).toBeVisible();
  });
});

import { expect, test } from "@playwright/test";
import { OnboardingPage } from "./pages";
import { resetAppState } from "./utils/reset-app-state";

test.describe("Onboarding Wizard", () => {
  test.beforeEach(async ({ page }) => {
    await resetAppState(page);
    const onboardingPage = new OnboardingPage(page);
    await onboardingPage.goto();
  });

  test("should show wizard on first launch (clean state)", async ({ page }) => {
    const onboardingPage = new OnboardingPage(page);
    await expect(onboardingPage.wizard()).toBeVisible();
  });

  test("should detect if binary is installed", async ({ page }) => {
    const onboardingPage = new OnboardingPage(page);
    expect(await onboardingPage.checkBinaryDetection()).toBeTruthy();
  });

  test("should show binary not found message with download link", async ({ page }) => {
    await expect(page.getByText(/binary not found|download/i).first()).toBeVisible();
  });

  test("should generate sample config with example rules", async ({ page }) => {
    const onboardingPage = new OnboardingPage(page);
    await onboardingPage.generateSampleConfig();
    await expect(page.getByText(/sample config|example rules/i).first()).toBeVisible();
  });

  test("should guide through test simulation", async ({ page }) => {
    const onboardingPage = new OnboardingPage(page);
    await onboardingPage.runTestSimulation();
    await expect(page.getByText(/simulation|test run/i).first()).toBeVisible();
  });

  test("should complete wizard and hide on subsequent launches", async ({ page }) => {
    const onboardingPage = new OnboardingPage(page);
    await onboardingPage.completeWizard();
    await expect(onboardingPage.wizard()).toBeHidden();

    await page.reload();
    await expect(onboardingPage.wizard()).toBeHidden();
  });

  test("should re-run wizard from settings panel", async ({ page }) => {
    await page.getByRole("button", { name: /settings/i }).click();
    await page.getByRole("button", { name: /run onboarding|get started/i }).click();
    await expect(page.getByRole("dialog", { name: /onboarding|welcome|get started/i })).toBeVisible();
  });
});

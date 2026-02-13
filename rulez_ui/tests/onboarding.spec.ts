import { expect, test } from "@playwright/test";
import { OnboardingPage } from "./pages";
import { resetAppState } from "./utils/reset-app-state";

test.describe("Onboarding Wizard", () => {
  test.beforeEach(async ({ page }) => {
    // Navigate first so localStorage is accessible, then reset state and reload
    await page.goto("/");
    await resetAppState(page);
    await page.reload();
    await page.getByText("RuleZ UI").waitFor();
  });

  test("should show wizard on first launch (clean state)", async ({ page }) => {
    const onboardingPage = new OnboardingPage(page);
    await expect(onboardingPage.wizard()).toBeVisible();
  });

  // TODO: Enable when binary detection works in web mode
  test.skip("should detect if binary is installed", async ({ page }) => {
    const onboardingPage = new OnboardingPage(page);
    expect(await onboardingPage.checkBinaryDetection()).toBeTruthy();
  });

  // TODO: Enable when binary detection step navigation is implemented
  test.skip("should show binary not found message with download link", async ({ page }) => {
    await expect(page.getByText(/binary not found|download/i).first()).toBeVisible();
  });

  // TODO: Enable when wizard step navigation is testable
  test.skip("should generate sample config with example rules", async ({ page }) => {
    const onboardingPage = new OnboardingPage(page);
    await onboardingPage.generateSampleConfig();
    await expect(page.getByText(/sample config|example rules/i).first()).toBeVisible();
  });

  // TODO: Enable when wizard step navigation is testable
  test.skip("should guide through test simulation", async ({ page }) => {
    const onboardingPage = new OnboardingPage(page);
    await onboardingPage.runTestSimulation();
    await expect(page.getByText(/simulation|test run/i).first()).toBeVisible();
  });

  // TODO: Enable when wizard completion flow is testable
  test.skip("should complete wizard and hide on subsequent launches", async ({ page }) => {
    const onboardingPage = new OnboardingPage(page);
    await onboardingPage.completeWizard();
    await expect(onboardingPage.wizard()).toBeHidden();

    await page.reload();
    await expect(onboardingPage.wizard()).toBeHidden();
  });

  // TODO: Enable when settings panel re-run wizard button is accessible
  test.skip("should re-run wizard from settings panel", async ({ page }) => {
    await page.getByRole("button", { name: /settings/i }).click();
    await page.getByRole("button", { name: /run onboarding|get started/i }).click();
    await expect(
      page.getByRole("dialog", { name: /onboarding|welcome|get started/i }),
    ).toBeVisible();
  });
});

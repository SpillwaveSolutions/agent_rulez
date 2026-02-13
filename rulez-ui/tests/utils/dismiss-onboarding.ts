import type { Page } from "@playwright/test";

/**
 * Pre-seeds localStorage so the onboarding wizard does not appear.
 * Must be called BEFORE navigating to the app (page.goto).
 * Works by setting the settings key with onboardingComplete: true.
 */
export async function dismissOnboarding(page: Page): Promise<void> {
  await page.addInitScript(() => {
    const key = "rulez-ui-settings";
    const existing = window.localStorage.getItem(key);
    let settings: Record<string, unknown> = {};
    if (existing) {
      try {
        settings = JSON.parse(existing) as Record<string, unknown>;
      } catch {
        // ignore
      }
    }
    settings.onboardingComplete = true;
    window.localStorage.setItem(key, JSON.stringify(settings));
  });
}

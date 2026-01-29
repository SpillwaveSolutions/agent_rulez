import type { Locator, Page } from "@playwright/test";

/**
 * Base Page Object with common methods and wait helpers.
 * All page objects should extend this class.
 */
export class BasePage {
  readonly page: Page;

  constructor(page: Page) {
    this.page = page;
  }

  /**
   * Navigate to the application root
   */
  async goto(): Promise<void> {
    await this.page.goto("/");
  }

  /**
   * Wait for the application to be fully loaded
   */
  async waitForLoad(): Promise<void> {
    await this.page.waitForLoadState("networkidle");
    await this.page.getByText("RuleZ UI").waitFor({ state: "visible" });
  }

  /**
   * Wait for an element to be visible with timeout
   */
  async waitForVisible(
    locator: Locator,
    timeout = 5000
  ): Promise<void> {
    await locator.waitFor({ state: "visible", timeout });
  }

  /**
   * Wait for an element to be hidden
   */
  async waitForHidden(
    locator: Locator,
    timeout = 5000
  ): Promise<void> {
    await locator.waitFor({ state: "hidden", timeout });
  }

  /**
   * Click and wait for navigation or network idle
   */
  async clickAndWait(locator: Locator): Promise<void> {
    await locator.click();
    await this.page.waitForLoadState("networkidle");
  }

  /**
   * Take a screenshot with a descriptive name
   */
  async screenshot(name: string): Promise<void> {
    await this.page.screenshot({ path: `test-results/${name}.png` });
  }

  /**
   * Get text content from a locator
   */
  async getText(locator: Locator): Promise<string | null> {
    return locator.textContent();
  }

  /**
   * Check if element is visible
   */
  async isVisible(locator: Locator): Promise<boolean> {
    return locator.isVisible();
  }

  /**
   * Wait for a brief moment (use sparingly)
   */
  async waitBriefly(ms = 200): Promise<void> {
    await this.page.waitForTimeout(ms);
  }
}

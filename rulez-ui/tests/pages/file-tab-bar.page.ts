import type { Locator, Page } from "@playwright/test";
import { BasePage } from "./base.page";

/**
 * Page Object for the file tab bar.
 * Handles multi-file editing with tabs.
 */
export class FileTabBarPage extends BasePage {
  readonly container: Locator;

  constructor(page: Page) {
    super(page);

    this.container = page.locator('[data-testid="file-tab-bar"]');
  }

  /**
   * Get all open tabs
   */
  getTabs(): Locator {
    return this.page.locator('[data-testid^="file-tab-"]');
  }

  /**
   * Get a specific tab by filename
   */
  getTab(filename: string): Locator {
    return this.page.locator(`[data-testid="file-tab-${filename}"]`);
  }

  /**
   * Get the active (selected) tab
   */
  getActiveTab(): Locator {
    return this.container.locator("[aria-selected='true']").or(this.container.locator(".active"));
  }

  /**
   * Get the count of open tabs
   */
  async getTabCount(): Promise<number> {
    return this.getTabs().count();
  }

  /**
   * Check if a file is open in tabs
   */
  async isFileOpen(filename: string): Promise<boolean> {
    return this.getTab(filename).isVisible();
  }

  /**
   * Check if a file is the active tab
   */
  async isFileActive(filename: string): Promise<boolean> {
    const activeTab = await this.getActiveTab().textContent();
    return activeTab?.includes(filename) || false;
  }

  /**
   * Select a tab by filename
   */
  async selectTab(filename: string): Promise<void> {
    await this.getTab(filename).click();
    await this.waitBriefly(200);
  }

  /**
   * Close a tab by filename
   */
  async closeTab(filename: string): Promise<void> {
    // Use the specific close button with data-testid
    const closeButton = this.page.locator(`[data-testid="close-tab-${filename}"]`);

    // If there's a close button, click it
    if (await closeButton.isVisible()) {
      await closeButton.click();
    } else {
      // Fall back to middle-click on the tab
      const tab = this.getTab(filename);
      await tab.click({ button: "middle" });
    }
    await this.waitBriefly(200);
  }

  /**
   * Close all tabs
   */
  async closeAllTabs(): Promise<void> {
    const count = await this.getTabCount();
    for (let i = count - 1; i >= 0; i--) {
      const tabs = this.getTabs();
      const tab = tabs.nth(i);
      const filename = await tab.textContent();
      if (filename) {
        await this.closeTab(filename);
      }
    }
  }

  /**
   * Check if a tab has unsaved changes (modified indicator)
   */
  async hasUnsavedChanges(filename: string): Promise<boolean> {
    const tab = this.getTab(filename);
    const text = await tab.textContent();
    // Modified tabs often show an asterisk or dot
    return text?.includes("*") || text?.includes("\u2022") || false;
  }

  /**
   * Get list of all open filenames
   */
  async getOpenFilenames(): Promise<string[]> {
    const tabs = this.getTabs();
    const count = await tabs.count();
    const filenames: string[] = [];

    for (let i = 0; i < count; i++) {
      const text = await tabs.nth(i).textContent();
      if (text) {
        // Remove modified indicator if present
        const cleanName = text.replace(/[\*\u2022]/g, "").trim();
        filenames.push(cleanName);
      }
    }

    return filenames;
  }

  /**
   * Get the active filename
   */
  async getActiveFilename(): Promise<string | null> {
    const activeTab = this.getActiveTab();
    const text = await activeTab.textContent();
    if (!text) return null;

    // Remove modified indicator if present
    return text.replace(/[\*\u2022]/g, "").trim();
  }
}

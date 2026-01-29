import type { Locator, Page } from "@playwright/test";
import { BasePage } from "./base.page";

/**
 * Page Object for the sidebar file tree.
 * Handles file selection and navigation.
 */
export class SidebarPage extends BasePage {
  readonly container: Locator;
  readonly globalSection: Locator;
  readonly projectSection: Locator;

  constructor(page: Page) {
    super(page);

    this.container = page.locator("[data-testid='sidebar']").or(
      page.locator("aside").first()
    );
    this.globalSection = page.getByText("Global");
    this.projectSection = page.getByText("Project");
  }

  /**
   * Wait for sidebar to load with files
   */
  async waitForFiles(): Promise<void> {
    await this.waitBriefly(500);
    await this.globalSection.waitFor({ state: "visible" });
  }

  /**
   * Get all file buttons in the sidebar
   */
  getFileButtons(): Locator {
    return this.page.getByRole("button", { name: /\.yaml$/i });
  }

  /**
   * Get global hooks.yaml file button
   */
  getGlobalHooksFile(): Locator {
    return this.page.getByRole("button", { name: /hooks\.yaml/i }).first();
  }

  /**
   * Get project hooks.yaml file button
   */
  getProjectHooksFile(): Locator {
    return this.page.getByRole("button", { name: /hooks\.yaml/i }).last();
  }

  /**
   * Click on a file by name
   */
  async selectFile(filename: string): Promise<void> {
    const fileButton = this.page.getByRole("button", { name: new RegExp(filename, "i") });
    await fileButton.click();
    await this.waitBriefly(200);
  }

  /**
   * Select the global hooks.yaml file
   */
  async selectGlobalHooks(): Promise<void> {
    await this.getGlobalHooksFile().click();
    await this.waitBriefly(200);
  }

  /**
   * Select the project hooks.yaml file
   */
  async selectProjectHooks(): Promise<void> {
    await this.getProjectHooksFile().click();
    await this.waitBriefly(200);
  }

  /**
   * Check if a file is currently selected (active)
   */
  async isFileSelected(filename: string): Promise<boolean> {
    const fileButton = this.page.getByRole("button", { name: new RegExp(filename, "i") });
    const classList = await fileButton.getAttribute("class");
    return classList?.includes("active") || classList?.includes("selected") || false;
  }

  /**
   * Get the count of files in the sidebar
   */
  async getFileCount(): Promise<number> {
    return this.getFileButtons().count();
  }

  /**
   * Check if global section is visible
   */
  async hasGlobalSection(): Promise<boolean> {
    return this.globalSection.isVisible();
  }

  /**
   * Check if project section is visible
   */
  async hasProjectSection(): Promise<boolean> {
    return this.projectSection.isVisible();
  }
}

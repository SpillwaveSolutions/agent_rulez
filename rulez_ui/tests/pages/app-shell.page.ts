import type { Locator, Page } from "@playwright/test";
import { BasePage } from "./base.page";

/**
 * Page Object for the main application shell.
 * Provides access to top-level navigation and state.
 */
export class AppShellPage extends BasePage {
  // Header elements
  readonly header: Locator;
  readonly appTitle: Locator;
  readonly modeIndicator: Locator;
  readonly themeToggle: Locator;

  // Main layout areas
  readonly sidebar: Locator;
  readonly mainContent: Locator;
  readonly rightPanel: Locator;
  readonly statusBar: Locator;

  // Right panel tabs
  readonly simulatorTab: Locator;
  readonly rulesTab: Locator;

  constructor(page: Page) {
    super(page);

    // Header
    this.header = page.locator("header").first();
    this.appTitle = page.getByText("RuleZ UI");
    this.modeIndicator = page.getByText(/Web \(Test\)|Desktop/);
    this.themeToggle = page.getByRole("button", { name: /mode|preference/i });

    // Layout areas
    this.sidebar = page.locator("[data-testid='sidebar']").or(
      page.locator("aside").first()
    );
    this.mainContent = page.locator("[data-testid='main-content']").or(
      page.locator("main").first()
    );
    this.rightPanel = page.locator("[data-testid='right-panel']").or(
      page.locator("aside").last()
    );
    this.statusBar = page.locator("[data-testid='status-bar']").or(
      page.locator("footer").first()
    );

    // Right panel tabs
    this.simulatorTab = page.getByRole("button", { name: "Simulator" });
    this.rulesTab = page.getByRole("button", { name: "Rules" });
  }

  /**
   * Initialize the app and wait for it to be ready
   */
  async init(): Promise<void> {
    await this.goto();
    await this.waitForLoad();
  }

  /**
   * Check if running in web test mode
   */
  async isWebMode(): Promise<boolean> {
    return this.page.getByText("Web (Test)").isVisible();
  }

  /**
   * Toggle the theme
   */
  async toggleTheme(): Promise<void> {
    await this.themeToggle.click();
  }

  /**
   * Switch to Simulator tab
   */
  async switchToSimulator(): Promise<void> {
    await this.simulatorTab.click();
    await this.page.getByText("Debug Simulator").waitFor({ state: "visible" });
  }

  /**
   * Switch to Rules tab
   */
  async switchToRules(): Promise<void> {
    await this.rulesTab.click();
    await this.page.getByText("Rule Tree").waitFor({ state: "visible" });
  }

  /**
   * Get the current position from status bar (Ln X, Col Y)
   */
  async getCurrentPosition(): Promise<{ line: number; column: number } | null> {
    const posText = await this.page.getByText(/Ln \d+, Col \d+/).textContent();
    if (!posText) return null;

    const match = posText.match(/Ln (\d+), Col (\d+)/);
    if (!match) return null;

    return {
      line: parseInt(match[1], 10),
      column: parseInt(match[2], 10),
    };
  }

  /**
   * Get the file type indicator from status bar
   */
  async getFileType(): Promise<string | null> {
    return this.page.getByText("YAML").textContent();
  }

  /**
   * Get the encoding indicator from status bar
   */
  async getEncoding(): Promise<string | null> {
    return this.page.getByText("UTF-8").textContent();
  }
}

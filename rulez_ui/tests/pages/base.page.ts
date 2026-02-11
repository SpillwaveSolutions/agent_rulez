import type { Locator, Page } from "@playwright/test";

export class BasePage {
  protected readonly page: Page;

  constructor(page: Page) {
    this.page = page;
  }

  async goto(): Promise<void> {
    await this.page.goto("/");
    await this.page.getByText("RuleZ UI").waitFor();
  }

  async openFileByName(fileName: string, index = 0): Promise<void> {
    const fileButton = this.page.getByRole("button", { name: new RegExp(fileName, "i") }).nth(index);
    await fileButton.click();
  }

  async openSimulatorTab(): Promise<void> {
    await this.page.getByRole("button", { name: "Simulator" }).click();
  }

  async openRulesTab(): Promise<void> {
    await this.page.getByRole("button", { name: "Rules" }).click();
  }

  header(): Locator {
    return this.page.getByRole("banner");
  }
}

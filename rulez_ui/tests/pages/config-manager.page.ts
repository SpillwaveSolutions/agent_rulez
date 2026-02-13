import type { Locator } from "@playwright/test";
import { BasePage } from "./base.page";

export class ConfigManagerPage extends BasePage {
  async switchScope(scope: "global" | "project"): Promise<void> {
    await this.page.getByRole("button", { name: new RegExp(scope, "i") }).click();
  }

  async importConfig(filename: string): Promise<void> {
    await this.page.getByRole("button", { name: /import/i }).click();
    const fileInput = this.page.locator("input[type=file]");
    await fileInput.setInputFiles(filename);
  }

  async exportConfig(): Promise<void> {
    await this.page.getByRole("button", { name: /export/i }).click();
  }

  scopeIndicator(): Locator {
    return this.page.getByTestId("scope-indicator");
  }
}

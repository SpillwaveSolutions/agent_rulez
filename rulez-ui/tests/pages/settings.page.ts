import type { Locator } from "@playwright/test";
import { BasePage } from "./base.page";

export class SettingsPage extends BasePage {
  async openSettings(): Promise<void> {
    await this.page.getByRole("button", { name: /settings/i }).click();
  }

  themeSelect(): Locator {
    return this.page.getByRole("combobox", { name: /theme/i });
  }

  async selectTheme(theme: "light" | "dark" | "system"): Promise<void> {
    const selector = this.themeSelect();
    if ((await selector.count()) > 0) {
      await selector.selectOption(theme);
      return;
    }

    const themeButton = this.page.getByRole("button", { name: new RegExp(theme, "i") });
    await themeButton.click();
  }

  async setFontSize(size: number): Promise<void> {
    const fontInput = this.page.getByRole("spinbutton", { name: /font size/i });
    await fontInput.fill(String(size));
  }

  async setBinaryPath(path: string): Promise<void> {
    const binaryInput = this.page.getByRole("textbox", { name: /binary path/i });
    await binaryInput.fill(path);
  }

  async saveSettings(): Promise<void> {
    await this.page.getByRole("button", { name: /save/i }).click();
  }

  async getTheme(): Promise<string> {
    const selector = this.themeSelect();
    if ((await selector.count()) > 0) {
      return selector.inputValue();
    }

    const selectedRadio = this.page.getByRole("radio", { checked: true });
    const label = await selectedRadio.textContent();
    return (label ?? "").trim();
  }
}

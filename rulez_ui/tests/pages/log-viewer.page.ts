import type { Locator, Page } from "@playwright/test";
import { BasePage } from "./base.page";

export class LogViewerPage extends BasePage {
  constructor(page: Page) {
    super(page);
  }

  async openLogViewer(): Promise<void> {
    await this.page.getByRole("button", { name: /log viewer|logs/i }).click();
  }

  async filterByText(text: string): Promise<void> {
    const search = this.page.getByPlaceholder(/search|filter/i);
    await search.fill(text);
  }

  async filterBySeverity(level: string): Promise<void> {
    const severitySelect = this.page.getByRole("combobox", { name: /severity/i });
    if ((await severitySelect.count()) > 0) {
      await severitySelect.selectOption(level);
      return;
    }

    await this.page.getByRole("button", { name: /severity/i }).click();
    await this.page.getByRole("menuitem", { name: new RegExp(level, "i") }).click();
  }

  async exportLogs(format: "json" | "csv"): Promise<void> {
    await this.page.getByRole("button", { name: /export/i }).click();
    const exportOption = this.page.getByRole("menuitem", { name: new RegExp(format, "i") });
    if ((await exportOption.count()) > 0) {
      await exportOption.click();
      return;
    }
    await this.page.getByRole("button", { name: new RegExp(format, "i") }).click();
  }

  async getVisibleLogCount(): Promise<number> {
    const entries = this.page.getByTestId("log-entry");
    return entries.count();
  }

  async copyLogEntry(index: number): Promise<void> {
    const entries = this.page.getByTestId("log-entry");
    const entry = entries.nth(index);
    await entry.getByRole("button", { name: /copy/i }).click();
  }
}

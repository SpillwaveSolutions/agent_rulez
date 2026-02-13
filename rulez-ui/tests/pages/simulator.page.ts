import { type Locator, expect } from "@playwright/test";
import { BasePage } from "./base.page";

export class SimulatorPage extends BasePage {
  async openSimulator(): Promise<void> {
    await this.openSimulatorTab();
    await expect(this.page.getByText("Debug Simulator")).toBeVisible();
  }

  eventTypeSelect(): Locator {
    return this.page.locator("select").first();
  }

  simulateButton(): Locator {
    return this.page.getByRole("button", { name: /simulate/i });
  }

  async selectEventType(type: string): Promise<void> {
    await this.eventTypeSelect().selectOption(type);
  }

  async fillTool(value: string): Promise<void> {
    await this.page.getByPlaceholder(/tool/i).fill(value);
  }

  async fillCommand(value: string): Promise<void> {
    await this.page.getByPlaceholder(/command/i).fill(value);
  }

  async runSimulation(): Promise<void> {
    await this.simulateButton().click();
  }

  outcomeBadge(): Locator {
    return this.page.locator("text=/Allow|Block|Inject/i");
  }
}

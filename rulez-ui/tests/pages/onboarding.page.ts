import type { Locator } from "@playwright/test";
import { BasePage } from "./base.page";

export class OnboardingPage extends BasePage {
  wizard(): Locator {
    return this.page.getByRole("dialog", { name: /onboarding|welcome|get started/i });
  }

  async isWizardVisible(): Promise<boolean> {
    return this.wizard().isVisible();
  }

  async checkBinaryDetection(): Promise<boolean> {
    const status = this.page.getByText(/binary.*(found|detected)/i);
    return status.isVisible();
  }

  async generateSampleConfig(): Promise<void> {
    await this.page.getByRole("button", { name: /sample config|generate/i }).click();
  }

  async runTestSimulation(): Promise<void> {
    await this.page.getByRole("button", { name: /run test|simulate/i }).click();
  }

  async completeWizard(): Promise<void> {
    await this.page.getByRole("button", { name: /finish|complete|done/i }).click();
  }

  async skipOnboarding(): Promise<void> {
    await this.page.getByRole("button", { name: /skip/i }).click();
  }
}

import { type Locator, expect } from "@playwright/test";
import { BasePage } from "./base.page";

export class EditorPage extends BasePage {
  editorContainer(): Locator {
    return this.page.locator(".monaco-editor");
  }

  editorContent(): Locator {
    return this.page.locator(".monaco-editor .view-lines");
  }

  async openDefaultFile(): Promise<void> {
    await this.openFileByName("hooks.yaml", 0);
    await this.page.waitForTimeout(300);
    await expect(this.editorContainer()).toBeVisible();
  }

  async typeInEditor(text: string): Promise<void> {
    await this.editorContent().click();
    await this.page.keyboard.type(text);
  }

  async formatDocument(): Promise<void> {
    const formatButton = this.page.getByRole("button", { name: /format/i });
    await formatButton.click();
  }

  validationPanel(): Locator {
    return this.page.getByText("Problems");
  }
}

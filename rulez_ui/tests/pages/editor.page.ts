import type { Locator, Page } from "@playwright/test";
import { BasePage } from "./base.page";

/**
 * Page Object for the Monaco editor.
 * Handles code editing, content manipulation, and editor state.
 */
export class EditorPage extends BasePage {
  readonly container: Locator;
  readonly monacoEditor: Locator;
  readonly textArea: Locator;

  constructor(page: Page) {
    super(page);

    this.container = page.locator("[data-testid='editor-container']").or(
      page.locator(".monaco-editor").first()
    );
    this.monacoEditor = page.locator(".monaco-editor");
    this.textArea = page.locator(".monaco-editor textarea");
  }

  /**
   * Wait for editor to be ready
   */
  async waitForReady(): Promise<void> {
    await this.monacoEditor.waitFor({ state: "visible" });
  }

  /**
   * Get the current editor content
   * Note: This reads from Monaco's internal model
   */
  async getContent(): Promise<string> {
    return this.page.evaluate(() => {
      // Access Monaco editor model through window
      const editors = (window as unknown as { monaco?: { editor: { getEditors: () => Array<{ getValue: () => string }> } } }).monaco?.editor.getEditors();
      if (editors && editors.length > 0) {
        return editors[0].getValue();
      }
      return "";
    });
  }

  /**
   * Set the editor content
   */
  async setContent(content: string): Promise<void> {
    await this.page.evaluate((text) => {
      const editors = (window as unknown as { monaco?: { editor: { getEditors: () => Array<{ setValue: (value: string) => void }> } } }).monaco?.editor.getEditors();
      if (editors && editors.length > 0) {
        editors[0].setValue(text);
      }
    }, content);
  }

  /**
   * Type text at current cursor position
   */
  async typeText(text: string): Promise<void> {
    await this.textArea.focus();
    await this.page.keyboard.type(text);
  }

  /**
   * Press a keyboard shortcut
   */
  async pressShortcut(shortcut: string): Promise<void> {
    await this.textArea.focus();
    await this.page.keyboard.press(shortcut);
  }

  /**
   * Save the current file (Ctrl/Cmd+S)
   */
  async save(): Promise<void> {
    const modifier = process.platform === "darwin" ? "Meta" : "Control";
    await this.pressShortcut(`${modifier}+s`);
    await this.waitBriefly(200);
  }

  /**
   * Undo last action (Ctrl/Cmd+Z)
   */
  async undo(): Promise<void> {
    const modifier = process.platform === "darwin" ? "Meta" : "Control";
    await this.pressShortcut(`${modifier}+z`);
  }

  /**
   * Redo last action (Ctrl/Cmd+Shift+Z or Ctrl+Y)
   */
  async redo(): Promise<void> {
    const modifier = process.platform === "darwin" ? "Meta" : "Control";
    await this.pressShortcut(`${modifier}+Shift+z`);
  }

  /**
   * Select all content (Ctrl/Cmd+A)
   */
  async selectAll(): Promise<void> {
    const modifier = process.platform === "darwin" ? "Meta" : "Control";
    await this.pressShortcut(`${modifier}+a`);
  }

  /**
   * Check if editor has unsaved changes
   */
  async hasUnsavedChanges(): Promise<boolean> {
    // Look for modified indicator (usually a dot or asterisk in tab)
    const modifiedIndicator = this.page.locator("[data-testid='modified-indicator']").or(
      this.page.locator("text=/\\*.*\\.yaml/")
    );
    return modifiedIndicator.isVisible();
  }

  /**
   * Go to a specific line
   */
  async goToLine(lineNumber: number): Promise<void> {
    const modifier = process.platform === "darwin" ? "Meta" : "Control";
    await this.pressShortcut(`${modifier}+g`);
    await this.page.keyboard.type(lineNumber.toString());
    await this.page.keyboard.press("Enter");
  }

  /**
   * Find text in editor (Ctrl/Cmd+F)
   */
  async openFind(): Promise<void> {
    const modifier = process.platform === "darwin" ? "Meta" : "Control";
    await this.pressShortcut(`${modifier}+f`);
  }

  /**
   * Replace text in editor (Ctrl/Cmd+H)
   */
  async openReplace(): Promise<void> {
    const modifier = process.platform === "darwin" ? "Meta" : "Control";
    await this.pressShortcut(`${modifier}+h`);
  }

  /**
   * Check if editor is visible and ready
   */
  async isReady(): Promise<boolean> {
    return this.monacoEditor.isVisible();
  }

  /**
   * Get error markers count from Monaco
   */
  async getErrorCount(): Promise<number> {
    return this.page.evaluate(() => {
      const markers = (window as unknown as { monaco?: { editor: { getModelMarkers: (opts: object) => Array<unknown> } } }).monaco?.editor.getModelMarkers({});
      return markers?.filter((m: unknown) => (m as { severity: number }).severity === 8).length || 0;
    });
  }

  /**
   * Get warning markers count from Monaco
   */
  async getWarningCount(): Promise<number> {
    return this.page.evaluate(() => {
      const markers = (window as unknown as { monaco?: { editor: { getModelMarkers: (opts: object) => Array<unknown> } } }).monaco?.editor.getModelMarkers({});
      return markers?.filter((m: unknown) => (m as { severity: number }).severity === 4).length || 0;
    });
  }
}

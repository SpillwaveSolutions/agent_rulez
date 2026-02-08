import type { Locator, Page } from "@playwright/test";
import { BasePage } from "./base.page";

/**
 * Page Object for dialog interactions.
 * Handles confirm dialogs, modals, and alerts.
 */
export class DialogsPage extends BasePage {
  // Common dialog elements
  readonly overlay: Locator;
  readonly dialog: Locator;

  // Confirm dialog elements
  readonly confirmDialog: Locator;
  readonly confirmTitle: Locator;
  readonly confirmMessage: Locator;
  readonly confirmButton: Locator;
  readonly cancelButton: Locator;

  constructor(page: Page) {
    super(page);

    // Dialog backdrop/overlay
    this.overlay = page
      .locator("[data-testid='dialog-overlay']")
      .or(page.locator(".overlay").or(page.locator("[role='dialog']").locator("..")));

    // Generic dialog
    this.dialog = page.locator("[role='dialog']").or(page.locator("[data-testid='dialog']"));

    // Confirm dialog specifics
    this.confirmDialog = page
      .locator("[data-testid='confirm-dialog']")
      .or(page.locator("[role='alertdialog']"));
    this.confirmTitle = this.dialog.locator("h2, h3, [data-testid='dialog-title']");
    this.confirmMessage = this.dialog.locator("p, [data-testid='dialog-message']");
    this.confirmButton = page.getByRole("button", { name: /confirm|ok|yes|save|delete/i });
    this.cancelButton = page.getByRole("button", { name: /cancel|no|close/i });
  }

  /**
   * Check if any dialog is visible
   */
  async isDialogVisible(): Promise<boolean> {
    return this.dialog.isVisible();
  }

  /**
   * Wait for dialog to appear
   */
  async waitForDialog(timeout = 5000): Promise<void> {
    await this.dialog.waitFor({ state: "visible", timeout });
  }

  /**
   * Wait for dialog to close
   */
  async waitForDialogClose(timeout = 5000): Promise<void> {
    await this.dialog.waitFor({ state: "hidden", timeout });
  }

  /**
   * Get dialog title text
   */
  async getTitle(): Promise<string | null> {
    return this.confirmTitle.textContent();
  }

  /**
   * Get dialog message text
   */
  async getMessage(): Promise<string | null> {
    return this.confirmMessage.textContent();
  }

  /**
   * Confirm the dialog (click confirm/ok/yes)
   */
  async confirm(): Promise<void> {
    await this.confirmButton.click();
    await this.waitForDialogClose();
  }

  /**
   * Cancel the dialog (click cancel/no)
   */
  async cancel(): Promise<void> {
    await this.cancelButton.click();
    await this.waitForDialogClose();
  }

  /**
   * Close dialog by clicking overlay (if supported)
   */
  async closeByOverlay(): Promise<void> {
    await this.overlay.click({ position: { x: 10, y: 10 } });
    await this.waitForDialogClose();
  }

  /**
   * Close dialog by pressing Escape
   */
  async closeByEscape(): Promise<void> {
    await this.page.keyboard.press("Escape");
    await this.waitForDialogClose();
  }

  /**
   * Check if confirm button is visible
   */
  async hasConfirmButton(): Promise<boolean> {
    return this.confirmButton.isVisible();
  }

  /**
   * Check if cancel button is visible
   */
  async hasCancelButton(): Promise<boolean> {
    return this.cancelButton.isVisible();
  }

  /**
   * Wait for and confirm an unsaved changes dialog
   */
  async handleUnsavedChangesDialog(action: "save" | "discard" | "cancel"): Promise<void> {
    await this.waitForDialog();

    const title = await this.getTitle();
    if (!title?.toLowerCase().includes("unsaved")) {
      throw new Error("Not an unsaved changes dialog");
    }

    switch (action) {
      case "save":
        await this.page.getByRole("button", { name: /save/i }).click();
        break;
      case "discard":
        await this.page.getByRole("button", { name: /discard|don't save/i }).click();
        break;
      case "cancel":
        await this.cancel();
        break;
    }

    await this.waitForDialogClose();
  }

  /**
   * Handle a delete confirmation dialog
   */
  async handleDeleteConfirmation(confirm: boolean): Promise<void> {
    await this.waitForDialog();

    if (confirm) {
      await this.page.getByRole("button", { name: /delete|confirm|yes/i }).click();
    } else {
      await this.cancel();
    }

    await this.waitForDialogClose();
  }
}

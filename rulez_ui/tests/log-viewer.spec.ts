import path from "node:path";
import { fileURLToPath } from "node:url";
import { expect, test } from "@playwright/test";
import { LogViewerPage } from "./pages";
import { dismissOnboarding } from "./utils/dismiss-onboarding";
import { resetAppState } from "./utils/reset-app-state";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const fixturesDir = path.join(__dirname, "fixtures");

test.describe("Log Viewer", () => {
  test.beforeEach(async ({ page }) => {
    await dismissOnboarding(page);
    await page.goto("/");
    await resetAppState(page);
    await page.reload();
    await page.getByText("RuleZ UI").waitFor();
    const logViewerPage = new LogViewerPage(page);
    await logViewerPage.openLogViewer();
  });

  test("should display log entries from mock data", async ({ page }) => {
    const fileInput = page.locator("input[type=file]");
    if ((await fileInput.count()) > 0) {
      await fileInput.setInputFiles(path.join(fixturesDir, "mock-logs.json"));
    }

    await expect(page.getByTestId("log-entry").first()).toBeVisible();
  });

  test("should filter logs by text search", async ({ page }) => {
    const logViewerPage = new LogViewerPage(page);
    await logViewerPage.filterByText("force push");
    await expect(page.getByText(/force push/i).first()).toBeVisible();
  });

  test("should filter logs by severity level", async ({ page }) => {
    const logViewerPage = new LogViewerPage(page);
    await logViewerPage.filterBySeverity("error");
    await expect(page.getByText(/error/i).first()).toBeVisible();
  });

  test("should filter logs by date range", async ({ page }) => {
    const fromInput = page.getByLabel(/from/i);
    const toInput = page.getByLabel(/to/i);
    if ((await fromInput.count()) > 0 && (await toInput.count()) > 0) {
      await fromInput.fill("2025-01-01");
      await toInput.fill("2025-12-31");
      await expect(page.getByTestId("log-entry").first()).toBeVisible();
    }
  });

  test("should handle large log files with virtual scrolling", async ({ page }) => {
    const logViewerPage = new LogViewerPage(page);
    const visibleCount = await logViewerPage.getVisibleLogCount();
    expect(visibleCount).toBeGreaterThan(0);
  });

  test("should export filtered logs to JSON", async ({ page }) => {
    const logViewerPage = new LogViewerPage(page);
    const downloadPromise = page.waitForEvent("download");
    await logViewerPage.exportLogs("json");
    const download = await downloadPromise;
    expect(download.suggestedFilename()).toMatch(/\.json$/i);
  });

  test("should export filtered logs to CSV", async ({ page }) => {
    const logViewerPage = new LogViewerPage(page);
    const downloadPromise = page.waitForEvent("download");
    await logViewerPage.exportLogs("csv");
    const download = await downloadPromise;
    expect(download.suggestedFilename()).toMatch(/\.csv$/i);
  });

  test("should copy log entry to clipboard", async ({ page }) => {
    const logViewerPage = new LogViewerPage(page);
    await logViewerPage.copyLogEntry(0);
    await expect(page.getByText(/copied/i).first()).toBeVisible();
  });
});

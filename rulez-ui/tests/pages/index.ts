/**
 * Page Object Models for Playwright E2E tests.
 *
 * Usage:
 *   import { AppShellPage, SimulatorPage } from "./pages";
 *
 *   test("example", async ({ page }) => {
 *     const app = new AppShellPage(page);
 *     await app.init();
 *     ...
 *   });
 */

export { BasePage } from "./base.page";
export { AppShellPage } from "./app-shell.page";
export { SidebarPage } from "./sidebar.page";
export { EditorPage } from "./editor.page";
export { SimulatorPage } from "./simulator.page";
export type { EventType, SimulationOutcome } from "./simulator.page";
export { FileTabBarPage } from "./file-tab-bar.page";
export { DialogsPage } from "./dialogs.page";

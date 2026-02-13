import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "./tests",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: process.env.CI
    ? [
        ["html", { outputFolder: "playwright-report" }],
        ["junit", { outputFile: "test-results/junit.xml" }],
        ["github"],
      ]
    : [["html", { open: "never" }]],
  use: {
    baseURL: "http://localhost:1420",
    trace: process.env.CI ? "on-first-retry" : "retain-on-failure",
    screenshot: process.env.CI ? "only-on-failure" : "off",
    video: process.env.CI ? "on-first-retry" : "off",
  },
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
    {
      name: "webkit",
      use: { ...devices["Desktop Safari"] },
    },
  ],
  webServer: {
    command: "bun run dev",
    url: "http://localhost:1420",
    reuseExistingServer: !process.env.CI,
    timeout: 120 * 1000,
  },
  outputDir: "test-results/",
  preserveOutput: "failures-only",
});

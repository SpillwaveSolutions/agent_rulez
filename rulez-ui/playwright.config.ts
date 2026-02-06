import { defineConfig, devices } from "@playwright/test";

/**
 * Playwright configuration for RuleZ UI E2E tests.
 *
 * Running tests:
 *   bunx playwright test                # Run all tests
 *   bunx playwright test --headed       # Run with browser visible
 *   bunx playwright test --debug        # Run in debug mode
 *   bunx playwright test --ui           # Open Playwright UI
 *   bunx playwright show-report         # View HTML report
 *
 * @see https://playwright.dev/docs/test-configuration
 */
export default defineConfig({
  testDir: "./tests",

  // Run tests in parallel
  fullyParallel: true,

  // Fail fast on CI to avoid long waits
  forbidOnly: !!process.env.CI,

  // Retry on CI for flaky test detection
  retries: process.env.CI ? 2 : 0,

  // Limit workers on CI to avoid resource contention
  workers: process.env.CI ? 1 : undefined,

  // Reporter configuration
  reporter: process.env.CI
    ? [
        ["html", { outputFolder: "playwright-report" }],
        ["junit", { outputFile: "test-results/junit.xml" }],
        ["github"],
      ]
    : [["html", { open: "never" }]],

  // Global test settings
  use: {
    baseURL: "http://localhost:1420",

    // Trace collection
    trace: process.env.CI ? "on-first-retry" : "retain-on-failure",

    // Screenshot settings
    screenshot: process.env.CI ? "only-on-failure" : "off",

    // Video capture on CI retry
    video: process.env.CI ? "on-first-retry" : "off",

    // Timeout for actions like click, fill
    actionTimeout: 10000,

    // Navigation timeout
    navigationTimeout: 30000,

    // Viewport size
    viewport: { width: 1280, height: 720 },
  },

  // Global timeout for each test
  timeout: 60000,

  // Expect timeout
  expect: {
    timeout: 10000,
    // Visual comparison settings
    toHaveScreenshot: {
      maxDiffPixels: 100,
      threshold: 0.2,
    },
    toMatchSnapshot: {
      maxDiffPixelRatio: 0.1,
    },
  },

  // Browser projects
  projects: [
    {
      name: "chromium",
      use: {
        ...devices["Desktop Chrome"],
        // Use channel for more realistic browser
        channel: process.env.CI ? undefined : "chrome",
      },
    },
    {
      name: "webkit",
      use: { ...devices["Desktop Safari"] },
    },
    // Firefox can be added if needed
    // {
    //   name: "firefox",
    //   use: { ...devices["Desktop Firefox"] },
    // },
  ],

  // Dev server configuration
  webServer: {
    command: "bun run dev",
    url: "http://localhost:1420",
    reuseExistingServer: !process.env.CI,
    timeout: 120 * 1000,
    stdout: "pipe",
    stderr: "pipe",
  },

  // Output directory for test artifacts
  outputDir: "test-results/",

  // Preserve test output on failure
  preserveOutput: "failures-only",
});

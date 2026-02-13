# Testing Patterns

**Analysis Date:** 2026-02-06

## Test Framework

**Unit/Integration Test Runner:**
- Bun test (built-in, zero-config)
- Run: `bun test`
- Config: Implicit (uses Bun's default test runner)

**E2E Test Framework:**
- Playwright 1.50.1
- Config: `playwright.config.ts`
- Run: `bun run test:e2e` or `bunx playwright test`

**Assertion Library:**
- Playwright Test assertions (built-in)
- `expect()` from `@playwright/test`

**Test Commands:**
```bash
bun test                           # Run all unit tests
bun run test:e2e                   # Run Playwright E2E tests
bunx playwright test --headed      # E2E with visible browser
bunx playwright test --debug       # E2E debug mode
bunx playwright test --ui          # E2E in UI mode
bunx playwright show-report        # View HTML test report
```

## Test File Organization

**Location:**
- E2E tests: `tests/` directory (co-located with source, not in src/)
- Playwright config: root directory (`playwright.config.ts`)
- Test fixtures: `tests/fixtures/`
- Page objects: `tests/pages/`

**Naming:**
- Test spec files: `*.spec.ts` (e.g., `app.spec.ts`, `editor.spec.ts`)
- Page objects: `*.page.ts` (e.g., `base.page.ts`, `app-shell.page.ts`)
- Fixtures: organized by type (e.g., `fixtures/mock-configs/`, `fixtures/events/`)

**Structure:**
```
tests/
├── app.spec.ts                    # Application-level tests
├── editor.spec.ts                 # Editor functionality tests
├── file-ops.spec.ts              # File operation tests
├── simulator.spec.ts             # Debug simulator tests
├── tree-view.spec.ts             # Rules tree view tests
├── pages/                         # Page object models
│   ├── base.page.ts             # Base class with helpers
│   ├── app-shell.page.ts        # App layout selectors
│   ├── editor.page.ts           # Editor page object
│   ├── simulator.page.ts        # Simulator page object
│   └── index.ts                 # Barrel export
└── fixtures/                      # Test data
    ├── mock-configs/            # YAML config files
    │   ├── valid-basic.yaml
    │   ├── invalid-syntax.yaml
    │   ├── empty.yaml
    │   └── large.yaml
    ├── events/                  # Event scenario JSON
    │   ├── allow-scenarios.json
    │   ├── block-scenarios.json
    │   └── inject-scenarios.json
    └── index.ts                 # Fixture loaders
```

## Test Structure

**Suite Organization:**
```typescript
import { expect, test } from "@playwright/test";

test.describe("Feature Name", () => {
  test.beforeEach(async ({ page }) => {
    // Setup before each test
    await page.goto("/");
    await page.waitForTimeout(500);
  });

  test("should perform action X", async ({ page }) => {
    // Arrange: Get elements
    const element = page.locator('[data-testid="selector"]');

    // Act: Perform action
    await element.click();

    // Assert: Check result
    await expect(page.getByText("Expected")).toBeVisible();
  });
});
```

**Patterns:**
- Use `test.describe()` for grouping related tests
- Use `test.beforeEach()` for common setup
- Test names are descriptive and start with "should"
- Follow Arrange-Act-Assert pattern

**Example from `app.spec.ts`:**
```typescript
test("should load the application", async ({ page }) => {
  await page.goto("/");

  // Check that the header is visible
  await expect(page.getByText("RuleZ UI")).toBeVisible();

  // Check that we're in web mode (not Tauri)
  await expect(page.getByText("Web (Test)")).toBeVisible();
});
```

## Mocking

**Framework:** Page object models and fixture loaders

**Patterns:**

### Page Objects
Base class with common helpers (`tests/pages/base.page.ts`):
```typescript
export class BasePage {
  readonly page: Page;

  constructor(page: Page) {
    this.page = page;
  }

  async goto(): Promise<void> {
    await this.page.goto("/");
  }

  async waitForVisible(locator: Locator, timeout = 5000): Promise<void> {
    await locator.waitFor({ state: "visible", timeout });
  }

  async clickAndWait(locator: Locator): Promise<void> {
    await locator.click();
    await this.page.waitForLoadState("networkidle");
  }
}
```

Specialized page objects extend base and provide domain-specific selectors:
```typescript
export class AppShellPage extends BasePage {
  // Selectors for app-shell specific elements
  // Methods wrapping common interactions
}
```

### Fixtures
Test data loading functions (`tests/fixtures/index.ts`):
```typescript
export function loadMockConfig(name: string): string {
  const configPath = join(__dirname, "mock-configs", `${name}.yaml`);
  return readFileSync(configPath, "utf-8");
}

export function getBlockScenarios(): EventScenario[] {
  return loadEventScenarios("block-scenarios.json").scenarios;
}

export const mockConfigs = {
  validBasic: "valid-basic",
  invalidSyntax: "invalid-syntax",
  empty: "empty",
  large: "large",
} as const;
```

**What to Mock:**
- Tauri IPC commands (fallback to mock data in web mode)
- File system operations (use fixture files)
- Event scenarios (load from JSON fixtures)

**What NOT to Mock:**
- Real browser interactions (DOM, navigation)
- Playwright assertions
- Real Monaco Editor behavior (interact with real instance)

## Fixtures and Factories

**Test Data:**
Fixtures are YAML and JSON files in `tests/fixtures/`:
- Mock configurations: `mock-configs/*.yaml`
- Event scenarios: `events/*.json`
- Pre-loaded via `mockConfigContents` object

**Example fixture loader:**
```typescript
export function getBlockScenarios(): EventScenario[] {
  return loadEventScenarios("block-scenarios.json").scenarios;
}

export interface EventScenario {
  name: string;
  event: {
    hook_event_name: string;
    tool_name: string;
    tool_input: Record<string, string>;
  };
  expectedOutcome: "Allow" | "Block" | "Inject";
  expectedReason?: string;
  expectedContext?: string;
}
```

**Location:**
- Fixtures: `tests/fixtures/`
- Loaders: `tests/fixtures/index.ts`

## Coverage

**Requirements:** Not enforced

**Known Test Coverage:**
- E2E tests cover: app loading, file operations, editor, simulator, theme toggle
- Tests focus on user workflows, not code coverage percentage
- No explicit coverage reporting configured

## Test Types

**Unit Tests:**
- Framework: Bun test
- Scope: Individual functions, stores, utilities
- Approach: Direct function calls with mocked dependencies
- Example: Testing Zustand store selectors

**Integration Tests:**
- Framework: Playwright
- Scope: Component interactions, state management, file operations
- Approach: Start app, interact with UI, verify results
- Example: Open file from sidebar → verify tab appears → modify content → save

**E2E Tests:**
- Framework: Playwright with multiple browsers
- Scope: Full user workflows
- Browsers: Chromium, WebKit
- Configuration in `playwright.config.ts`:
  - `fullyParallel: true` - Tests run in parallel
  - `retries: 2` on CI, 0 locally
  - `workers: 1` on CI for stability
  - Trace/screenshot/video collection on failure/CI
  - Viewport: 1280x720

## Common Patterns

**Async Testing:**
```typescript
test("should handle async operation", async ({ page }) => {
  setIsLoading(true);
  setError(null);
  try {
    const result = await runDebug(params);
    setResult(result);
  } catch (err) {
    setError(err instanceof Error ? err.message : "Failed");
  } finally {
    setIsLoading(false);
  }
});
```

**Error Testing:**
```typescript
test("should show error message on failure", async ({ page }) => {
  // Trigger error condition
  await page.locator('[data-testid="invalid-input"]').click();

  // Assert error is displayed
  await expect(page.getByText(/error message/i)).toBeVisible();
});
```

**DOM Selectors:**
- Prefer `data-testid` attributes for reliable test selectors
- Example from `file-ops.spec.ts`:
  ```typescript
  const globalFile = page.locator('[data-testid="sidebar-global-file-hooks.yaml"]');
  await globalFile.click();

  // Verify tab appears
  await expect(page.locator('[data-testid="file-tab-hooks.yaml"]')).toBeVisible();
  ```

**Waiting Patterns:**
```typescript
// Wait for specific element
await page.waitForTimeout(500);

// Wait for load state
await page.waitForLoadState("networkidle");

// Wait for element visibility
await expect(locator).toBeVisible({ timeout: 2000 });

// Use page object helper
await basePage.waitForVisible(locator, 5000);
```

**Keyboard Interactions:**
```typescript
// Type text
await page.keyboard.type("text to type");

// Press arrow keys
await page.keyboard.press("ArrowDown");
await page.keyboard.press("ArrowRight");

// Press Escape (handled by components)
await page.keyboard.press("Escape");
```

**Playwright Config Settings:**
- `baseURL: "http://localhost:1420"` - Tauri dev server port
- `actionTimeout: 10000` - Timeout for click, fill, etc.
- `navigationTimeout: 30000` - Page navigation timeout
- `timeout: 60000` - Global test timeout
- `expect.timeout: 10000` - Assertion timeout
- `webServer.command: "bun run dev"` - Auto-start dev server
- `webServer.reuseExistingServer: !process.env.CI` - Reuse server locally

---

*Testing analysis: 2026-02-06*

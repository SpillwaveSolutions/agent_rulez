import { describe, expect, test, mock } from "bun:test";
import { isTauri } from "./tauri";

describe("tauri.ts", () => {
  describe("isTauri", () => {
    test("returns false when __TAURI__ is not defined", () => {
      // In test environment, __TAURI__ should not be defined
      expect(isTauri()).toBe(false);
    });

    test("returns true when __TAURI__ is defined", () => {
      // Mock window.__TAURI__
      const originalWindow = globalThis.window;
      // @ts-expect-error - mocking window for test
      globalThis.window = { __TAURI__: {} };

      expect(isTauri()).toBe(true);

      // Restore
      // @ts-expect-error - restoring window for test
      globalThis.window = originalWindow;
    });
  });
});

describe("mock-data", () => {
  test("getMockConfigFiles returns files array", async () => {
    const { getMockConfigFiles } = await import("./mock-data");
    const files = getMockConfigFiles();

    expect(files).toBeArray();
    expect(files.length).toBeGreaterThan(0);
    expect(files[0]).toHaveProperty("path");
    expect(files[0]).toHaveProperty("exists");
  });

  test("getMockConfig returns string content", async () => {
    const { getMockConfig } = await import("./mock-data");
    const content = getMockConfig("~/.claude/hooks.yaml");

    expect(content).toBeString();
    expect(content).toContain("version");
    expect(content).toContain("rules");
  });

  test("setMockConfig updates stored content", async () => {
    const { getMockConfig, setMockConfig } = await import("./mock-data");
    const testPath = "test/config.yaml";
    const testContent = "test: content";

    setMockConfig(testPath, testContent);
    const retrieved = getMockConfig(testPath);

    expect(retrieved).toBe(testContent);
  });
});

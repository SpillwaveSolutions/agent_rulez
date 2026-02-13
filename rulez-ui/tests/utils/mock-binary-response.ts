import type { Page } from "@playwright/test";

export interface MockDebugResponse {
  outcome: "Allow" | "Block" | "Inject";
  reason?: string;
  matchedRules: string[];
  evaluationTimeMs: number;
  evaluations: Array<{
    ruleName: string;
    matched: boolean;
    timeMs: number;
    details?: string;
    pattern?: string;
    input?: string;
  }>;
}

declare global {
  interface Window {
    __rulezMockDebugResponse?: MockDebugResponse;
  }
}

export async function mockBinaryResponse(page: Page, response: MockDebugResponse): Promise<void> {
  await page.addInitScript((value) => {
    window.__rulezMockDebugResponse = value;
  }, response);
}

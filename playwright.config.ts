import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "e2e",
  timeout: 30_000,
  expect: { timeout: 10_000 },
  use: {
    baseURL: process.env.E2E_BASE_URL || "http://127.0.0.1:8080",
    headless: process.env.E2E_HEADLESS !== "false",
    viewport: { width: 1280, height: 720 },
    trace: "retain-on-failure",
    video: "retain-on-failure",
  },
});



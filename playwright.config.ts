import { defineConfig } from "@playwright/test";

// End-to-end smoke test of the Electron application (ADR-018).
// Requires a prior `npm run build`.
export default defineConfig({
  testDir: "e2e",
  timeout: 60_000,
  retries: 0,
  workers: 1,
  reporter: [["list"]],
});

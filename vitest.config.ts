import { defineConfig } from "vitest/config";

// Unit tests of the TypeScript side and of the Node scripts.
// Coverage minimum 70 % lines (docs/project-context.md section 7).
export default defineConfig({
  test: {
    include: ["app/**/*.test.ts", "scripts/**/*.test.mjs"],
    exclude: ["**/node_modules/**", "spikes/**", "e2e/**"],
    coverage: {
      provider: "v8",
      include: ["app/src/**/*.ts", "app/renderer/**/*.ts", "scripts/**/*.mjs"],
      exclude: [
        "**/*.test.ts",
        "**/*.test.mjs",
        // Electron entry points: only wiring, covered by the smoke test e2e/smoke.spec.ts (ADR-018).
        "app/src/main/main.ts",
        "app/src/main/core-host.ts",
        "app/renderer/main.ts",
      ],
      reporter: ["text", "lcov"],
      thresholds: { lines: 70 },
    },
  },
});

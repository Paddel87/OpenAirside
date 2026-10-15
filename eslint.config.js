// ESLint configuration (flat config) of OpenAirside, docs/project-context.md section 7.
import js from "@eslint/js";
import tseslint from "typescript-eslint";

export default tseslint.config(
  {
    ignores: [
      "**/node_modules/**",
      "**/dist/**",
      "app/native/**",
      "coverage/**",
      "release/**",
      "target/**",
      "test-results/**",
      "playwright-report/**",
      "spikes/**",
      "templates/**",
    ],
  },
  js.configs.recommended,
  {
    files: ["**/*.ts", "**/*.cts"],
    extends: [tseslint.configs.strictTypeChecked, tseslint.configs.stylisticTypeChecked],
    languageOptions: {
      parserOptions: { projectService: true, tsconfigRootDir: import.meta.dirname },
    },
  },
  {
    // Sandboxed preloads must be CommonJS; under verbatimModuleSyntax TypeScript then needs
    // `import x = require(...)` (app/src/main/preload.cts, roadmap step 2.7).
    files: ["**/*.cts"],
    rules: { "@typescript-eslint/no-require-imports": ["error", { allowAsImport: true }] },
  },
  {
    files: ["**/*.js", "**/*.mjs"],
    languageOptions: {
      globals: { console: "readonly", process: "readonly", URL: "readonly" },
    },
  },
);

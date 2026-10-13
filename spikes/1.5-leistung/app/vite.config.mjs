// Spike 1.4 – bundles the table panel (React, TanStack Table) into one ES module
// that the sandboxed renderer loads under the strict CSP (no eval, no inline scripts).
import { defineConfig } from "vite";

export default defineConfig({
  // Library mode does not replace process.env; React needs its production build.
  define: { "process.env.NODE_ENV": JSON.stringify("production") },
  build: {
    outDir: "renderer/dist",
    emptyOutDir: true,
    target: "chrome150",
    sourcemap: false,
    lib: {
      entry: "renderer/panel/index.js",
      formats: ["es"],
      fileName: () => "panel.js",
    },
  },
});

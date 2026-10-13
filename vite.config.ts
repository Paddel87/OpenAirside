import { defineConfig } from "vite";

// Renderer bundle only; the main process is compiled with tsc (app/tsconfig.main.json).
export default defineConfig({
  root: "app/renderer",
  base: "./",
  build: {
    outDir: "../dist/renderer",
    emptyOutDir: true,
    target: "chrome152",
  },
});

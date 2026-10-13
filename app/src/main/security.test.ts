import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { hardenedWebPreferences, isAllowedNavigation } from "./security.js";

describe("hardenedWebPreferences", () => {
  it("isolates the renderer from Node.js (project-context.md section 6)", () => {
    const prefs = hardenedWebPreferences();
    expect(prefs.contextIsolation).toBe(true);
    expect(prefs.nodeIntegration).toBe(false);
    expect(prefs.nodeIntegrationInWorker).toBe(false);
    expect(prefs.sandbox).toBe(true);
    expect(prefs.webSecurity).toBe(true);
    expect(prefs.allowRunningInsecureContent).toBe(false);
    expect(prefs.webviewTag).toBe(false);
  });
});

describe("isAllowedNavigation", () => {
  const root = "file:///opt/openairside/app/dist/renderer/";

  it("allows bundled renderer files", () => {
    expect(isAllowedNavigation(`${root}index.html`, root)).toBe(true);
  });

  it("accepts a root without trailing slash", () => {
    expect(isAllowedNavigation(`${root}index.html`, root.slice(0, -1))).toBe(true);
  });

  it.each([
    "https://example.org/",
    "http://localhost:5173/",
    "file:///etc/passwd",
    "file:///opt/openairside/app/dist/renderer-evil/index.html",
    "javascript:alert(1)",
    "not a url",
  ])("blocks %s", (url) => {
    expect(isAllowedNavigation(url, root)).toBe(false);
  });
});

describe("renderer Content Security Policy", () => {
  const html = readFileSync(new URL("../../renderer/index.html", import.meta.url), "utf8");
  const match = /http-equiv="Content-Security-Policy"\s+content="([^"]+)"/.exec(html);
  const directives = new Map(
    (match?.[1] ?? "")
      .split(";")
      .map((part) => part.trim().split(/\s+/))
      .filter((tokens) => tokens.length > 0 && tokens[0] !== "")
      .map((tokens) => [tokens[0] ?? "", tokens.slice(1)]),
  );

  it("is present", () => {
    expect(match).not.toBeNull();
  });

  it("denies everything by default", () => {
    expect(directives.get("default-src")).toEqual(["'none'"]);
  });

  it("allows only local scripts without inline or eval", () => {
    expect(directives.get("script-src")).toEqual(["'self'"]);
  });

  it("allows no network connections", () => {
    expect(directives.get("connect-src")).toEqual(["'none'"]);
  });

  it("names no remote origin", () => {
    expect(match?.[1]).not.toMatch(/https?:|\*/);
  });
});

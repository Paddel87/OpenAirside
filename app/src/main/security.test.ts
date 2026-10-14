import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import {
  hardenedWebPreferences,
  isAllowedNavigation,
  isTrustedSender,
  menuTemplate,
} from "./security.js";

describe("hardenedWebPreferences", () => {
  it("isolates the renderer from Node.js (project-context.md section 6)", () => {
    const prefs = hardenedWebPreferences("/app/preload.cjs", false);
    expect(prefs.contextIsolation).toBe(true);
    expect(prefs.nodeIntegration).toBe(false);
    expect(prefs.nodeIntegrationInWorker).toBe(false);
    expect(prefs.sandbox).toBe(true);
    expect(prefs.webSecurity).toBe(true);
    expect(prefs.allowRunningInsecureContent).toBe(false);
    expect(prefs.webviewTag).toBe(false);
    expect(prefs.devTools).toBe(false);
    expect(prefs.preload).toBe("/app/preload.cjs");
    expect(hardenedWebPreferences("/p", true).devTools).toBe(true);
  });
});

describe("isTrustedSender", () => {
  const entry = "file:///opt/openairside/app/dist/renderer/index.html";
  const good = { frameUrl: entry, isMainFrame: true, webContentsId: 7 };

  it("accepts the main frame of the game window on the start page", () => {
    expect(isTrustedSender(good, 7, entry)).toBe(true);
    expect(isTrustedSender({ ...good, frameUrl: `${entry}#x` }, 7, entry)).toBe(true);
  });

  it.each([
    { ...good, isMainFrame: false },
    { ...good, webContentsId: 8 },
    { ...good, frameUrl: "" },
    { ...good, frameUrl: "https://example.org/index.html" },
    { ...good, frameUrl: "file:///opt/openairside/app/dist/renderer/other.html" },
    { ...good, frameUrl: "file:///opt/openairside/app/dist/renderer/../renderer/index.htm" },
  ])("rejects %j", (sender) => {
    expect(isTrustedSender(sender, 7, entry)).toBe(false);
  });

  it("rejects everything for an unparsable entry", () => {
    expect(isTrustedSender(good, 7, "not a url")).toBe(false);
  });
});

describe("menuTemplate", () => {
  it("keeps the default menu in development builds", () => {
    expect(menuTemplate("linux", false)).toBeUndefined();
  });

  it("removes the menu of release builds on Windows and Linux", () => {
    expect(menuTemplate("win32", true)).toBeNull();
    expect(menuTemplate("linux", true)).toBeNull();
  });

  it("offers no reload or developer tools on macOS", () => {
    const roles = (menuTemplate("darwin", true) ?? []).map((item) => item.role);
    expect(roles).toEqual(["appMenu", "editMenu", "windowMenu"]);
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
    "file:///opt/openairside/app/dist/renderer/../../../etc/passwd",
    "file://server/opt/openairside/app/dist/renderer/index.html",
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

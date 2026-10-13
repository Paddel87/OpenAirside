/**
 * Electron hardening of OpenAirside (docs/project-context.md section 6).
 *
 * Kept free of Electron imports so that the settings can be unit-tested. The
 * Content Security Policy lives in app/renderer/index.html (checked by
 * security.test.ts), because file: pages receive no HTTP headers.
 */

/** Web preferences every OpenAirside window is created with. */
export interface HardenedWebPreferences {
  readonly contextIsolation: true;
  readonly nodeIntegration: false;
  readonly nodeIntegrationInWorker: false;
  readonly sandbox: true;
  readonly webSecurity: true;
  readonly allowRunningInsecureContent: false;
  readonly webviewTag: false;
  readonly spellcheck: false;
}

/** Returns the hardened web preferences for a renderer window. */
export function hardenedWebPreferences(): HardenedWebPreferences {
  return {
    contextIsolation: true,
    nodeIntegration: false,
    nodeIntegrationInWorker: false,
    sandbox: true,
    webSecurity: true,
    allowRunningInsecureContent: false,
    webviewTag: false,
    spellcheck: false,
  };
}

/**
 * Decides whether the renderer may navigate to a URL.
 * Only the bundled renderer files (file: URLs below `rendererRoot`) are allowed.
 */
export function isAllowedNavigation(url: string, rendererRoot: string): boolean {
  let parsed: URL;
  try {
    parsed = new URL(url);
  } catch {
    return false;
  }
  if (parsed.protocol !== "file:") {
    return false;
  }
  const root = new URL(rendererRoot.endsWith("/") ? rendererRoot : `${rendererRoot}/`);
  return parsed.href.startsWith(root.href);
}

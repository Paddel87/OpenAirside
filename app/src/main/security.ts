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
  readonly devTools: boolean;
  readonly preload: string;
}

/**
 * Returns the hardened web preferences for a renderer window.
 *
 * @param preload absolute path of the preload script (S-6)
 * @param devTools developer tools only in development builds (finding 2.1-N2)
 */
export function hardenedWebPreferences(preload: string, devTools: boolean): HardenedWebPreferences {
  return {
    contextIsolation: true,
    nodeIntegration: false,
    nodeIntegrationInWorker: false,
    sandbox: true,
    webSecurity: true,
    allowRunningInsecureContent: false,
    webviewTag: false,
    spellcheck: false,
    devTools,
    preload,
  };
}

function parse(url: string): URL | undefined {
  try {
    return new URL(url);
  } catch {
    return undefined;
  }
}

/**
 * Decides whether the renderer may navigate to a URL.
 * Only the bundled renderer files (file: URLs below `rendererRoot`) are
 * allowed. Both URLs are compared after parsing, so that `..`, percent
 * encoding and host parts cannot sneak past a string comparison
 * (Electron security checklist item 13).
 */
export function isAllowedNavigation(url: string, rendererRoot: string): boolean {
  const target = parse(url);
  const root = parse(rendererRoot.endsWith("/") ? rendererRoot : `${rendererRoot}/`);
  if (target === undefined || root === undefined) {
    return false;
  }
  return (
    target.protocol === "file:" &&
    root.protocol === "file:" &&
    target.host === root.host &&
    target.pathname.startsWith(root.pathname)
  );
}

/** What the main process knows about the sender of an IPC call. */
export interface SenderInfo {
  /** URL of the sending frame; empty if the frame is gone. */
  readonly frameUrl: string;
  /** `true` for the top-level frame of the page. */
  readonly isMainFrame: boolean;
  /** `webContents.id` of the sending page. */
  readonly webContentsId: number;
}

/**
 * Checks the sender of an IPC call (S-6 "Prüfung im Hauptprozess"): main
 * frame of the game window showing exactly the bundled start page
 * (Electron security checklist items 13 and 17).
 */
export function isTrustedSender(sender: SenderInfo, windowId: number, entryUrl: string): boolean {
  const frame = parse(sender.frameUrl);
  const entry = parse(entryUrl);
  if (frame === undefined || entry === undefined) {
    return false;
  }
  return (
    sender.isMainFrame &&
    sender.webContentsId === windowId &&
    frame.protocol === "file:" &&
    frame.protocol === entry.protocol &&
    frame.host === entry.host &&
    frame.pathname === entry.pathname
  );
}

/** Entry of the application menu, as accepted by `Menu.buildFromTemplate`. */
export interface MenuItemTemplate {
  readonly role: "appMenu" | "editMenu" | "windowMenu";
}

/**
 * Application menu: development builds keep Electron's default menu
 * (`undefined`), release builds get none on Windows and Linux (`null`) and
 * only the application, edit and window menus on macOS, without "Reload"
 * and developer tools (finding 2.1-N2).
 */
export function menuTemplate(
  platform: NodeJS.Platform,
  packaged: boolean,
): readonly MenuItemTemplate[] | null | undefined {
  if (!packaged) {
    return undefined;
  }
  if (platform === "darwin") {
    return [{ role: "appMenu" }, { role: "editMenu" }, { role: "windowMenu" }];
  }
  return null;
}

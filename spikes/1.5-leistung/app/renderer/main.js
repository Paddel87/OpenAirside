// Spike 1.5 – renderer: playfield from spike 1.3 (PixiJS, S-1 per frame, S-2)
// and flight table panel from spike 1.4, with a long-run measurement series of
// the five-floor reference scenario at the highest game speed. Throwaway code.
//
// Display logic only (rule 002): the renderer decides nothing about the game
// state. The panel shows the table the core sends and only sorts it for display.

import { createPanel } from "./dist/panel.js";

const PIXI = globalThis.PIXI;
const { Application, Container, Particle, ParticleContainer, Sprite, Texture } =
  PIXI;

// PixiJS 8.20.1: the global build of `unsafe-eval` replaces the eval-based
// particle upload only in its own bundled copy of ParticleBuffer, not in the
// one of pixi.min.js. Without this line the strict CSP blocks ParticleContainer.
Object.assign(PIXI.ParticleBuffer.prototype, {
  generateParticleUpdate: PIXI.generateParticleUpdatePolyfill,
});

window.addEventListener("error", (e) =>
  window.spike.log(`error: ${e.message} @${e.lineno}`),
);
window.addEventListener("unhandledrejection", (e) =>
  window.spike.log(`rejection: ${e.reason}`),
);

const hudEl = document.getElementById("hud");
const statusEl = document.getElementById("status");
const HEADER = 48;
const SUBCELL = 32;
const NEAR_ZOOM = 6; // px per cell from which persons are drawn as dots
const BRUSH = 3;

// ---------------------------------------------------------------- setup

function getConfig() {
  return new Promise((resolve) => window.spike.onConfig(resolve));
}

function getPort() {
  return new Promise((resolve) => {
    window.addEventListener("message", function handler(e) {
      if (e.data === "sim-port" && e.ports.length > 0) {
        window.removeEventListener("message", handler);
        resolve(e.ports[0]);
      }
    });
  });
}

const portPromise = getPort();
const cfg = await getConfig();
const port = await portPromise;

const app = new Application();
await app.init({
  preference: "webgl",
  resizeTo: window,
  antialias: false,
  autoStart: false,
  background: 0x111317,
  resolution: window.devicePixelRatio,
  autoDensity: true,
});
document.body.prepend(app.canvas);
app.ticker.stop();

function gpuName() {
  const gl = app.renderer.gl;
  if (!gl) return `renderer type ${app.renderer.name}`;
  const ext = gl.getExtension("WEBGL_debug_renderer_info");
  return String(
    ext
      ? gl.getParameter(ext.UNMASKED_RENDERER_WEBGL)
      : gl.getParameter(gl.RENDERER),
  );
}

const env = {
  gpu: gpuName(),
  pixiRenderer: app.renderer.name,
  webgl2: Boolean(
    app.renderer.gl && app.renderer.gl instanceof WebGL2RenderingContext,
  ),
  screen: {
    w: window.innerWidth,
    h: window.innerHeight,
    dpr: window.devicePixelRatio,
  },
};

// ---------------------------------------------------------------- textures

function canvasTexture(size, draw) {
  const c = document.createElement("canvas");
  c.width = size;
  c.height = size;
  draw(c.getContext("2d"), size);
  return Texture.from(c);
}

const dotTex = canvasTexture(16, (ctx) => {
  ctx.fillStyle = "#fff";
  ctx.strokeStyle = "rgba(0,0,0,0.7)";
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.arc(8, 8, 6.5, 0, Math.PI * 2);
  ctx.fill();
  ctx.stroke();
});
const squareTex = canvasTexture(4, (ctx) => {
  ctx.fillStyle = "#fff";
  ctx.fillRect(0, 0, 4, 4);
});
const NEAR_SIZE = 0.8; // cells
const FAR_SIZE = 1.2; // cells

const PALETTE = [
  0xe8554e, 0xf2a541, 0x4fb286, 0x3c91e6, 0xa266dd, 0xf3e37c, 0x5bc0be,
  0xf28cb1,
];

// Cell colours (RGBA) by zone; walls, stairs and void override.
const ZONE_RGB = [
  [217, 212, 199], // landside
  [201, 214, 227], // airside
  [241, 212, 138], // security
  [168, 213, 162], // exit
];
const WALL_RGB = [61, 67, 76];
const STAIRS_RGB = [154, 127, 209];

// ---------------------------------------------------------------- world state

const world = new Container();
app.stage.addChild(world);
const floorLayer = new Container();
world.addChild(floorLayer);

const S = {
  w: 0,
  h: 0,
  flags: null,
  zone: null,
  floorImages: [],
  floorTextures: [],
  floorSprites: [],
  dirtyFloors: new Set(),
  floor: 0,
  cam: { x: 0, y: 0, zoom: 1 },
  transfer: cfg.transfer === "full" ? "full" : "view",
  format: 0,
  render: cfg.render === "sprites" ? "sprites" : "particles",
  gridVersion: 0,
  // decoded persons of the newest frame
  px: new Float32Array(0),
  py: new Float32Array(0),
  pfield: new Uint8Array(0),
  drawn: 0,
  persons: 0,
  tick: 0,
  lastCmd: 0,
};

let personLayers = null;

function paintCell(i) {
  const per = S.w * S.h;
  const f = Math.floor(i / per);
  const j = i % per;
  const flags = S.flags[i];
  const rgb =
    flags & 1
      ? WALL_RGB
      : flags & 2
        ? STAIRS_RGB
        : (ZONE_RGB[S.zone[i]] ?? WALL_RGB);
  const d = S.floorImages[f].data;
  d[j * 4] = rgb[0];
  d[j * 4 + 1] = rgb[1];
  d[j * 4 + 2] = rgb[2];
  d[j * 4 + 3] = 255;
  S.dirtyFloors.add(f);
}

function setupGrid(buf) {
  const dv = new DataView(buf);
  S.w = dv.getUint32(0, true);
  S.h = dv.getUint32(4, true);
  const floors = dv.getUint32(8, true);
  const cells = S.w * S.h * floors;
  S.flags = new Uint8Array(buf, 12, cells).slice();
  S.zone = new Uint8Array(buf, 12 + cells, cells).slice();
  floorLayer.removeChildren();
  S.floorImages = [];
  S.floorTextures = [];
  S.floorSprites = [];
  for (let f = 0; f < floors; f++) {
    const c = document.createElement("canvas");
    c.width = S.w;
    c.height = S.h;
    const ctx = c.getContext("2d");
    S.floorImages.push(ctx.createImageData(S.w, S.h));
    const tex = Texture.from(c);
    tex.source.scaleMode = "nearest";
    S.floorTextures.push({ canvas: c, ctx, tex });
    const sprite = new Sprite(tex);
    sprite.visible = f === S.floor;
    floorLayer.addChild(sprite);
    S.floorSprites.push(sprite);
  }
  for (let i = 0; i < cells; i++) paintCell(i);
  flushFloors();
  S.gridVersion = 0;
}

function flushFloors() {
  for (const f of S.dirtyFloors) {
    const t = S.floorTextures[f];
    t.ctx.putImageData(S.floorImages[f], 0, 0);
    t.tex.source.update();
  }
  S.dirtyFloors.clear();
}

function setFloor(f) {
  S.floor = f;
  S.floorSprites.forEach((s, i) => (s.visible = i === f));
}

// ---------------------------------------------------------------- persons

function makePersonLayers(max) {
  if (personLayers) {
    world.removeChild(personLayers.root);
    personLayers.root.destroy({ children: true });
  }
  const root = new Container();
  world.addChild(root);
  const layers = {
    root,
    max,
    render: S.render,
    lastField: new Int16Array(max).fill(-1),
  };
  if (S.render === "particles") {
    const mk = (texture, size) => {
      const pc = new ParticleContainer({
        texture,
        dynamicProperties: { position: true, color: true },
      });
      const pool = [];
      const scale = size / texture.width;
      for (let i = 0; i < max; i++) {
        pool.push(
          new Particle({
            texture,
            scaleX: scale,
            scaleY: scale,
            anchorX: 0.5,
            anchorY: 0.5,
          }),
        );
      }
      root.addChild(pc);
      return { pc, pool, shown: 0 };
    };
    layers.near = mk(dotTex, NEAR_SIZE);
    layers.far = mk(squareTex, FAR_SIZE);
  } else {
    const mk = (texture, size) => {
      const c = new Container();
      const pool = [];
      const scale = size / texture.width;
      for (let i = 0; i < max; i++) {
        const s = new Sprite(texture);
        s.anchor.set(0.5);
        s.scale.set(scale);
        s.visible = false;
        c.addChild(s);
        pool.push(s);
      }
      root.addChild(c);
      return { c, pool, shown: 0 };
    };
    layers.near = mk(dotTex, NEAR_SIZE);
    layers.far = mk(squareTex, FAR_SIZE);
  }
  personLayers = layers;
}

function isNear() {
  return S.cam.zoom >= NEAR_ZOOM;
}

/** Writes the decoded persons into the active detail level. */
function updatePersons() {
  const L = personLayers;
  const near = isNear();
  const active = near ? L.near : L.far;
  const inactive = near ? L.far : L.near;
  const n = Math.min(S.drawn, L.max);
  if (L.render === "particles") {
    inactive.pc.visible = false;
    active.pc.visible = true;
    for (let i = 0; i < n; i++) {
      const p = active.pool[i];
      p.x = S.px[i];
      p.y = S.py[i];
      const fld = S.pfield[i];
      if (L.lastField[i] !== fld || L.lastNear !== near) {
        p.tint = PALETTE[fld % PALETTE.length];
        L.lastField[i] = fld;
      }
    }
    if (active.shown !== n) {
      active.pc.particleChildren = active.pool.slice(0, n);
      active.pc.update();
      active.shown = n;
    }
  } else {
    inactive.c.visible = false;
    active.c.visible = true;
    for (let i = 0; i < n; i++) {
      const s = active.pool[i];
      s.x = S.px[i];
      s.y = S.py[i];
      const fld = S.pfield[i];
      if (L.lastField[i] !== fld || L.lastNear !== near) {
        s.tint = PALETTE[fld % PALETTE.length];
        L.lastField[i] = fld;
      }
      if (i >= active.shown) s.visible = true;
    }
    for (let i = n; i < active.shown; i++) active.pool[i].visible = false;
    active.shown = n;
  }
  L.lastNear = near;
}

// ---------------------------------------------------------------- camera

function applyCamera() {
  const { x, y, zoom } = S.cam;
  world.scale.set(zoom);
  world.position.set(
    window.innerWidth / 2 - x * zoom,
    window.innerHeight / 2 - y * zoom,
  );
}

function viewRect(margin = 4) {
  const hw = window.innerWidth / 2 / S.cam.zoom + margin;
  const hh = window.innerHeight / 2 / S.cam.zoom + margin;
  return [S.cam.x - hw, S.cam.y - hh, S.cam.x + hw, S.cam.y + hh];
}

function fitMap() {
  S.cam.x = S.w / 2;
  S.cam.y = S.h / 2;
  S.cam.zoom =
    Math.min(window.innerWidth / S.w, window.innerHeight / S.h) * 0.95;
}

function screenToCell(sx, sy) {
  return [
    Math.floor(S.cam.x + (sx - window.innerWidth / 2) / S.cam.zoom),
    Math.floor(S.cam.y + (sy - window.innerHeight / 2) / S.cam.zoom),
  ];
}

// ---------------------------------------------------------------- core link

let reqId = 0;
let pending = null; // request in flight
let latest = null; // newest frame not yet applied
let cmdId = 0;
const builds = new Map(); // id -> { t, raf }
let rafCount = 0;
let onReady = null;
let onStats = null;

// Panel state: table requests at `rate` per second (0 = none, 60 = every frame).
let panel = null;
const T = {
  rows: 5000,
  rate: 4,
  scroll: 0, // px per second, measurement series only
  pending: null,
  latest: null,
  lastReq: 0,
  reqId: 0,
  last: null, // { totalMs, modelMs, domRows } of the last update
};

port.onmessage = (e) => {
  const m = e.data;
  if (m.type === "frame") {
    const now = performance.now();
    rec.rtt.push(now - pending.t);
    rec.hostMs.push(m.hostMs);
    rec.bytes.push(m.buf.byteLength);
    pending = null;
    latest = m.buf;
  } else if (m.type === "table") {
    rec.tableRtt.push(performance.now() - T.pending.t);
    rec.tableHostMs.push(m.hostMs);
    rec.tableBytes.push(m.buf.byteLength);
    T.pending = null;
    T.latest = m.buf;
  } else if (m.type === "ready") {
    onReady?.(m);
  } else if (m.type === "stats") {
    onStats?.(m.stats);
  }
};

function request() {
  pending = { id: ++reqId, t: performance.now() };
  let floor = -1;
  let rect = [0, 0, 0, 0];
  if (S.transfer === "view") {
    floor = S.floor;
    rect = viewRect();
  }
  port.postMessage({
    type: "req",
    id: pending.id,
    format: S.format,
    floor,
    x0: rect[0],
    y0: rect[1],
    x1: rect[2],
    y1: rect[3],
    gridVersion: S.gridVersion,
  });
}

function requestTable(now) {
  T.pending = { id: ++T.reqId, t: now };
  T.lastReq = now;
  port.postMessage({ type: "table", id: T.pending.id, rows: T.rows });
}

function build(cx, cy, wall, floor = S.floor) {
  const id = ++cmdId;
  builds.set(id, { t: performance.now(), raf: rafCount, seen: false });
  port.postMessage({
    type: "build",
    id,
    x: cx,
    y: cy,
    floor,
    size: BRUSH,
    wall,
  });
}

function ensureCapacity(n) {
  if (S.px.length < n) {
    S.px = new Float32Array(n);
    S.py = new Float32Array(n);
    S.pfield = new Uint8Array(n);
  }
}

/** Decodes a frame: persons, grid changes, applied command id. */
function applyFrame(buf) {
  const dv = new DataView(buf);
  S.tick = dv.getFloat64(0, true);
  const count = dv.getUint32(8, true);
  S.persons = dv.getUint32(12, true);
  const includedVersion = dv.getUint32(16, true);
  S.lastCmd = dv.getUint32(20, true);
  const nChanges = dv.getUint32(24, true);
  rec.encodeUs.push(dv.getFloat32(32, true));
  rec.moved.push(dv.getFloat32(36, true));
  const format = dv.getUint32(40, true);
  ensureCapacity(count);
  let pos = HEADER;
  let n = 0;
  const filter = S.transfer === "full";
  if (format === 0) {
    const bytes = new Uint8Array(buf);
    for (let i = 0; i < count; i++, pos += 6) {
      if (filter && bytes[pos + 4] !== S.floor) continue;
      S.px[n] = (bytes[pos] | (bytes[pos + 1] << 8)) / SUBCELL;
      S.py[n] = (bytes[pos + 2] | (bytes[pos + 3] << 8)) / SUBCELL;
      S.pfield[n] = bytes[pos + 5];
      n++;
    }
  } else {
    for (let i = 0; i < count; i++, pos += 12) {
      if (filter && dv.getUint8(pos + 8) !== S.floor) continue;
      S.px[n] = dv.getFloat32(pos, true);
      S.py[n] = dv.getFloat32(pos + 4, true);
      S.pfield[n] = dv.getUint8(pos + 9);
      n++;
    }
  }
  S.drawn = n;
  pos = (pos + 3) & ~3;
  for (let i = 0; i < nChanges; i++, pos += 8) {
    const cell = dv.getUint32(pos, true);
    S.flags[cell] = dv.getUint32(pos + 4, true);
    paintCell(cell);
  }
  S.gridVersion = includedVersion;
  for (const [id, b] of builds) {
    if (id <= S.lastCmd && !b.seen) b.seen = true;
  }
}

// ---------------------------------------------------------------- recording

function newRec() {
  return {
    frameMs: [],
    rtt: [],
    hostMs: [],
    bytes: [],
    encodeUs: [],
    moved: [],
    applyMs: [],
    renderMs: [],
    drawn: [],
    states: 0,
    buildMs: [],
    buildFrames: [],
    tableRtt: [],
    tableHostMs: [],
    tableBytes: [],
    panelMs: [],
    modelMs: [],
    domRows: [],
    longTasks: [],
  };
}
let rec = newRec();

function q(v, p) {
  if (v.length === 0) return 0;
  const s = [...v].sort((a, b) => a - b);
  return s[Math.min(s.length - 1, Math.floor((s.length - 1) * p))];
}
const avg = (v) => (v.length ? v.reduce((a, b) => a + b, 0) / v.length : 0);
const r2 = (x) => Math.round(x * 100) / 100;

function summarize(r, seconds) {
  return {
    fps: r2(r.frameMs.length / seconds),
    frameMsP50: r2(q(r.frameMs, 0.5)),
    frameMsP99: r2(q(r.frameMs, 0.99)),
    framesOver20ms: r.frameMs.filter((x) => x > 20).length,
    framesOver33ms: r.frameMs.filter((x) => x > 33.4).length,
    statesPerSec: r2(r.states / seconds),
    rttMsP50: r2(q(r.rtt, 0.5)),
    rttMsP99: r2(q(r.rtt, 0.99)),
    hostMsMean: r2(avg(r.hostMs)),
    encodeUsMean: r2(avg(r.encodeUs)),
    bytesMean: Math.round(avg(r.bytes)),
    applyMsMean: r2(avg(r.applyMs)),
    renderMsMean: r2(avg(r.renderMs)),
    renderMsP99: r2(q(r.renderMs, 0.99)),
    drawnMean: Math.round(avg(r.drawn)),
    movedShareMean: r2(avg(r.moved)),
    tableUpdatesPerSec: r2(r.panelMs.length / seconds),
    tableRttMsP50: r2(q(r.tableRtt, 0.5)),
    tableRttMsP99: r2(q(r.tableRtt, 0.99)),
    tableHostMsMean: r2(avg(r.tableHostMs)),
    tableBytesMean: Math.round(avg(r.tableBytes)),
    panelMsMean: r2(avg(r.panelMs)),
    panelMsP99: r2(q(r.panelMs, 0.99)),
    panelMsMax: r2(Math.max(0, ...r.panelMs)),
    modelMsMean: r2(avg(r.modelMs)),
    domRowsMean: Math.round(avg(r.domRows)),
    longTasks: r.longTasks.length,
    longTaskMsMax: r2(Math.max(0, ...r.longTasks)),
  };
}

// ---------------------------------------------------------------- main loop

let lastRaf = 0;
let script = null; // camera script of the running phase: (tSeconds) => void

function frame(now) {
  rafCount++;
  if (lastRaf) rec.frameMs.push(now - lastRaf);
  lastRaf = now;
  if (script) script(now);
  applyCamera();
  if (latest !== null) {
    const t0 = performance.now();
    applyFrame(latest);
    flushFloors();
    updatePersons();
    rec.applyMs.push(performance.now() - t0);
    rec.drawn.push(S.drawn);
    rec.states++;
    latest = null;
  }
  if (pending === null && S.w > 0) request();
  // Panel: newest table first, then the next request according to the rate.
  if (panel?.visible) {
    if (T.latest !== null) {
      T.last = panel.setData(T.latest);
      rec.panelMs.push(T.last.totalMs);
      rec.modelMs.push(T.last.modelMs);
      rec.domRows.push(T.last.domRows);
      T.latest = null;
    }
    if (
      T.rate > 0 &&
      T.pending === null &&
      S.w > 0 &&
      now - T.lastReq >= 1000 / T.rate - 2
    ) {
      requestTable(now);
    }
    if (T.scroll > 0 && rec.frameMs.length > 0) {
      panel.scrollBy((T.scroll * rec.frameMs[rec.frameMs.length - 1]) / 1000);
    }
  }
  const t1 = performance.now();
  if (cfg.draw) app.render();
  rec.renderMs.push(performance.now() - t1);
  // Builds whose frame was drawn just now.
  for (const [id, b] of builds) {
    if (b.seen) {
      rec.buildMs.push(performance.now() - b.t);
      rec.buildFrames.push(rafCount - b.raf);
      lastBuild = { ms: performance.now() - b.t, frames: rafCount - b.raf };
      builds.delete(id);
    }
  }
  requestAnimationFrame(frame);
}

let lastBuild = null;
let phaseName = "interaktiv";

setInterval(() => {
  const recent = rec.frameMs.slice(-120);
  const fps = recent.length ? 1000 / avg(recent) : 0;
  hudEl.textContent = [
    `Phase      ${phaseName}`,
    `Bilder/s   ${fps.toFixed(1)}  (p99 ${q(recent, 0.99).toFixed(1)} ms)`,
    `Personen   ${S.drawn} gezeichnet / ${S.persons}`,
    `Übergabe   ${S.transfer}, ${S.format ? "f32" : "kompakt"}, ${(avg(rec.bytes.slice(-30)) / 1024).toFixed(1)} KiB, ${avg(rec.rtt.slice(-30)).toFixed(2)} ms`,
    `Zeichnung  ${S.render}, Detail ${isNear() ? "nah" : "fern"}, Zoom ${S.cam.zoom.toFixed(2)}`,
    `Etage      ${S.floor}   Tick ${S.tick}`,
    `Bauaktion  ${lastBuild ? `${lastBuild.ms.toFixed(1)} ms, ${lastBuild.frames} Bilder` : "–"}`,
    `Panel      ${panel?.visible ? `${T.rows} Zeilen, ${T.rate}/s, ${panel.virtual ? "virtualisiert" : "alle im DOM"}, Update ${T.last ? T.last.totalMs.toFixed(1) : "–"} ms` : "aus"}`,
    `GPU        ${env.gpu}`,
  ].join("\n");
}, 250);

// ---------------------------------------------------------------- input

let drag = null;
app.canvas.addEventListener("contextmenu", (e) => e.preventDefault());
app.canvas.addEventListener("pointerdown", (e) => {
  if (e.button === 2 || e.button === 1) {
    drag = { x: e.clientX, y: e.clientY, cx: S.cam.x, cy: S.cam.y };
  } else if (e.button === 0 && !script) {
    const [cx, cy] = screenToCell(e.clientX, e.clientY);
    build(cx - 1, cy - 1, !e.shiftKey);
  }
});
window.addEventListener("pointermove", (e) => {
  if (!drag) return;
  S.cam.x = drag.cx - (e.clientX - drag.x) / S.cam.zoom;
  S.cam.y = drag.cy - (e.clientY - drag.y) / S.cam.zoom;
});
window.addEventListener("pointerup", () => (drag = null));
app.canvas.addEventListener(
  "wheel",
  (e) => {
    if (script) return;
    e.preventDefault();
    const [bx, by] = [
      S.cam.x + (e.clientX - window.innerWidth / 2) / S.cam.zoom,
      S.cam.y + (e.clientY - window.innerHeight / 2) / S.cam.zoom,
    ];
    S.cam.zoom = Math.min(
      64,
      Math.max(0.5, S.cam.zoom * (e.deltaY > 0 ? 1 / 1.15 : 1.15)),
    );
    S.cam.x = bx - (e.clientX - window.innerWidth / 2) / S.cam.zoom;
    S.cam.y = by - (e.clientY - window.innerHeight / 2) / S.cam.zoom;
  },
  { passive: false },
);
window.addEventListener("keydown", (e) => {
  if (script) return;
  if (e.key >= "1" && e.key <= "5" && Number(e.key) <= S.floorSprites.length)
    setFloor(Number(e.key) - 1);
  else if (e.key === "t") S.transfer = S.transfer === "view" ? "full" : "view";
  else if (e.key === "f") S.format = 1 - S.format;
  else if (e.key === "r") {
    S.render = S.render === "particles" ? "sprites" : "particles";
    makePersonLayers(personLayers.max);
  } else if (e.key === "0") fitMap();
  else if (e.key === "p") panel.setVisible(!panel.visible);
  else if (e.key === "v") panel.setVirtual(!panel.virtual);
  else if (e.key === "n") {
    const sizes = [2000, 5000, 20000];
    T.rows = sizes[(sizes.indexOf(T.rows) + 1) % sizes.length];
  } else if (e.key === "+" || e.key === "-") {
    const rates = [0, 1, 4, 10, 30, 60];
    const k = rates.indexOf(T.rate) + (e.key === "+" ? 1 : -1);
    T.rate = rates[Math.max(0, Math.min(rates.length - 1, k))];
  }
});

// Long tasks (> 50 ms) on the renderer's main thread, for the series.
try {
  new PerformanceObserver((list) => {
    for (const e of list.getEntries()) rec.longTasks.push(e.duration);
  }).observe({ type: "longtask", buffered: false });
} catch {
  window.spike.log("longtask observer not available");
}

// ---------------------------------------------------------------- series

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

function init(persons) {
  return new Promise((resolve) => {
    S.w = 0; // no requests until the new simulation is ready
    onReady = (m) => {
      onReady = null;
      pending = null;
      latest = null;
      setupGrid(m.grid);
      makePersonLayers(persons);
      if (!panel) panel = createPanel(document.getElementById("panel"), m.dict);
      resolve(m.initMs);
    };
    port.postMessage({
      type: "init",
      persons,
      threads: cfg.threads,
      hz: cfg.hz,
      size: cfg.size,
      floors: cfg.floors,
      gates: cfg.gates,
      incremental: cfg.incremental,
      bgThreads: cfg.bgThreads,
    });
  });
}

function simStats() {
  return new Promise((resolve) => {
    onStats = (s) => {
      onStats = null;
      resolve(s);
    };
    port.postMessage({ type: "stats" });
  });
}

/** Ground floor (landside hall): floor 2 of five, floor 0 of two. */
const ground = () => (S.floorSprites.length === 5 ? 2 : 0);

const PHASES = {
  overview: () => {
    setFloor(ground());
    fitMap();
    return null;
  },
  // Camera circles around the densest spot, as in spike 1.3 ("pan"): the
  // playfield moves every frame while the panel updates.
  pan: () => {
    setFloor(ground());
    S.cam.zoom = 4;
    const t0 = performance.now();
    const r = S.w * 0.15;
    return (now) => {
      const a = ((now - t0) / 1000) * 0.6;
      S.cam.x = hotspot[0] + Math.cos(a) * r;
      S.cam.y = hotspot[1] + Math.sin(a) * r;
    };
  },
};

/** Centre of the densest 16 × 16 block among the persons drawn right now. */
let hotspot = [0, 0];
function findHotspot() {
  const B = 16;
  const bw = Math.ceil(S.w / B);
  const counts = new Uint32Array(bw * Math.ceil(S.h / B));
  for (let i = 0; i < S.drawn; i++) {
    counts[Math.floor(S.py[i] / B) * bw + Math.floor(S.px[i] / B)]++;
  }
  let best = 0;
  for (let i = 1; i < counts.length; i++)
    if (counts[i] > counts[best]) best = i;
  hotspot = [(best % bw) * B + B / 2, Math.floor(best / bw) * B + B / 2];
}

async function measure(name, seconds) {
  phaseName = name;
  script = PHASES[name]();
  await sleep(500); // warm-up, not recorded
  rec = newRec();
  const t0 = performance.now();
  await sleep(seconds * 1000);
  const result = { name, ...summarize(rec, (performance.now() - t0) / 1000) };
  if (name === "overview") findHotspot();
  script = null;
  return result;
}

/** Waits until the panel shows a table with the current row count. */
async function loadTable() {
  T.latest = null;
  while (T.pending !== null) await sleep(5);
  requestTable(performance.now());
  while (T.pending !== null || T.latest !== null) await sleep(5);
}

/**
 * One panel configuration while the camera pans: rows, update rate,
 * sorting, virtualization; the panel scrolls continuously.
 */
async function measurePanel(label, { visible, rows, rate, sort, virtual }) {
  panel.setVisible(visible);
  T.rate = 0;
  T.scroll = 0;
  if (visible) {
    T.rows = rows;
    panel.setVirtual(virtual);
    panel.setSorting(sort ? [{ id: "onBoard", desc: true }] : []);
    await loadTable();
    panel.scrollTo(0);
    T.rate = rate;
    T.scroll = SCROLL_PX_PER_S;
  }
  const r = await measure("pan", cfg.phaseSeconds);
  T.rate = 0;
  T.scroll = 0;
  return { label, visible, rows, rate, sort, virtual, ...r };
}

const SCROLL_PX_PER_S = 600;

function seriesPlan() {
  const pick = (v, all) => (v ? [Number(v)] : all);
  const rows = pick(cfg.rows, [2000, 5000, 20000]);
  const rates = pick(cfg.rate, [1, 4, 10, 30, 60]);
  const plan = [{ label: "ohne Panel", visible: false }];
  plan.push({
    label: "Panel ohne Updates",
    visible: true,
    rows: rows.includes(5000) ? 5000 : rows[0],
    rate: 0,
    sort: true,
    virtual: true,
  });
  for (const n of rows) {
    for (const rate of rates) {
      plan.push({
        label: "sortiert",
        visible: true,
        rows: n,
        rate,
        sort: true,
        virtual: true,
      });
    }
  }
  const big = rows[rows.length - 1];
  const mid = rates.includes(10) ? 10 : rates[0];
  plan.push({
    label: "unsortiert",
    visible: true,
    rows: big,
    rate: mid,
    sort: false,
    virtual: true,
  });
  if (rows[0] <= 2000) {
    for (const rate of rates.filter((x) => x <= 4)) {
      plan.push({
        label: "alle Zeilen im DOM",
        visible: true,
        rows: rows[0],
        rate,
        sort: true,
        virtual: false,
      });
    }
  }
  plan.push({ label: "ohne Panel (Ende)", visible: false });
  return plan;
}

async function runSeries() {
  const persons = Number(cfg.persons) || 10000;
  statusEl.textContent = `Messung läuft: ${persons} Personen werden angelegt …`;
  const initMs = await init(persons);
  panel.setVisible(false);
  // Fast-forward so that persons spread over the airport like in a running game.
  statusEl.textContent = `Messung läuft: ${persons} Personen, Zeitraffer …`;
  port.postMessage({ type: "hz", hz: 0 });
  await sleep(3000);
  port.postMessage({ type: "hz", hz: cfg.hz });
  await measure("overview", 1); // finds the densest spot for the camera
  await simStats(); // reset
  const plan = seriesPlan();
  const phases = [];
  for (const [k, p] of plan.entries()) {
    statusEl.textContent = `Messung läuft (${k + 1}/${plan.length}): ${p.label}${p.visible ? `, ${p.rows} Zeilen, ${p.rate}/s` : ""}. Bitte nicht bedienen.`;
    const r = await measurePanel(p.label, p);
    phases.push(r);
    window.spike.log(JSON.stringify(r));
  }
  return [
    {
      persons,
      transfer: S.transfer,
      render: S.render,
      initMs: Math.round(initMs),
      hotspot,
      scrollPxPerS: SCROLL_PX_PER_S,
      phases,
      sim: await simStats(),
    },
  ];
}

// ------------------------------------------------------- long run (1.5)

/**
 * Builds and removes a 3 × 3 wall in the landside hall of the ground floor
 * every `BUILD_EVERY_S` seconds (alternating), so that the core recomputes
 * all direction fields in the background during the run (ADR-010).
 */
const BUILD_EVERY_S = 10;
let buildSpot = null;
let buildRng = 12345;
function nextBuild() {
  if (buildSpot) {
    build(buildSpot[0], buildSpot[1], false, ground());
    buildSpot = null;
  } else {
    buildRng = (buildRng * 1103515245 + 12345) >>> 0;
    const x = Math.floor(S.w / 8 + (buildRng % Math.floor((S.w * 3) / 4)));
    const y = Math.floor(
      S.h / 2 + 15 + ((buildRng >>> 12) % Math.floor(S.h / 2 - 90)),
    );
    buildSpot = [x, y];
    build(x, y, true, ground());
  }
}

/**
 * Long run: camera alternates every window between the whole ground floor
 * and circling the densest spot on the departures level; the panel shows a
 * sorted table and updates 4 times per second; walls are built and removed
 * regularly. One record per window: frames, simulation ticks, backlog,
 * recomputation, memory.
 */
async function runLongrun() {
  const persons = Number(cfg.persons) || 10000;
  statusEl.textContent = `Messung läuft: ${persons} Personen, ${cfg.floors} Etagen, Raster ${cfg.size} × ${cfg.size} werden angelegt …`;
  const initMs = await init(persons);
  panel.setVisible(false);
  statusEl.textContent = `Messung läuft: Zeitraffer, damit sich die Personen verteilen …`;
  port.postMessage({ type: "hz", hz: 0 });
  await sleep(5000);
  // Highest possible tick rate while drawing (no throttle), 5 s.
  await simStats();
  const p0 = performance.now();
  await measure("overview", 5);
  const probe = await simStats();
  const maxTicksPerSec = r2(probe.ticks / ((performance.now() - p0) / 1000));
  port.postMessage({ type: "hz", hz: cfg.hz });
  findHotspot();
  // Panel as in normal play: sorted table, 4 updates per second.
  panel.setVisible(true);
  T.rows = Number(cfg.rows) || 5000;
  panel.setVirtual(true);
  panel.setSorting([{ id: "onBoard", desc: true }]);
  await loadTable();
  T.rate = cfg.rate === "" ? 4 : Number(cfg.rate);
  await simStats(); // reset
  const windowS = 30;
  const windows = [];
  const started = performance.now();
  const end = started + cfg.minutes * 60 * 1000;
  let k = 0;
  let nextBuildAt = performance.now() + 2000;
  const buildTimer = setInterval(() => {
    if (performance.now() >= nextBuildAt) {
      nextBuildAt += BUILD_EVERY_S * 1000;
      nextBuild();
    }
  }, 250);
  while (performance.now() < end) {
    const name = k % 2 === 0 ? "overview" : "pan";
    phaseName = `Langzeitlauf ${name}`;
    const left = Math.max(0, Math.round((end - performance.now()) / 60000));
    statusEl.textContent = `Messung läuft: Langzeitlauf, noch etwa ${left} Minuten. Bitte nicht bedienen, Fenster nicht verdecken.`;
    script = PHASES[name]();
    if (name === "pan")
      setFloor(ground() + (S.floorSprites.length === 5 ? 1 : 0));
    rec = newRec();
    const t0 = performance.now();
    await sleep(windowS * 1000);
    const seconds = (performance.now() - t0) / 1000;
    const sim = await simStats();
    const mem = await window.spike.memory();
    const r = {
      window: k,
      atS: Math.round((performance.now() - started) / 1000),
      name,
      ...summarize(rec, seconds),
      buildMsP50: r2(q(rec.buildMs, 0.5)),
      buildMsMax: r2(Math.max(0, ...rec.buildMs)),
      buildFramesMax: Math.max(0, ...rec.buildFrames),
      ticksPerSec: r2(sim.ticks / seconds),
      stepUsMean: r2(sim.stepUsMean),
      stepUsP99: r2(sim.stepUsP99),
      behindTicks: sim.behindTicks,
      builds: sim.builds,
      recomputes: sim.recomputes,
      recomputeMsMean: r2(sim.recomputeMsMean),
      recomputeMsMax: r2(sim.recomputeMsMax),
      recomputeTicksMax: sim.recomputeTicksMax,
      memTotalMb: mem?.totalMb ?? null,
      memSimCoreMb: mem?.["sim-core"] ?? null,
    };
    windows.push(r);
    window.spike.log(JSON.stringify(r));
    script = null;
    k++;
  }
  clearInterval(buildTimer);
  if (buildSpot) nextBuild();
  T.rate = 0;
  return [
    {
      persons,
      size: cfg.size,
      floors: cfg.floors,
      transfer: S.transfer,
      render: S.render,
      initMs: Math.round(initMs),
      maxTicksPerSec,
      maxTicksProbe: probe,
      windowS,
      buildEveryS: BUILD_EVERY_S,
      panelRows: T.rows,
      windows,
    },
  ];
}

// ---------------------------------------------------------------- start

requestAnimationFrame(frame);
if (cfg.series === "none") {
  await init(Number(cfg.persons) || 10000);
  setFloor(ground());
  fitMap();
  T.rows = Number(cfg.rows) || 5000;
  T.rate = cfg.rate === "" ? 4 : Number(cfg.rate);
  await loadTable();
  statusEl.textContent =
    "Interaktiv. Spielfeld: rechte Maustaste ziehen = verschieben, Mausrad = zoomen, Linksklick = Wand bauen (Umschalt = entfernen), 1–5 = Etage, 0 = ganze Karte. Panel: Spaltenkopf = sortieren, p = ein/aus, +/- = Aktualisierungsrate, n = Zeilenzahl, v = Virtualisierung.";
} else {
  const started = performance.now();
  const runs =
    cfg.series === "longrun" ? await runLongrun() : await runSeries();
  const file = await window.spike.report({
    ...env,
    durationS: Math.round((performance.now() - started) / 1000),
    runs,
  });
  fitMap();
  phaseName = "interaktiv";
  panel.setVisible(true);
  T.rows = 5000;
  T.rate = 4;
  statusEl.textContent =
    (file
      ? `Messung fertig. Ergebnis gespeichert unter:\n${file}\nBitte diese Datei schicken.\n\n`
      : "Messung fertig.\n\n") +
    "Jetzt interaktiv: Spaltenkopf = sortieren, p = Panel ein/aus, +/- = Aktualisierungsrate, n = Zeilenzahl, v = Virtualisierung; Spielfeld wie in Spike 1.3.";
}
